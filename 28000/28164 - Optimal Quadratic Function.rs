use io::Write;
use std::{io, str};

/// Value at which the exponent overflows (15 bits, but signed)
const TWO_TO_THE_14: i16 = 16384;
/// Value at which the fraction overflows
const TWO_TO_THE_112: u128 = 5192296858534827628530496329220096;

/// Struct containing a single unsiged 128-bits integer. We use this instead of two u64 for
/// simplicity and to leverage LLVM's fast operations on large numbers. In this u128 we use
/// the first bit to store the sign. The next 15 bits store the exponent. Then finally, the
/// folling 112 bits store the fraction. The value represented is then `sign * 1.fraction *
/// 2^exponent`. This allows us to store numbers between `1.0 * 2^(-2^14)` and `1.99.. *
/// (2^(2^14-1)-1)`, both negative and positive, all with 112 bit precision.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct f128(u128);

/// This module contains several important numbers and constants. Some of them come from
/// mathematics, others arise from the requirement to invalidate some results, for example
/// `f128::from(1) / f128::from(0) == consts::NAN`
pub mod consts {
    use super::f128;

    /// Zero
    pub const ZERO: f128 = f128(0x8000_0000_0000_0000_0000_0000_0000_0000);
    /// One
    pub const ONE: f128 = f128(0xC000_0000_0000_0000_0000_0000_0000_0000);
    /// Negative zero, only semantically different from zero
    pub const NEG_ZERO: f128 = f128(0x0000_0000_0000_0000_0000_0000_0000_0000);
    /// Not A Number. Arisis from invalid operations such as `INF + NEG_INF`
    pub const NAN: f128 = f128(0xFFFF_0000_0000_0000_0000_0000_0000_0000);
    /// Infinity
    pub const INF: f128 = f128(0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF);
    /// Negative Infinity
    pub const NEG_INF: f128 = f128(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF);
    /// `PI / f128::from(2)`
    pub const HALF_PI: f128 = f128(0xC000_921F_B544_42D1_8469_898C_C517_01B8);
    /// The circumference of a circle divided by its diameter
    pub const PI: f128 = f128(0xC001_921F_B544_42D1_8469_898C_C517_01B8);
    /// `PI * f128::from(2)`
    pub const TWO_PI: f128 = f128(0xC002_921F_B544_42D1_8469_898C_C517_01B8);
    /// Limit as n goes to infinity of `(1 + 1/n)^n`
    pub const EULER: f128 = f128(0xC001_5BF0_A8B1_4576_9535_5FB8_AC40_4E7A);
    /// Used to tolerate some error when comparing values
    pub const EPSILON: f128 = f128(0x0FFF_0000_0000_0000_0000_0000_0000_0000);
}

use consts::*;

impl f128 {
    /// Creates a new f128 from three pieces. The first boolean is true if the number is
    /// positive. Note that the exponent is a signed number and that its values must range
    /// from -2^14 to 2^14 - 1. The fraction must be smaller that 2^112
    fn new(sign: bool, exp: i16, frac: u128) -> f128 {
        debug_assert!(exp <= TWO_TO_THE_14 - 1);
        debug_assert!(exp >= -TWO_TO_THE_14);
        debug_assert!(frac < TWO_TO_THE_112);
        let sign = if sign { 1 } else { 0 } << 127;
        let exp = ((exp + TWO_TO_THE_14) as u128) << 112;
        f128(sign | exp | frac)
    }

    /// Returns true if the number is positive, and false if the number is negative
    fn sign(self) -> bool {
        if self.0 & 0x8000_0000_0000_0000_0000_0000_0000_0000 == 0 {
            false
        } else {
            true
        }
    }

    /// Returns the number modified to have the provided sign;
    fn with_sign(self, sign: bool) -> f128 {
        if sign {
            f128(self.0 | 0x8000_0000_0000_0000_0000_0000_0000_0000)
        } else {
            f128(self.0 & 0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF)
        }
    }

    /// Returns the exponent of the number
    fn exp(self) -> i16 {
        (self.0 >> 112 & 0x7FFF) as i16 - TWO_TO_THE_14
    }

    /// Returns the number with the provided exponent
    fn with_exp(self, exp: i16) -> f128 {
        let exp = ((exp + TWO_TO_THE_14) as u128) << 112;
        f128(self.0 & 0x8000_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF | exp)
    }

    /// Returns the fraction of then number
    fn frac(self) -> u128 {
        self.0 & 0x0000_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF
    }

    /*fn with_frac(self, frac: u128) -> f128 {
        debug_assert!(frac & 0xFFFF_0000_0000_0000_0000_0000_0000_0000 == 0);
        f128(self.0 & 0xFFFF_0000_0000_0000_0000_0000_0000_0000 | frac)
    }*/

    /// Computes the absolute value of the number. This transforms negative numbers into their
    /// positive counterparts, and leaves positive numbers unaffected
    /// ```
    /// let num1 = f128::from(-3);
    /// let num2 = f128::from(3);
    /// assert_eq!(num1.abs(), num2);
    /// ```
    pub fn abs(self) -> f128 {
        f128(self.0 | 0x8000_0000_0000_0000_0000_0000_0000_0000)
    }

    /// Computes the nearest integer rounded down.
    /// ```
    /// let num1 = f128::from(3.6);
    /// let num2 = f128::from(-2.7);
    /// assert_eq!(num1.floor(), f128::from(3));
    /// assert_eq!(num2.floor(), f128::from(-2));
    /// ```
    pub fn floor(self) -> f128 {
        // at this level the truncation error becomes so large that a substraction of 1 isn't
        // possible anymore, making floor dangerous.
        debug_assert!(self.abs() < f128::from(9007199254741000usize));
        if self.abs() < ONE {
            return ZERO;
        }
        let exp = self.exp();
        // bitwise destruction of the last `exp` bits
        f128(self.0 >> 112 - exp << 112 - exp)
    }

    /// Lookup table for internal use. Returns the precomputed factorial number at index n in
    /// fixed point representation.
    fn inv_fact(n: usize) -> u128 {
        match n {
            0 => 5192296858534827628530496329220097,
            1 => 5192296858534827628530496329220097,
            2 => 2596148429267413814265248164610048,
            3 => 865382809755804604755082721536682,
            4 => 216345702438951151188770680384170,
            5 => 43269140487790230237754136076834,
            6 => 7211523414631705039625689346139,
            7 => 1030217630661672148517955620877,
            8 => 128777203832709018564744452609,
            9 => 14308578203634335396082716956,
            10 => 1430857820363433539608271695,
            11 => 130077983669403049055297426,
            12 => 10839831972450254087941452,
            13 => 833833228650019545226265,
            14 => 59559516332144253230447,
            15 => 3970634422142950215363,
            16 => 248164651383934388460,
            17 => 14597920669643199321,
            18 => 810995592757955517,
            19 => 42683978566208185,
            20 => 2134198928310409,
            21 => 101628520395733,
            22 => 4619478199806,
            23 => 200846878252,
            24 => 8368619927,
            25 => 334744797,
            26 => 12874799,
            27 => 476844,
            28 => 17030,
            _ => unimplemented!(),
        }
    }

    /// Computes the sine of the current number using a Taylor Series. Uses fixed point
    /// representation internally for performance.
    /// ```
    /// assert_eq!((PI / f128::from(2)).sin(), f128::from(1))
    /// assert_eq!(PI.sin(), f128::from(0))
    /// assert_eq!((PI * f128::from(1.5)).sin(), f128::from(-1))
    /// assert_eq!((PI * f128::from(2)).sin(), f128::from(0))
    /// ```
    pub fn sin(self) -> f128 {
        // we're computing a sine, so we can subtract 2pi as many times as we want without
        // altering the result. We normalize to [0, 2pi] to make fixed point computation
        // possible
        let num = self % TWO_PI;
        // since we have that sin(x) = -sin(x - pi), we can transform to [0, pi] and make the
        // result negative if the original number was in [pi, 2pi]. Doing so ensures that the
        // number we are working with is always positive. This is necessary because the fixed
        // point numbers cannot annotate negativity (they don't need it in their lives)
        let (num, sign) = if num < PI {
            (num, true)
        } else {
            ((num - PI), false)
        };
        // if our number is larger than 1/2 pi we get an overflow later, so what we can do is
        // reduce the domain one last time to prevent that from happening. We do this by
        // noting that for all x in [1/2pi, pi] we have that sin(x) = sin(pi - x), where
        // obviously pi - x must lie in [0, 1/2pi].
        let num = if num > HALF_PI { PI - num } else { num };
        // covert our floating point number to a 112 bit fixed point number
        let num = num.to_fixed_point();
        // precompute the square for performance
        let square = fixp_mul(num, num);
        // this constant denots the number of taylor terms we'll use
        const LENGTH: usize = 24;
        // we efficiently compute the powers and store them for later use
        let mut powers = [0u128; LENGTH / 2];
        (3..LENGTH).step_by(2).fold(num, |previous, i| {
            debug_assert!(previous.checked_mul((square >> 112) + 1).is_some());
            // this overflows for large i and num in [1/2pi , pi]
            let power = fixp_mul(previous, square);
            powers[i / 2] = power;
            power
        });
        // first add every positive term of the taylor series. Computing the positive and
        // negative terms seperately prevents some minor overhead cause by keeping track of
        // whether the current term is positive or negative at runtime, but more importantly,
        // it ensures that none of the intermediate results can be negative
        let frac = (5..LENGTH).step_by(4).fold(num, |result, i| {
            result + fixp_mul(powers[i / 2], f128::inv_fact(i))
        });
        // now do the negative terms
        let frac = (3..LENGTH).step_by(4).fold(frac, |result, i| {
            result - fixp_mul(powers[i / 2], f128::inv_fact(i))
        });
        if frac == 0 {
            return ZERO;
        }
        // shift the number such that the 113'th bit is one, then remove it and use the result
        // as mantissa for our floating point
        let overflow = 15 - frac.leading_zeros() as i16;
        let frac = (frac << -overflow) - TWO_TO_THE_112;
        f128::new(sign, overflow, frac)
    }

    /// Computes the cosine of the number. Does this by subtracting 1/2pi and then calling
    /// the `sin()` method.
    /// ```
    /// assert_eq!((PI / f128::from(2)).cos(), f128::from(0))
    /// assert_eq!(PI.cos(), f128::from(-1))
    /// assert_eq!((PI * f128::from(1.5)).cos(), f128::from(0))
    /// assert_eq!((PI * f128::from(2)).cos(), f128::from(1))
    /// ```
    pub fn cos(self) -> f128 {
        (self - HALF_PI).sin()
    }

    /// Computes the square root of the number by first computing the inverse square root
    /// and then multiplying it by the number itself
    /// ```
    /// // Note that machine error make assertions hard
    /// assert_eq!(f128::from(4).sqrt() - f128::from(2) < EPSILON);
    /// assert_eq!(f128::from(2).sqrt(), f128::from(1.41421356237) < EPSILON);
    /// #[should_panic] f128::from(-2).sqrt();
    /// ```
    pub fn sqrt(self) -> f128 {
        assert!(self >= ZERO);
        // constants needed for computation
        let three = f128::from(3);
        let half = f128::from(0.5);
        // our initial guess of the number is the number itself, but with the exponent divided
        // by -2
        let init = self.with_exp(self.exp() / -2);
        // this newtons method converges to the solution of self - x^2 = 0, which is either
        // sqrt(self) or -sqrt(self). Therefore we force the sign to be positive in the end.
        (0..5)
            .fold(init, |result, _| {
                result * (three - self * result * result) * half
            })
            .with_sign(true)
            * self
    }

    /// converts a floating point number to a fixed point number. For internal use. Performs
    /// no checks of any kind for speed. Will crash if used with numbers that are too large or
    /// small or NAN or INF or whatevs.
    fn to_fixed_point(self) -> u128 {
        if self == ZERO {
            return 0;
        }
        let exp = self.exp();
        debug_assert!(exp < 11);
        if exp < 0 {
            (self.frac() + TWO_TO_THE_112) >> -exp
        } else {
            (self.frac() + TWO_TO_THE_112) << exp
        }
    }

    /// Converts the current number to a `u128`
    /// ```
    /// assert_eq!(f128::from(5.3).to_u128(), 5u128);
    /// #[should_panic] f128::from(-2.3).to_u128();
    /// ```
    pub fn to_u128(self) -> u128 {
        assert!(self >= ZERO);
        assert!(self != NAN);
        if self == ZERO {
            return 0;
        }
        let exp = self.exp();
        let factor = if exp >= 0 { 2u128.pow(exp as u32) } else { 0 };
        (TWO_TO_THE_112 + self.frac()) * factor / TWO_TO_THE_112
    }

    // pub fn log(self) -> f128 {
    //     assert!(self > ZERO);

    //     log(self.frac()) + LOG_TWO * f128::from(self.exp());
    // }
}

impl From<f64> for f128 {
    fn from(number: f64) -> f128 {
        // doesnt work for [0,1] :(
        if number == 0.0 {
            return ZERO;
        }
        let sign = number > 0.0;
        let number = number.abs();
        let exp = number.log(2.0) as i16;
        let frac = (number as f64 / 2f64.powf(exp as f64) * TWO_TO_THE_112 as f64) as u128;
        f128::new(sign, exp, frac - TWO_TO_THE_112)
    }
}

impl From<i32> for f128 {
    fn from(number: i32) -> f128 {
        if number == 0 {
            return ZERO;
        }
        let sign = number > 0;
        let number = number.abs();
        let exp = 31 - number.leading_zeros() as i16;
        let frac = number as u128 * TWO_TO_THE_112 / 2i32.pow(exp as u32) as u128 - TWO_TO_THE_112;
        f128::new(sign, exp, frac)
    }
}

impl From<usize> for f128 {
    fn from(number: usize) -> f128 {
        if number == 0 {
            return ZERO;
        }
        let exp = 63 - number.leading_zeros() as i16;
        let frac =
            TWO_TO_THE_112 / 2usize.pow(exp as u32) as u128 * number as u128 - TWO_TO_THE_112;
        f128::new(true, exp, frac)
    }
}

impl std::fmt::Display for f128 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if !self.sign() {
            write!(f, "-")?
        }
        write!(
            f,
            "{}",
            (TWO_TO_THE_112 + self.frac()) as f64 / TWO_TO_THE_112 as f64
                * 2f64.powf(self.exp() as f64)
        )
    }
}

impl std::fmt::Debug for f128 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if self.sign() {
            write!(f, "")
        } else {
            write!(f, "-")
        }?;
        write!(
            f,
            "{}*2^{}",
            (TWO_TO_THE_112 + self.frac()) as f64 / TWO_TO_THE_112 as f64,
            self.exp()
        )
    }
}

/// Multiplies two numbers, treating them as 112 bit fixed point numbers with only zeroes
/// before the comma
fn floating_point_mul_112(a: u128, b: u128) -> u128 {
    debug_assert!(a >> 112 == 0);
    debug_assert!(b >> 112 == 0);
    let hi = |num| num >> 56;
    let lo = |num| num & 0x0000_0000_0000_0000_00FF_FFFF_FFFF_FFFF;
    hi(a) * hi(b) + hi(hi(a) * lo(b) + lo(a) * hi(b))
}

/// Performs full fixed point multiplications with the comma between the 112th and the 113th
/// bit
fn fixp_mul(a: u128, b: u128) -> u128 {
    let (a_int, a_float): (u128, u128) = (a >> 112, a & 0x0000_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF);
    let (b_int, b_float): (u128, u128) = (b >> 112, b & 0x0000_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF);
    (a_int * b_int << 112)
        + a_int * b_float
        + b_int * a_float
        + floating_point_mul_112(a_float, b_float)
}

impl std::cmp::PartialEq for f128 {
    fn eq(&self, other: &f128) -> bool {
        if self.0 == NAN.0 || other.0 == NAN.0 {
            false
        } else {
            self.0 == other.0
        }
    }
}

/// Full comparison of self and other, pretty slow, takes 4 ns
impl std::cmp::PartialOrd for f128 {
    fn partial_cmp(&self, other: &f128) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        if self.0 == NAN.0 || other.0 == NAN.0 {
            return None;
        } else if self.0 == other.0 {
            return Some(Ordering::Equal);
        }
        let (self_sign, other_sign) = (self.sign(), other.sign());
        if self_sign && !other_sign {
            return Some(Ordering::Greater);
        } else if !self_sign && other_sign {
            return Some(Ordering::Less);
        }
        // We have to return the opposite result if the numbers are both negative, the fastest
        // way to do this is by flipping the numbers in the negative cast
        let (num1, num2) = if self.0 & 0x8000_0000_0000_0000_0000_0000_0000_0000 == 0 {
            (other, self)
        } else {
            (self, other)
        };
        let (self_exp, other_exp) = (num1.exp(), num2.exp());
        if self_exp > other_exp {
            return Some(Ordering::Greater);
        } else if self_exp < other_exp {
            return Some(Ordering::Less);
        }
        let (self_frac, other_frac) = (num1.frac(), num2.frac());
        if self_frac > other_frac {
            return Some(Ordering::Greater);
        } else if self_frac < other_frac {
            return Some(Ordering::Less);
        }
        unreachable!()
    }
}

impl std::ops::Neg for f128 {
    type Output = f128;

    fn neg(self) -> f128 {
        f128(self.0 ^ 0x8000_0000_0000_0000_0000_0000_0000_0000)
    }
}

impl std::ops::Mul for f128 {
    type Output = f128;

    fn mul(self, other: f128) -> f128 {
        if self == NAN || other == NAN {
            return NAN;
        } else if self == ZERO || other == ZERO {
            return ZERO;
        } else if self == NEG_ZERO || other == NEG_ZERO {
            return NEG_ZERO;
        } else if self == INF || other == INF {
            return INF;
        } else if self == NEG_INF || other == NEG_INF {
            return NEG_INF;
        }

        let frac = floating_point_mul_112(self.frac(), other.frac())
            + self.frac()
            + other.frac()
            + TWO_TO_THE_112;
        let overflow = 15 - frac.leading_zeros();
        let frac = (frac >> overflow) - TWO_TO_THE_112;
        let exp = (self.exp() + other.exp() + TWO_TO_THE_14 + overflow as i16) as u128;
        let sign = if self.sign() == other.sign() {
            1 as u128
        } else {
            0
        } << 127;
        f128(sign | exp << 112 | frac)
    }
}

impl std::ops::MulAssign for f128 {
    fn mul_assign(&mut self, other: f128) {
        *self = *self * other;
    }
}

impl std::ops::Div for f128 {
    type Output = f128;

    fn div(self, other: f128) -> f128 {
        if self == NAN || other == NAN {
            return NAN;
        } else if other == ZERO {
            return NAN;
        } else if other.abs() == INF {
            return ZERO.with_sign(other.sign());
        } else if self.abs() == INF {
            return INF.with_sign(self.sign());
        }

        let recip = {
            let other = (other.frac() + TWO_TO_THE_112) >> 1;
            let start = 0x0002_D2D2_D2D2_D2D2_D2D2_D2D2_D2D2_D2D2
                - fixp_mul(0x0001_E1E1_E1E1_E1E1_E1E1_E1E1_E1E1_E1E1, other);
            (0..5).fold(start, |x, _| 2 * x - fixp_mul(fixp_mul(other, x), x))
        };
        let overflow = 15 - recip.leading_zeros() as i16;
        let recip = (recip >> overflow) - TWO_TO_THE_112 as u128;
        self * f128::new(other.sign(), -other.exp() + overflow - 1, recip)
    }
}

impl std::ops::DivAssign for f128 {
    fn div_assign(&mut self, other: f128) {
        *self = *self / other;
    }
}

impl std::ops::Add for f128 {
    type Output = f128;

    fn add(self, other: f128) -> f128 {
        if self == NAN || other == NAN {
            return NAN;
        } else if self.abs() == INF {
            return if self == -other { NAN } else { self };
        } else if other.abs() == INF {
            return other;
        }

        // don't use PartialOrd here, we dont need its NAN checks and sign check, we're only
        // interested in the magnitude of the number
        let (self_exp, other_exp) = (self.exp(), other.exp());
        let (great, small) = if self_exp > other_exp {
            (self, other)
        } else if self_exp < other_exp {
            (other, self)
        } else if self.frac() > other.frac() {
            (self, other)
        } else {
            (other, self)
        };
        let exp_diff = ((great.0 & 0x7FFF_0000_0000_0000_0000_0000_0000_0000)
            - (small.0 & 0x7FFF_0000_0000_0000_0000_0000_0000_0000))
            >> 112;
        let frac = if exp_diff > 112 {
            great.frac() + TWO_TO_THE_112
        } else if great.sign() == small.sign() {
            great.frac() + TWO_TO_THE_112 + (TWO_TO_THE_112 + small.frac() >> exp_diff)
        } else {
            great.frac() + TWO_TO_THE_112 - (TWO_TO_THE_112 + small.frac() >> exp_diff)
        };
        if frac == 0 {
            return ZERO;
        }
        let overflow = 15i16 - frac.leading_zeros() as i16;
        let frac = if overflow > 0 {
            (frac >> overflow) - TWO_TO_THE_112
        } else {
            (frac << -overflow) - TWO_TO_THE_112
        };
        let exp = great.exp() + overflow;
        f128::new(great.sign(), exp, frac)
    }
}

impl std::ops::AddAssign for f128 {
    fn add_assign(&mut self, other: f128) {
        *self = *self + other;
    }
}

impl std::ops::Sub for f128 {
    type Output = f128;

    fn sub(self, other: f128) -> f128 {
        self + -other
    }
}

impl std::ops::SubAssign for f128 {
    fn sub_assign(&mut self, other: f128) {
        *self = *self - other;
    }
}

impl std::ops::Rem for f128 {
    type Output = f128;

    fn rem(self, other: f128) -> f128 {
        if self == NAN || other == NAN || self.abs() == INF || other.abs() == INF {
            return NAN;
        }
        self - (self / other).floor() * other
    }
}

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f128,
    y: f128,
}

impl Point {
    fn new(x: f128, y: f128) -> Self {
        Self { x, y }
    }

    fn ccw(p1: Point, p2: Point, p3: Point) -> f128 {
        let (x1, y1) = (p1.x, p1.y);
        let (x2, y2) = (p2.x, p2.y);
        let (x3, y3) = (p3.x, p3.y);

        (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)
    }

    fn cross(&self, other: Self) -> f128 {
        self.x * other.y - self.y * other.x
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::cmp::Eq for Point {}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl std::cmp::Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x == other.x {
            self.y.partial_cmp(&other.y)
        } else {
            self.x.partial_cmp(&other.x)
        }
    }
}

struct ConvexHull {
    points: Vec<Point>,
}

impl ConvexHull {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn make_upper(&mut self) -> Vec<Point> {
        let mut ret = Vec::new();

        for p in self.points.iter() {
            while ret.len() >= 2
                && Point::ccw(ret[ret.len() - 2], ret[ret.len() - 1], *p) <= f128::from(0.0)
            {
                ret.pop();
            }

            ret.push(*p);
        }

        ret
    }

    fn make_lower(&mut self) -> Vec<Point> {
        let mut ret = Vec::new();

        for p in self.points.iter() {
            while ret.len() >= 2
                && Point::ccw(ret[ret.len() - 2], ret[ret.len() - 1], *p) >= f128::from(0.0)
            {
                ret.pop();
            }

            ret.push(*p);
        }

        ret
    }
}

#[inline]
fn calculate_value(b: f128, p: &Point, q: &Point) -> f128 {
    (b * (p.x - q.x) + (p.y - q.y)).abs()
}

fn rotating_calipers(points: &[Point]) -> f128 {
    let n = points.len();
    let mut j = if n < 2 { 0 } else { 1 };

    let mut y_min = points[0].y;
    let mut y_max = points[0].y;

    for point in points.iter() {
        if point.y < y_min {
            y_min = point.y;
        }

        if point.y > y_max {
            y_max = point.y;
        }
    }

    let mut ret = y_max - y_min;

    let mut vec = Vec::with_capacity(n);

    for i in 0..n {
        vec.push(Point::new(
            points[(i + 1) % n].x - points[i].x,
            points[(i + 1) % n].y - points[i].y,
        ));
    }

    let mut i = 0;

    while i < j {
        loop {
            let should_break = vec[j].cross(vec[i]) >= f128::from(0.0);
            let vec_b = if should_break { vec[i] } else { vec[j] };

            if i == 0 && !should_break {
                j = (j + 1) % n;
                continue;
            }

            if vec_b.x.abs() > EPSILON {
                let b = -vec_b.y / vec_b.x;
                let val = calculate_value(b, &points[i], &points[j]);

                if val < ret {
                    ret = val;
                }
            }

            if should_break {
                break;
            }

            j = (j + 1) % n;
        }

        i += 1;
    }

    ret
}

fn calculate_a_fixed(points: &[(i32, i32)], a: f128) -> f128 {
    let mut y_delta = Vec::with_capacity(points.len());
    let mut y_min = f128::from(f64::MAX);
    let mut y_max = f128::from(f64::MIN);

    for i in 0..points.len() {
        y_delta.push(Point::new(
            f128::from(points[i].0),
            a * f128::from(points[i].0) * f128::from(points[i].0) - f128::from(points[i].1),
        ));

        if y_delta[i].y < y_min {
            y_min = y_delta[i].y;
        }

        if y_delta[i].y > y_max {
            y_max = y_delta[i].y;
        }
    }

    for i in 0..points.len() {
        y_delta[i].y -= (y_min + y_max) / f128::from(2.0);
    }

    let mut convex_hull = ConvexHull::new(y_delta);
    let upper = convex_hull.make_upper();
    let lower = convex_hull.make_lower();
    let points = if upper.last().unwrap() == lower.last().unwrap() {
        let mut ret = upper
            .iter()
            .chain(lower.iter().rev().skip(1))
            .cloned()
            .collect::<Vec<Point>>();
        ret.pop().unwrap();
        ret
    } else {
        upper
            .iter()
            .chain(lower.iter().rev())
            .cloned()
            .collect::<Vec<Point>>()
    };

    return rotating_calipers(&points);
}

// Reference: https://github.com/dacin21/dacin21_codebook
// Reference: "2022-2023 Winter Petrozavodsk Camp, Day 2: GP of ainta" Editorial
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut points = Vec::with_capacity(n);
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for _ in 0..n {
            let x = scan.token::<i32>();
            let y = scan.token::<i32>();

            points.push((x, y));

            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        points.sort();

        for i in 0..n {
            points[i].0 -= (min_x + max_x) / 2;
            points[i].1 -= (min_y + max_y) / 2;
        }

        // Choose a by using the golden section search
        let golden_ratio = f128::from(1.618033988749894848204);
        let mut a_left = f128::from(-1e13);
        let mut a_right = f128::from(1e13);
        let mut mid1 = (golden_ratio * a_left + a_right) / (f128::from(1.0) + golden_ratio);
        let mut mid2 = (a_left + golden_ratio * a_right) / (f128::from(1.0) + golden_ratio);
        let mut val_mid1 = calculate_a_fixed(&points, mid1);
        let mut val_mid2 = calculate_a_fixed(&points, mid2);
        let mut ret = if val_mid1 < val_mid2 {
            val_mid1
        } else {
            val_mid2
        };

        for _ in 0..200 {
            if val_mid1 < val_mid2 {
                a_right = mid2;
                mid2 = mid1;
                val_mid2 = val_mid1;
                mid1 = (golden_ratio * a_left + a_right) / (f128::from(1.0) + golden_ratio);
                val_mid1 = calculate_a_fixed(&points, mid1);
            } else {
                a_left = mid1;
                mid1 = mid2;
                val_mid1 = val_mid2;
                mid2 = (a_left + golden_ratio * a_right) / (f128::from(1.0) + golden_ratio);
                val_mid2 = calculate_a_fixed(&points, mid2);
            }

            let min = if val_mid1 < val_mid2 {
                val_mid1
            } else {
                val_mid2
            };

            if ret > min {
                ret = min;
            }
        }

        writeln!(out, "{:.12}", ret * ret / f128::from(4.0)).unwrap();
    }
}

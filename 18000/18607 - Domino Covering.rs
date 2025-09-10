#![allow(dead_code)]

use io::Write;
use std::{io, str};

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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ModInt {
    value: i64,
    modulo: i64,
}

impl ModInt {
    #[inline(always)]
    fn new(value: i64, modulo: i64) -> Self {
        let mut val = value % modulo;

        if val < 0 {
            val += modulo;
        }

        ModInt { value: val, modulo }
    }

    #[inline(always)]
    fn pow(self, mut exp: i64) -> Self {
        let m = self.modulo;
        let mut base = self.value;
        let mut ret = 1;

        while exp > 0 {
            if exp & 1 == 1 {
                ret = (ret * base) % m;
            }

            base = (base * base) % m;
            exp >>= 1;
        }

        ModInt::new(ret, m)
    }

    #[inline(always)]
    fn inv(self) -> Self {
        self.pow(self.modulo - 2)
    }
}

#[inline(always)]
fn add_mod(a: i64, b: i64, m: i64) -> i64 {
    let mut ret = a + b;

    if ret >= m {
        ret -= m;
    }

    ret
}

#[inline(always)]
fn sub_mod(a: i64, b: i64, m: i64) -> i64 {
    let mut ret = a - b;

    if ret < 0 {
        ret += m;
    }

    ret
}

impl std::ops::Add for ModInt {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        ModInt {
            value: add_mod(self.value, other.value, self.modulo),
            modulo: self.modulo,
        }
    }
}

impl std::ops::Sub for ModInt {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        ModInt {
            value: sub_mod(self.value, other.value, self.modulo),
            modulo: self.modulo,
        }
    }
}

impl std::ops::Mul for ModInt {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        ModInt {
            value: (self.value * other.value) % self.modulo,
            modulo: self.modulo,
        }
    }
}

#[derive(Clone)]
struct Polynomial {
    coeffs: Vec<ModInt>,
}

impl Polynomial {
    #[inline(always)]
    fn new_with_coeffs(mut coeffs: Vec<ModInt>) -> Self {
        while coeffs.len() > 1 && coeffs.last().unwrap().value == 0 {
            coeffs.pop();
        }

        Self { coeffs }
    }

    #[inline(always)]
    fn constant(c: i64, modulo: i64) -> Self {
        Self::new_with_coeffs(vec![ModInt::new(c.rem_euclid(modulo), modulo)])
    }

    #[inline(always)]
    fn monomial_x(modulo: i64) -> Self {
        Self::new_with_coeffs(vec![ModInt::new(0, modulo), ModInt::new(1, modulo)])
    }

    #[inline(always)]
    fn zero(modulo: i64) -> Self {
        Self::new_with_coeffs(vec![ModInt::new(0, modulo)])
    }

    #[inline(always)]
    fn one(modulo: i64) -> Self {
        Self::new_with_coeffs(vec![ModInt::new(1, modulo)])
    }

    #[inline(always)]
    fn deg(&self) -> i64 {
        if self.coeffs.len() == 1 && self.coeffs[0].value == 0 {
            -1
        } else {
            self.coeffs.len() as i64 - 1
        }
    }

    #[inline(always)]
    fn modulo(&self) -> i64 {
        self.coeffs[0].modulo
    }

    #[inline(always)]
    fn coeff_leading(&self) -> ModInt {
        *self.coeffs.last().unwrap()
    }

    #[inline(always)]
    fn scale(&self, k: ModInt) -> Self {
        let ret = self.coeffs.iter().map(|&c| c * k).collect::<Vec<_>>();
        Polynomial::new_with_coeffs(ret)
    }

    #[inline(always)]
    fn multiply_x(&self) -> Self {
        let modulo = self.coeffs[0].modulo;

        if self.deg() < 0 {
            return Polynomial::zero(modulo);
        }

        let mut ret = Vec::with_capacity(self.coeffs.len() + 1);
        ret.push(ModInt::new(0, modulo));
        ret.extend_from_slice(&self.coeffs);

        Polynomial::new_with_coeffs(ret)
    }
}

impl<'a, 'b> std::ops::Add<&'b Polynomial> for &'a Polynomial {
    type Output = Polynomial;

    #[inline(always)]
    fn add(self, other: &'b Polynomial) -> Polynomial {
        let n = self.coeffs.len().max(other.coeffs.len());
        let modulo = self.modulo();
        let mut ret = vec![ModInt::new(0, modulo); n];

        for i in 0..n {
            let a = if i < self.coeffs.len() {
                self.coeffs[i]
            } else {
                ModInt::new(0, modulo)
            };
            let b = if i < other.coeffs.len() {
                other.coeffs[i]
            } else {
                ModInt::new(0, modulo)
            };

            ret[i] = a + b;
        }

        Polynomial::new_with_coeffs(ret)
    }
}

impl std::ops::Add for Polynomial {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        (&self) + (&other)
    }
}

impl<'a, 'b> std::ops::Sub<&'b Polynomial> for &'a Polynomial {
    type Output = Polynomial;

    #[inline(always)]
    fn sub(self, other: &'b Polynomial) -> Polynomial {
        let n = self.coeffs.len().max(other.coeffs.len());
        let modulo = self.modulo();
        let mut ret = vec![ModInt::new(0, modulo); n];

        for i in 0..n {
            let a = if i < self.coeffs.len() {
                self.coeffs[i]
            } else {
                ModInt::new(0, modulo)
            };
            let b = if i < other.coeffs.len() {
                other.coeffs[i]
            } else {
                ModInt::new(0, modulo)
            };

            ret[i] = a - b;
        }

        Polynomial::new_with_coeffs(ret)
    }
}

impl std::ops::Sub for Polynomial {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        (&self) - (&other)
    }
}

impl<'a, 'b> std::ops::Mul<&'b Polynomial> for &'a Polynomial {
    type Output = Polynomial;

    #[inline(always)]
    fn mul(self, other: &'b Polynomial) -> Polynomial {
        let modulo = self.modulo();

        if self.deg() < 0 || other.deg() < 0 {
            return Polynomial::zero(modulo);
        }

        let mut ret = vec![ModInt::new(0, modulo); self.coeffs.len() + other.coeffs.len() - 1];

        for i in 0..self.coeffs.len() {
            let ai = self.coeffs[i];

            if ai.value == 0 {
                continue;
            }

            for j in 0..other.coeffs.len() {
                let bj = other.coeffs[j];

                if bj.value == 0 {
                    continue;
                }

                let idx = i + j;
                let val = ret[idx] + (ai * bj);

                ret[idx] = val;
            }
        }

        Polynomial::new_with_coeffs(ret)
    }
}

impl std::ops::Mul for Polynomial {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        (&self) * (&other)
    }
}

impl std::ops::Rem<&Polynomial> for Polynomial {
    type Output = Polynomial;

    #[inline(always)]
    fn rem(self, divisor: &Polynomial) -> Polynomial {
        let modulo = self.coeffs[0].modulo;

        if self.deg() < divisor.deg() || divisor.deg() < 0 {
            return Polynomial::new_with_coeffs(self.coeffs.clone());
        }

        let mut ret = self.coeffs.clone();
        let degree = divisor.deg() as usize;
        let coeff_leading_inv = divisor.coeff_leading().inv();

        while (ret.len() as i64 - 1) >= divisor.deg() {
            let shift = ret.len() - 1 - degree;
            let coef = ModInt::new(ret.last().unwrap().value, modulo) * coeff_leading_inv;

            ret.pop();

            for i in 0..degree {
                let idx = shift + i;
                let sub = (divisor.coeffs[i] * coef).value;
                let value_new = sub_mod(ret[idx].value, sub, modulo);

                ret[idx].value = value_new;
            }

            while ret.len() > 1 && ret.last().unwrap().value == 0 {
                ret.pop();
            }
        }

        Polynomial::new_with_coeffs(ret)
    }
}

impl std::ops::Rem for Polynomial {
    type Output = Self;

    #[inline(always)]
    fn rem(self, divisor: Self) -> Self {
        self % &divisor
    }
}

fn char_poly(n: usize, modulo: i64) -> Polynomial {
    let x = Polynomial::monomial_x(modulo);

    if n == 0 {
        return Polynomial::one(modulo);
    }

    if n == 1 {
        return x;
    }

    let mut f_prev = Polynomial::one(modulo);
    let mut f_curr = Polynomial::monomial_x(modulo);

    for _ in 2..=n {
        let f_next = f_curr.multiply_x() - f_prev;
        f_prev = f_curr;
        f_curr = f_next;
    }

    f_curr
}

fn r_poly_from_char(poly_char: &Polynomial, w: usize) -> Polynomial {
    let modulo = poly_char.coeffs[0].modulo;
    let mut coeffs = Vec::with_capacity((w + 2) / 2);

    if w & 1 == 0 {
        for i in (0..=w).step_by(2) {
            coeffs.push(if i < poly_char.coeffs.len() {
                poly_char.coeffs[i]
            } else {
                ModInt::new(0, modulo)
            });
        }
    } else {
        for i in (1..=w).step_by(2) {
            coeffs.push(if i < poly_char.coeffs.len() {
                poly_char.coeffs[i]
            } else {
                ModInt::new(0, modulo)
            });
        }
    }

    Polynomial::new_with_coeffs(coeffs)
}

fn multiply_poly2x2(
    a: &[[Polynomial; 2]; 2],
    b: &[[Polynomial; 2]; 2],
    poly: &Polynomial,
) -> [[Polynomial; 2]; 2] {
    let c00 = ((&a[0][0] * &b[0][0]) + (&a[0][1] * &b[1][0])) % poly;
    let c01 = ((&a[0][0] * &b[0][1]) + (&a[0][1] * &b[1][1])) % poly;
    let c10 = ((&a[1][0] * &b[0][0]) + (&a[1][1] * &b[1][0])) % poly;
    let c11 = ((&a[1][0] * &b[0][1]) + (&a[1][1] * &b[1][1])) % poly;

    [[c00, c01], [c10, c11]]
}

fn multiply_poly2x2_vec(
    a: &[[Polynomial; 2]; 2],
    v: &[Polynomial; 2],
    poly: &Polynomial,
) -> [Polynomial; 2] {
    let r0 = ((&a[0][0] * &v[0]) + (&a[0][1] * &v[1])) % poly;
    let r1 = ((&a[1][0] * &v[0]) + (&a[1][1] * &v[1])) % poly;

    [r0, r1]
}

fn calculate(poly: &Polynomial, m: i64, is_even: bool) -> Polynomial {
    let modulo = poly.coeffs[0].modulo;
    let one = Polynomial::one(modulo);
    let zero = Polynomial::zero(modulo);
    let x = Polynomial::monomial_x(modulo);
    let x_plus_two = &x + &Polynomial::constant(2, modulo);
    let minus_one = Polynomial::constant(modulo - 1, modulo);

    let mut base = [
        [x_plus_two.clone(), minus_one.clone()],
        [one.clone(), zero.clone()],
    ];
    let mut ret = if is_even {
        [&x + &Polynomial::constant(1, modulo), one.clone()]
    } else {
        [x_plus_two.clone(), one.clone()]
    };
    let mut e = m;

    while e > 0 {
        if e & 1 == 1 {
            ret = multiply_poly2x2_vec(&base, &ret, poly);
        }

        base = multiply_poly2x2(&base, &base, poly);
        e >>= 1;
    }

    ret[1].clone()
}

fn resultant(mut f: Polynomial, mut g: Polynomial) -> ModInt {
    let modulo = f.coeffs[0].modulo;
    let mut ret = ModInt::new(1, modulo);

    loop {
        if g.deg() < 0 {
            return ModInt::new(0, modulo);
        }

        if g.deg() == 0 {
            let f_deg = f.deg().max(0) as i64;
            let g_coeff = g.coeffs[0];

            return ret * g_coeff.pow(f_deg);
        }

        let r = f.clone() % &g;

        if r.deg() < 0 {
            return ModInt::new(0, modulo);
        }

        let deg_f = f.deg() as i64;
        let deg_g = g.deg() as i64;
        let deg_r = r.deg() as i64;

        if (deg_f * deg_g) & 1 == 1 {
            ret = ret * ModInt::new(modulo - 1, modulo);
        }

        let g_coeff_leading_pow = g.coeff_leading().pow(deg_f - deg_r);

        ret = ret * g_coeff_leading_pow;
        f = g;
        g = r;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (mut n, mut m, p) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if n & 1 == 1 && m & 1 == 1 {
            writeln!(out, "0").unwrap();
            continue;
        }

        if n > m {
            std::mem::swap(&mut n, &mut m);
        }

        let poly_char = char_poly(n as usize, p);
        let r = r_poly_from_char(&poly_char, n as usize);

        if r.deg() == 0 {
            writeln!(out, "{}", 1 % p).unwrap();
            continue;
        }

        let poly_f = if m & 1 == 0 {
            calculate(&r, m / 2, true)
        } else {
            calculate(&r, (m - 1) / 2, false)
        };
        let ret = resultant(r, poly_f).value;

        writeln!(out, "{ret}").unwrap();
    }
}

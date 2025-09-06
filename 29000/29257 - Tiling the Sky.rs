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
    fn new(value: i64, modulo: i64) -> Self {
        ModInt {
            value: value % modulo,
            modulo,
        }
    }

    fn pow(self, mut exp: i64) -> Self {
        let mut base = self.value;
        let mut ret = 1;

        while exp > 0 {
            if exp % 2 == 1 {
                ret = (ret * base) % self.modulo;
            }

            base = (base * base) % self.modulo;
            exp /= 2;
        }

        ModInt::new(ret, self.modulo)
    }

    fn inv(self) -> Self {
        self.pow(self.modulo - 2)
    }
}

impl std::ops::Add for ModInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ModInt {
            value: (self.value + other.value) % self.modulo,
            modulo: self.modulo,
        }
    }
}

impl std::ops::Sub for ModInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        ModInt {
            value: (self.value - other.value + self.modulo) % self.modulo,
            modulo: self.modulo,
        }
    }
}

impl std::ops::Mul for ModInt {
    type Output = Self;

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
    fn new_with_coeffs(mut coeffs: Vec<ModInt>) -> Self {
        while coeffs.len() > 1 && coeffs.last().unwrap().value == 0 {
            coeffs.pop();
        }

        Self { coeffs }
    }

    fn constant(c: i64, modulo: i64) -> Self {
        Self::new_with_coeffs(vec![ModInt::new(c.rem_euclid(modulo), modulo)])
    }

    fn monomial_x(modulo: i64) -> Self {
        Self::new_with_coeffs(vec![ModInt::new(0, modulo), ModInt::new(1, modulo)])
    }

    fn zero(modulo: i64) -> Self {
        Self::new_with_coeffs(vec![ModInt::new(0, modulo)])
    }

    fn one(modulo: i64) -> Self {
        Self::new_with_coeffs(vec![ModInt::new(1, modulo)])
    }

    #[inline]
    fn deg(&self) -> i64 {
        if self.coeffs.len() == 1 && self.coeffs[0].value == 0 {
            -1
        } else {
            self.coeffs.len() as i64 - 1
        }
    }

    #[inline]
    fn coeff_leading(&self) -> ModInt {
        *self.coeffs.last().unwrap()
    }

    fn scale(&self, k: ModInt) -> Self {
        let ret = self.coeffs.iter().map(|&c| c * k).collect::<Vec<_>>();
        Polynomial::new_with_coeffs(ret)
    }

    fn multiply_x(&self) -> Self {
        let modulo = self.coeffs[0].modulo;

        if self.deg() < 0 {
            return Polynomial::zero(modulo);
        }

        let mut ret = vec![ModInt::new(0, modulo)];
        ret.extend_from_slice(&self.coeffs);

        Polynomial::new_with_coeffs(ret)
    }
}

impl std::ops::Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let n = self.coeffs.len().max(other.coeffs.len());
        let modulo = self.coeffs[0].modulo;
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

impl std::ops::Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let n = self.coeffs.len().max(other.coeffs.len());
        let modulo = self.coeffs[0].modulo;
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

impl std::ops::Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let modulo = self.coeffs[0].modulo;

        if self.deg() < 0 || other.deg() < 0 {
            return Polynomial::zero(modulo);
        }

        let mut ret = vec![ModInt::new(0, modulo); self.coeffs.len() + other.coeffs.len() - 1];

        for i in 0..self.coeffs.len() {
            if self.coeffs[i].value == 0 {
                continue;
            }

            for j in 0..other.coeffs.len() {
                if other.coeffs[j].value == 0 {
                    continue;
                }

                let val = ret[i + j] + (self.coeffs[i] * other.coeffs[j]);
                ret[i + j] = val;
            }
        }

        Polynomial::new_with_coeffs(ret)
    }
}

impl std::ops::Rem for Polynomial {
    type Output = Self;

    fn rem(self, divisor: Self) -> Self {
        let modulo = self.coeffs[0].modulo;

        if self.deg() < divisor.deg() || divisor.deg() < 0 {
            return Polynomial::new_with_coeffs(self.coeffs.clone());
        }

        let mut ret = self.coeffs.clone();
        let degree = divisor.deg() as usize;
        let coeff_leading_inv = divisor.coeff_leading().inv();

        while ret.len() as i64 - 1 >= divisor.deg() {
            let shift = ret.len() - 1 - degree;
            let coef = ModInt::new(ret.last().unwrap().value, modulo) * coeff_leading_inv;

            ret.pop();

            for i in 0..degree {
                let idx = shift + i;
                let t = divisor.coeffs[i] * coef;

                ret[idx].value = (ret[idx].value - t.value + modulo) % modulo;
            }

            while ret.len() > 1 && ret.last().unwrap().value == 0 {
                ret.pop();
            }
        }

        Polynomial::new_with_coeffs(ret)
    }
}

fn char_poly(m: usize, modulo: i64) -> Polynomial {
    let x = Polynomial::monomial_x(modulo);
    let g1 = x.clone();
    let g2 = x.clone() * x.clone() + Polynomial::constant(1, modulo);

    let (mut g_prev, mut g_curr) = (g1, g2);

    for _ in 3..=m - 1 {
        let g_next = x.clone() * g_curr.clone() + g_prev;
        g_prev = g_curr;
        g_curr = g_next;
    }

    let g_m_minus1 = g_curr;
    let g_m_minus2 = g_prev;

    let term1 = g_m_minus1.multiply_x();
    let term2 = g_m_minus2.scale(ModInt::new(2, modulo));
    let constant = if m % 2 == 0 {
        Polynomial::constant(2, modulo)
    } else {
        Polynomial::constant(0, modulo)
    };

    term1 + term2 + constant
}

fn multiply_poly2x2(
    a: &[[Polynomial; 2]; 2],
    b: &[[Polynomial; 2]; 2],
    poly: &Polynomial,
) -> [[Polynomial; 2]; 2] {
    let c00 =
        (a[0][0].clone() * b[0][0].clone() + a[0][1].clone() * b[1][0].clone()) % poly.clone();
    let c01 =
        (a[0][0].clone() * b[0][1].clone() + a[0][1].clone() * b[1][1].clone()) % poly.clone();
    let c10 =
        (a[1][0].clone() * b[0][0].clone() + a[1][1].clone() * b[1][0].clone()) % poly.clone();
    let c11 =
        (a[1][0].clone() * b[0][1].clone() + a[1][1].clone() * b[1][1].clone()) % poly.clone();

    [[c00, c01], [c10, c11]]
}

fn calculate(poly: &Polynomial, n: i64) -> Polynomial {
    let modulo = poly.coeffs[0].modulo;
    let one = Polynomial::one(modulo);
    let zero = Polynomial::zero(modulo);
    let x = Polynomial::monomial_x(modulo);
    let minus_one = Polynomial::constant(modulo - 1, modulo);

    let mut m = [[x.clone(), minus_one.clone()], [one.clone(), zero.clone()]];
    let mut r = [[one.clone(), zero.clone()], [zero.clone(), one.clone()]];
    let mut e = if n == 0 { 0 } else { n - 1 };

    while e > 0 {
        if e % 2 == 1 {
            r = multiply_poly2x2(&r, &m, poly);
        }

        m = multiply_poly2x2(&m, &m, poly);
        e /= 2;
    }

    (r[0][0].clone() * x.clone() + r[0][1].clone() * one.clone()) % poly.clone()
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

        let r = f.clone() % g.clone();

        if r.deg() < 0 {
            return ModInt::new(0, modulo);
        }

        let deg_f = f.deg() as i64;
        let deg_g = g.deg() as i64;
        let deg_r = r.deg() as i64;

        if (deg_f * deg_g) % 2 == 1 {
            ret = ret * ModInt::new(modulo - 1, modulo);
        }

        let g_coeff_leading_pow = g.coeff_leading().pow(deg_f - deg_r);

        ret = ret * g_coeff_leading_pow;
        f = g;
        g = r;
    }
}

fn sign_correction(m: usize, n: i64, modulo: i64) -> ModInt {
    let is_minus = m % 2 == 1 && (n >> 1) % 2 == 1;

    if is_minus {
        ModInt::new(modulo - 1, modulo)
    } else {
        ModInt::new(1, modulo)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, p) = (
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );

    if n % 2 == 1 && m % 2 == 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    let step1 = char_poly(m, p);
    let step2 = calculate(&step1, n);
    let mut ret = resultant(step1, step2);

    ret = ret * sign_correction(m, n, p);

    writeln!(out, "{}", ret.value).unwrap();
}

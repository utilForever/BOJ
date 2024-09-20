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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const MOD: i64 = 1_000_000_007;

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: i64,
    imag: i64,
}

impl Complex {
    fn new(real: i64, imag: i64) -> Self {
        Complex {
            real: real % MOD,
            imag: imag % MOD,
        }
    }

    fn add(self, other: Complex) -> Complex {
        Complex::new(
            (self.real + other.real) % MOD,
            (self.imag + other.imag) % MOD,
        )
    }

    fn mul(self, other: Complex) -> Complex {
        let real = (self.real * other.real - self.imag * other.imag) % MOD;
        let imag = (self.real * other.imag + self.imag * other.real) % MOD;

        Complex::new(real, imag)
    }

    fn div(self, other: Complex) -> Complex {
        let denominator = (other.real * other.real + other.imag * other.imag) % MOD;
        let denominator_inv = Complex::mod_inverse(denominator);

        let real = (self.real * other.real + self.imag * other.imag) % MOD;
        let imag = (self.imag * other.real - self.real * other.imag) % MOD;

        Complex::new(real * denominator_inv % MOD, imag * denominator_inv % MOD)
    }

    fn mod_inverse(x: i64) -> i64 {
        Complex::mod_pow(x, MOD - 2)
    }

    fn pow(mut self, mut exp: i64) -> Complex {
        let mut ret = Complex::new(1, 0);

        while exp > 0 {
            if exp % 2 == 1 {
                ret = ret.mul(self);
            }

            self = self.mul(self);
            exp /= 2;
        }

        ret
    }

    fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
        let mut ret = 1;

        base %= MOD;

        while exp > 0 {
            if exp % 2 == 1 {
                ret = ret * base % MOD;
            }

            base = base * base % MOD;
            exp /= 2;
        }

        ret
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (i, j, k) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        let n = j - i + 1;
        let unit = Complex::new(1, 2);
        let m = Complex::new(-1, 0);

        let a = unit.pow(i * k);
        let r = unit.pow(k);
        let d = r.add(m);

        let f_n = r.pow(n);
        let numerator = a.mul(f_n.add(m));
        let ret = numerator.div(d);
        let ret = (ret.real % MOD + MOD) % MOD;

        writeln!(out, "{ret}").unwrap();
    }
}

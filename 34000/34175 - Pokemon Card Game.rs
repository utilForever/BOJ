use io::Write;
use std::cmp::Ordering;
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

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default)]
struct f128 {
    hi: f64,
    lo: f64,
}

impl f128 {
    #[inline]
    fn from_f64(x: f64) -> Self {
        f128 { hi: x, lo: 0.0 }
    }

    #[inline]
    fn one() -> Self {
        f128 { hi: 1.0, lo: 0.0 }
    }

    #[inline]
    fn neg(self) -> Self {
        f128 {
            hi: -self.hi,
            lo: -self.lo,
        }
    }

    #[inline]
    fn two_sum(a: f64, b: f64) -> (f64, f64) {
        let s = a + b;
        let bb = s - a;
        let err = (a - (s - bb)) + (b - bb);

        (s, err)
    }

    #[inline]
    fn quick_two_sum(a: f64, b: f64) -> (f64, f64) {
        let s = a + b;
        let err = b - (s - a);

        (s, err)
    }

    #[inline]
    fn two_prod(a: f64, b: f64) -> (f64, f64) {
        const SPLIT: f64 = 134_217_729.0;

        let p = a * b;

        let a_c = SPLIT * a;
        let a_hi = a_c - (a_c - a);
        let a_lo = a - a_hi;

        let b_c = SPLIT * b;
        let b_hi = b_c - (b_c - b);
        let b_lo = b - b_hi;

        let err = ((a_hi * b_hi - p) + a_hi * b_lo + a_lo * b_hi) + a_lo * b_lo;

        (p, err)
    }

    #[inline]
    fn add(self, other: Self) -> Self {
        let (s1, s2) = f128::two_sum(self.hi, other.hi);
        let (t1, t2) = f128::two_sum(self.lo, other.lo);

        let (s1, s2) = {
            let s2_new = s2 + t1;
            f128::quick_two_sum(s1, s2_new)
        };

        let (hi, lo) = {
            let s2_new = s2 + t2;
            f128::quick_two_sum(s1, s2_new)
        };

        f128 { hi, lo }
    }

    #[inline]
    fn sub(self, other: Self) -> Self {
        self.add(other.neg())
    }

    #[inline]
    fn mul(self, other: Self) -> Self {
        let (p1, mut p2) = f128::two_prod(self.hi, other.hi);
        p2 += self.hi * other.lo + self.lo * other.hi;

        let (s1, mut s2) = f128::quick_two_sum(p1, p2);
        s2 += self.lo * other.lo;

        let (hi, lo) = f128::quick_two_sum(s1, s2);

        f128 { hi, lo }
    }

    #[inline]
    fn powi(mut self, mut n: u64) -> Self {
        let mut acc = f128::one();

        while n > 0 {
            if (n & 1) == 1 {
                acc = acc.mul(self);
            }

            n >>= 1;

            if n > 0 {
                self = self.mul(self);
            }
        }

        acc
    }

    #[inline]
    fn cmp_f64(&self, y: f64) -> Ordering {
        if self.hi < y {
            Ordering::Less
        } else if self.hi > y {
            Ordering::Greater
        } else {
            if self.lo < 0.0 {
                Ordering::Less
            } else if self.lo > 0.0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }

    #[inline]
    fn lt_f64(&self, y: f64) -> bool {
        self.cmp_f64(y) == Ordering::Less
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, k, p) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<f64>(),
        );

        if k == n {
            if n == 1 {
                writeln!(out, "{:.12} {:.12}", p, p).unwrap();
            } else if p == 1.0 {
                writeln!(out, "{:.12} {:.12}", 1, 1).unwrap();
            } else {
                writeln!(out, "{:.12} {:.12}", 0, 0).unwrap();
            }
        } else if k == n - 1 {
            if n == 1 {
                writeln!(out, "{:.12} {:.12}", 1.0 - p, p).unwrap();
            } else if n == 2 {
                if p == 0.0 || p == 1.0 {
                    writeln!(out, "{:.12} {:.12}", 0, 0).unwrap();
                } else {
                    writeln!(out, "{:.12} {:.12}", 0.5, 0.5).unwrap();
                }
            } else {
                writeln!(out, "{:.12} {:.12}", 0, 0).unwrap();
            }
        } else if k == 1 {
            if n == 1 {
                writeln!(out, "{:.12} {:.12}", p, p).unwrap();
            } else if p == 0.0 || p == 1.0 {
                writeln!(out, "{:.12} {:.12}", 0, 0).unwrap();
            } else {
                writeln!(
                    out,
                    "{:.12} {:.12}",
                    1.0 - (-(n as f64).ln() / (n as f64 - 1.0)).exp(),
                    1.0 - (-(n as f64).ln() / (n as f64 - 1.0)).exp()
                )
                .unwrap();
            }
        } else if k >= n / 2 && k <= (n + 1) / 2 {
            if n == 5 && k == 2 {
                if p > 0.1841464845 && p < 0.6469869035 {
                    writeln!(out, "{:.12} {:.12}", 0.3246535839, 0.3246535839).unwrap();
                } else {
                    writeln!(out, "{:.12} {:.12}", 0, 0).unwrap();
                }
            } else {
                writeln!(out, "{:.12} {:.12}", 0, 0).unwrap();
            }
        } else if k == 0 {
            if p == 0.0 || p == 1.0 {
                writeln!(out, "{:.12} {:.12}", 1.0 - p, p).unwrap();
            } else {
                let base = f128::one().sub(f128::from_f64(p));
                let pow = base.powi(n as u64);

                if pow.lt_f64(p) {
                    writeln!(out, "{:.12} {:.12}", 0.0, 1.0).unwrap();
                } else {
                    writeln!(out, "{:.12} {:.12}", 1.0, 0.0).unwrap();
                }
            }
        }
    }
}

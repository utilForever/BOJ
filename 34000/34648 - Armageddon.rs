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

const MOD: i64 = 1_000_000_007;

#[derive(Debug, Clone, Copy)]
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

impl std::cmp::PartialEq for ModInt {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl std::cmp::PartialOrd for ModInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (p, q, n) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let a = ModInt::new(p, MOD) * ModInt::new(q, MOD).inv();
    let (mut x, mut y, mut z) = (1, 1, 0);

    write!(out, "0 ").unwrap();

    if n > 1 {
        write!(out, "1 ").unwrap();
    }

    for _ in 2..n {
        if x > y {
            let val1 = (y + 2) * q;
            let val2 = y * p;

            if val1 > val2 {
                y += 1;
            } else {
                z += 1;
            }
        } else {
            let val1 = (x + 2) * q;
            let val2 = x * p;

            if val1 > val2 {
                x += 1;
            } else {
                z += 1;
            }
        }

        let val1 = ModInt::new(x * (x + 1) / 2, MOD);
        let val2 = ModInt::new(y * (y + 1) / 2, MOD);
        let ret = val1 * val2 * a.pow(z);

        write!(out, "{} ", ret.value).unwrap();
    }

    writeln!(out).unwrap();
}

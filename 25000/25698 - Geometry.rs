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

const MOD: i64 = 998_244_353;

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

// Reference: https://en.wikipedia.org/wiki/Plane_partition#Plane_partitions_in_a_box
// Reference: https://www.stat.berkeley.edu/~vadicgor/Random_tilings.pdf
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut fact = vec![ModInt::new(1, MOD); 4_000_001];

    for i in 1..=4_000_000 {
        fact[i] = fact[i - 1] * ModInt::new(i as i64, MOD);
    }

    let mut fact_hyper = vec![ModInt::new(1, MOD); 4_000_001];

    for i in 1..=4_000_000 {
        fact_hyper[i] = fact_hyper[i - 1] * fact[i - 1];
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (a, b, c) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        let s1 = (a + b - c).max(0);
        let s2 = (b + c - a).max(0);
        let s3 = (c + a - b).max(0);
        
        let a1 = 4 * a * b - s1 * s1;
        let a2 = 4 * b * c - s2 * s2;
        let a3 = 4 * c * a - s3 * s3;
        let size = a1.max(a2).max(a3);

        let mut numerator = fact_hyper[s1 as usize];
        numerator = numerator * fact_hyper[s2 as usize];
        numerator = numerator * fact_hyper[s3 as usize];
        numerator = numerator * fact_hyper[(s1 + s2 + s3) as usize];

        let mut denomiator = fact_hyper[(s1 + s2) as usize];
        denomiator = denomiator * fact_hyper[(s2 + s3) as usize];
        denomiator = denomiator * fact_hyper[(s3 + s1) as usize];

        let ret = numerator * denomiator.inv();

        writeln!(out, "{size} {}", ret.value).unwrap();
    }
}

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
}

static MOD: i64 = 1_000_000_007;

#[derive(Clone, Copy)]
struct Base {
    a: i64,
    b: i64,
}

impl Base {
    fn new(a: i64, b: i64) -> Self {
        Self { a, b }
    }
}

impl std::ops::Add for Base {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            a: (self.a + other.a) % MOD,
            b: (self.b + other.b) % MOD,
        }
    }
}

impl std::ops::Mul for Base {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            a: (self.a * other.b + self.b * other.a + self.a * other.a % MOD * (MOD - 1)) % MOD,
            b: (self.b * other.b + self.a * other.a % MOD * (MOD - 1)) % MOD,
        }
    }
}

fn process_fwht(poly: &mut Vec<Base>, inv: bool) {
    let mut omega1 = Base::new(1, 0);
    let mut omega2 = Base::new(MOD - 1, MOD - 1);

    if inv {
        std::mem::swap(&mut omega1, &mut omega2);
    }

    let mut len = 1;

    while len < poly.len() {
        let mut i = 0;

        while i < poly.len() {
            for j in 0..len {
                let u = poly[i + j];
                let v = poly[i + j + len];
                let w = poly[i + j + len * 2];

                poly[i + j] = u + v + w;
                poly[i + j + len] = u + omega1 * v + omega2 * w;
                poly[i + j + len * 2] = u + omega2 * v + omega1 * w;
            }

            i += 3 * len;
        }

        len *= 3;
    }

    if inv {
        for i in 0..poly.len() {
            poly[i] = poly[i] * Base::new(0, 237480738);
        }
    }
}

fn gpow(a: &mut Base, mut b: i64) -> Base {
    let mut ret = Base::new(0, 1);

    while b > 0 {
        if b % 2 == 1 {
            ret = ret * *a;
        }

        *a = *a * *a;
        b /= 2;
    }

    ret
}

fn multiply(ret: &mut Vec<i64>, values: &Vec<i64>, k: i64) {
    let n = 59049;

    let mut poly = Vec::new();

    for i in 0..n {
        poly.push(Base::new(0, values[i]));
    }

    process_fwht(&mut poly, false);

    for i in 0..n {
        poly[i] = gpow(&mut poly[i], k);
    }

    process_fwht(&mut poly, true);

    for i in 0..n {
        ret[i] = poly[i].b;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut members = vec![0; 59049];
    let mut ret = vec![0; 59049];

    for _ in 0..m {
        let c = scan.token::<usize>();
        let mut blubs = vec![0; n];

        for _ in 0..c {
            let (num, dir) = (scan.token::<usize>() - 1, scan.token::<char>());
            blubs[num] = if dir == 'L' { 1 } else { 2 };
        }

        let mut idx = 0;

        for j in 0..n {
            idx = idx * 3 + blubs[j];
        }

        members[idx] += 1;
    }

    multiply(&mut ret, &members, k);

    writeln!(out, "{}", ret[0]).unwrap();
}

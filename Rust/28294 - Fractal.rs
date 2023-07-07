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

fn pow(x: i64, mut p: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x;

    while p != 0 {
        if p & 1 != 0 {
            ret = ret * piv % MOD;
        }

        piv = piv * piv % MOD;
        p >>= 1;
    }

    ret
}

fn perimeter(n: i64, a: i64) -> i64 {
    if a == 1 {
        1
    } else if a % 2 == 0 {
        (((pow(n - 1, a / 2) + pow(n, a / 2)) % MOD) * perimeter(n, a / 2)) % MOD
    } else {
        (pow(n - 1, a - 1) + n * perimeter(n, a - 1) % MOD) % MOD
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, a) = (scan.token::<i64>(), scan.token::<i64>());

    let ret = (n * (pow(n, a) + ((n - 2) * perimeter(n, a)) % MOD) % MOD) % MOD;

    writeln!(out, "{ret}").unwrap();
}

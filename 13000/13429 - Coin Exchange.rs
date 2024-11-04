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

const MOD: i64 = 1_000_000_007;

fn pow(x: i64, mut y: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % MOD;

    while y != 0 {
        if y & 1 != 0 {
            ret = ret * piv % MOD
        }

        piv = piv * piv % MOD;
        y >>= 1;
    }

    ret
}

fn combination(factorial: &Vec<i64>, n: usize, r: usize) -> i64 {
    factorial[n] * pow(factorial[r] * factorial[n - r] % MOD, MOD - 2) % MOD
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut factorial = vec![1; 2_000_001];

    for i in 2..=2_000_000 {
        factorial[i] = factorial[i - 1] * i as i64 % MOD;
    }

    let d = scan.token::<usize>();
    let mut w = vec![0; d + 1];

    for i in 0..=d {
        w[i] = scan.token::<i64>();
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (v, n) = (scan.token::<usize>(), scan.token::<usize>());
        let mut ret = 0;

        for k in 0..=(d / v) {
            let comb = combination(&factorial, n + k - 1, k);
            ret = (ret + comb * w[d - k * v] * (-1i64).pow(k as u32)) % MOD;
        }

        if ret < 0 {
            ret = (ret + MOD) % MOD;
        }

        writeln!(out, "{ret}").unwrap();
    }
}

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

fn multiply(x: i64, y: i64) -> i64 {
    (x as i128 * y as i128 % MOD as i128) as i64
}

fn pow(x: i64, mut y: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % MOD;

    while y != 0 {
        if y & 1 != 0 {
            ret = multiply(ret, piv);
        }

        piv = multiply(piv, piv);
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

    let mut factorial = vec![0; 1_000_001];
    factorial[0] = 1;
    factorial[1] = 1;

    for i in 2..=1_000_000 {
        factorial[i] = factorial[i - 1] * i as i64 % MOD;
    }

    let (n, d, p) = (
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut prev = vec![1; d];
    let mut ret = 1;

    for _ in 0..p {
        let mut curr = vec![0; d];
        let mut sum = 0;

        for i in 0..d {
            curr[i] = scan.token::<i64>();
            sum += (curr[i] - prev[i]).abs();
        }

        for i in 0..d {
            ret *= combination(&factorial, sum as usize, (curr[i] - prev[i]).abs() as usize);
            sum -= (curr[i] - prev[i]).abs();
            ret %= MOD;
        }

        prev = curr;
    }

    let mut sum = 0;

    for i in 0..d {
        sum += (n - prev[i]).abs();
    }

    for i in 0..d {
        ret *= combination(&factorial, sum as usize, (n - prev[i]).abs() as usize);
        sum -= (n - prev[i]).abs();
        ret %= MOD;
    }

    writeln!(out, "{ret}").unwrap();
}

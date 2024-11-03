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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut hours = vec![0; n + 1];

    for i in 0..n {
        hours[i] = m - scan.token::<i64>();
    }

    let sum = hours.iter().sum::<i64>();

    if sum > m {
        writeln!(out, "0").unwrap();
        return;
    }

    hours[n] = m - sum;

    let mut factorial = vec![1; 1_000_001];
    let mut factorial_inv = vec![1; 1_000_001];

    for i in 2..=1_000_000 {
        factorial[i] = factorial[i - 1] * i as i64 % MOD;
    }

    factorial_inv[1_000_000] = pow(factorial[1_000_000], MOD - 2);

    for i in (1..=1_000_000).rev() {
        factorial_inv[i - 1] = factorial_inv[i] * i as i64 % MOD;
    }

    let mut ret = factorial[m as usize];

    for i in 0..=n {
        ret = ret * factorial_inv[hours[i] as usize] % MOD;
    }

    writeln!(out, "{ret}").unwrap();
}

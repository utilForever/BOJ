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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (d, r) = (scan.token::<usize>(), scan.token::<usize>());
    let mut dp = vec![vec![0; r * r + 1]; d + 1];
    let mut sum = vec![vec![0; r * r + 1]; d + 1];

    dp[0][0] = 1;

    for i in 1..=d {
        for j in 0..=r * r {
            let limit = (j as f64).sqrt() as i64;

            for k in -limit..=limit {
                dp[i][j] = (dp[i][j] + dp[i - 1][j - (k * k) as usize]) % MOD;
                sum[i][j] = (sum[i][j]
                    + sum[i - 1][j - (k * k) as usize]
                    + k.abs() * dp[i - 1][j - (k * k) as usize])
                    % MOD;
            }
        }
    }

    let mut ret = 0;

    for i in 0..=r * r {
        ret = (ret + sum[d][i]) % MOD;
    }

    writeln!(out, "{ret}").unwrap();
}

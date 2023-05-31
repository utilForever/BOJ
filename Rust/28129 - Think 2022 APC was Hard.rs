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

static MOD: usize = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut dp = vec![vec![0; 3001]; n + 1];

    for i in 1..=n {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

        if i == 1 {
            for j in a..=b {
                dp[1][j] = 1;
            }
        } else {
            for j in a..=b {
                let val_min = 1.max(j.saturating_sub(k));
                let val_max = 3000.min(j + k);

                dp[i][j] = (dp[i - 1][val_max] - dp[i - 1][val_min - 1] + MOD) % MOD;
            }
        }

        for j in 1..=3000 {
            dp[i][j] = (dp[i][j] + dp[i][j - 1]) % MOD;
        }
    }

    writeln!(out, "{}", dp[n][3000]).unwrap();
}

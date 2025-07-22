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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut calories = vec![0; n];

    for i in 0..n {
        calories[i] = scan.token::<i64>();
    }

    let mut capacities = vec![0; n + 1];
    capacities[0] = m;

    for i in 1..=n {
        capacities[i] = capacities[i - 1] * 2 / 3;
    }

    let mut dp = vec![vec![vec![i64::MIN; 3]; n + 1]; n + 1];
    dp[0][0][2] = 0;

    for i in 0..n {
        for j in 0..=n {
            for k in 0..3 {
                let curr = dp[i][j][k];

                if curr == i64::MIN {
                    continue;
                }

                // Eat
                let eat = calories[i].min(capacities[j]);
                let next_j = (j + 1).min(n);
                dp[i + 1][next_j][0] = dp[i + 1][next_j][0].max(curr + eat);

                // Skip
                let next_k = (k + 1).min(2);
                let next_j = if next_k == 1 {
                    (j as i64 - 1).max(0) as usize
                } else {
                    0
                };
                dp[i + 1][next_j][next_k] = dp[i + 1][next_j][next_k].max(curr);
            }
        }
    }

    let mut ret = 0;

    for i in 0..=n {
        for j in 0..3 {
            ret = ret.max(dp[n][i][j]);
        }
    }

    writeln!(out, "{ret}").unwrap();
}

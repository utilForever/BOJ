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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k, t) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut hypercube = vec![vec![vec![vec![0; t + 1]; k + 1]; m + 1]; n + 1];

    for x in 1..=n {
        for y in 1..=m {
            for z in 1..=k {
                for w in 1..=t {
                    hypercube[x][y][z][w] = scan.token::<i64>();
                }
            }
        }
    }

    let mut dp = vec![vec![vec![vec![0; t + 1]; k + 1]; m + 1]; n + 1];
    let mut ret = 0;

    for x in 1..=n {
        for y in 1..=m {
            for z in 1..=k {
                for w in 1..=t {
                    if hypercube[x][y][z][w] == 1 {
                        continue;
                    }

                    dp[x][y][z][w] = dp[x - 1][y][z][w]
                        .min(dp[x][y - 1][z][w])
                        .min(dp[x][y][z - 1][w])
                        .min(dp[x][y][z][w - 1]);
                    dp[x][y][z][w] = dp[x][y][z][w]
                        .min(dp[x - 1][y - 1][z][w])
                        .min(dp[x - 1][y][z - 1][w])
                        .min(dp[x - 1][y][z][w - 1]);
                    dp[x][y][z][w] = dp[x][y][z][w]
                        .min(dp[x][y - 1][z - 1][w])
                        .min(dp[x][y - 1][z][w - 1]);
                    dp[x][y][z][w] = dp[x][y][z][w].min(dp[x][y][z - 1][w - 1]);
                    dp[x][y][z][w] = dp[x][y][z][w]
                        .min(dp[x - 1][y - 1][z - 1][w])
                        .min(dp[x - 1][y - 1][z][w - 1])
                        .min(dp[x - 1][y][z - 1][w - 1])
                        .min(dp[x][y - 1][z - 1][w - 1]);
                    dp[x][y][z][w] = dp[x][y][z][w].min(dp[x - 1][y - 1][z - 1][w - 1]);
                    dp[x][y][z][w] += 1;

                    ret = ret.max(dp[x][y][z][w]);
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

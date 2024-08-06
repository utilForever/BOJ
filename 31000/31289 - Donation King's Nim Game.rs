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

    let t = scan.token::<i64>();
    let mut dp = vec![vec![vec![0; 201]; 201]; 201];

    for i in 0..=200 {
        for j in 0..=200 {
            for k in 0..=200 {
                if (i ^ j ^ k) == 0 {
                    dp[i][j][k] = 0;

                    for a in 0..i {
                        dp[i][j][k] = dp[i][j][k].max(dp[a][j][k] + i - a);
                    }

                    for b in 0..j {
                        dp[i][j][k] = dp[i][j][k].max(dp[i][b][k] + j - b);
                    }

                    for c in 0..k {
                        dp[i][j][k] = dp[i][j][k].max(dp[i][j][c] + k - c);
                    }
                } else {
                    dp[i][j][k] = 1000;

                    if (j ^ k) < i {
                        dp[i][j][k] = dp[i][j][k].min(dp[j ^ k][j][k]);
                    }

                    if (i ^ k) < j {
                        dp[i][j][k] = dp[i][j][k].min(dp[i][i ^ k][k]);
                    }

                    if (i ^ j) < k {
                        dp[i][j][k] = dp[i][j][k].min(dp[i][j][i ^ j]);
                    }
                }
            }
        }
    }

    for _ in 0..t {
        let (x, y, z) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        let mut ret_first = dp[x][y][z];
        let mut ret_second = x + y + z - ret_first;

        if (x ^ y ^ z) != 0 {
            std::mem::swap(&mut ret_first, &mut ret_second);
        }

        writeln!(out, "{} {}", ret_first * 10000, ret_second * 10000).unwrap();
    }
}

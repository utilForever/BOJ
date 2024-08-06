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

    let (f, l, mut t, n, m) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    t = t.min(f);

    let mut tactics = vec![vec![vec![0; m]; n]; f];
    let mut tactics_todo = vec![0; l];
    let mut dp = vec![vec![vec![0_i64; t]; f]; f];

    for i in 0..f {
        for j in 0..n {
            for k in 0..m {
                tactics[i][j][k] = scan.token::<i64>();
            }
        }
    }

    for i in 0..l {
        tactics_todo[i] = scan.token::<usize>() - 1;
    }

    for i in 0..f - 1 {
        for j in i + 1..f {
            let mut diff = 0;

            for r in 0..n {
                for c in 0..m {
                    if tactics[i][r][c] != tactics[j][r][c] {
                        diff += 1;
                    }
                }
            }

            dp[i][j][0] = diff * diff;
            dp[j][i][0] = diff * diff;
        }
    }

    for i in 1..t {
        for j in 0..f {
            for k in 0..f {
                dp[j][k][i] = dp[j][k][i - 1];

                for l in 0..f {
                    dp[j][k][i] = dp[j][k][i].min(dp[j][l][i - 1] + dp[l][k][0]);
                }
            }
        }
    }

    let mut idx_curr = 0;
    let mut ret = 0;

    for i in 0..l {
        let idx_next = tactics_todo[i];

        ret += dp[idx_curr][idx_next][t - 1];
        idx_curr = idx_next;
    }

    writeln!(out, "{ret}").unwrap();
}

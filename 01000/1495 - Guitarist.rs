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

    let (n, s, m) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut volumes = vec![0; n + 1];

    for i in 1..=n {
        volumes[i] = scan.token::<i64>();
    }

    let mut dp = vec![vec![false; m + 1]; n + 1];
    dp[0][s] = true;

    for i in 1..=n {
        for j in 0..=m {
            if dp[i - 1][j] {
                if j as i64 + volumes[i] <= m as i64 {
                    dp[i][j + volumes[i] as usize] = true;
                }

                if j as i64 - volumes[i] >= 0 {
                    dp[i][j - volumes[i] as usize] = true;
                }
            }
        }
    }

    let mut ret = -1;

    for idx in 0..=m {
        if dp[n][idx] {
            ret = idx as i64;
        }
    }

    writeln!(out, "{ret}").unwrap();
}

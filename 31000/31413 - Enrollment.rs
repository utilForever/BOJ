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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut services = vec![0; n * 2 + 1];

    for i in 1..=n {
        services[i] = scan.token::<i64>();
    }

    let (a, d) = (scan.token::<i64>(), scan.token::<usize>());
    let mut dp = vec![vec![0; n + d + 1]; n + 1];

    for i in 1..=n {
        dp[0][i] = dp[0][i - 1] + services[i];
    }

    if dp[0][n] >= m {
        writeln!(out, "0").unwrap();
        return;
    }

    for i in 1..=n {
        for j in d..n + d {
            dp[i][j] = (dp[i][j - 1] + services[j]).max(dp[i - 1][j - d] + a);

            if dp[i][j] >= m {
                writeln!(out, "{i}").unwrap();
                return;
            }
        }
    }

    writeln!(out, "-1").unwrap();
}

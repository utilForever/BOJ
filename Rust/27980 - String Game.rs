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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let (s, t) = (scan.token::<String>(), scan.token::<String>());
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![0; n + 2]; 2];

    for i in 1..=n {
        if s[i - 1] == t[0] {
            dp[0][i] = 1;
        }
    }

    let mut idx = 0;

    for i in 1..m {
        for j in 1..=n {
            dp[idx ^ 1][j] = dp[idx][j - 1].max(dp[idx][j + 1]);

            if s[j - 1] == t[i] {
                dp[idx ^ 1][j] += 1;
            }
        }

        idx ^= 1;
    }

    let mut ret = 0;

    for i in 1..=n {
        ret = ret.max(dp[idx][i]);
    }

    writeln!(out, "{ret}").unwrap();
}

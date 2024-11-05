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
    let mut operators = vec![vec![-1; 2]; n + 1];

    for _ in 0..m {
        let (x, y, t) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        operators[y][x / 2] = t;
    }

    for i in 1..=n {
        if operators[i][0] == 0 && operators[i][1] == 0 {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    let mut dp = vec![vec![i64::MAX / 2; 5]; n + 1];
    dp[0][0] = 0;
    dp[0][1] = 0;

    for i in 1..=n {
        if operators[i][0] != 0 && operators[i][1] != 0 {
            dp[i][4] = dp[i - 1].iter().min().unwrap() + 2;
        }

        if operators[i][0] != 1 && operators[i][1] != 0 {
            dp[i][3] = dp[i - 1][1] + 1;

            if i > 1 {
                dp[i][1] = dp[i - 1][0].min(dp[i - 1][2]).min(dp[i - 1][4]) + 1;
            }
        }

        if operators[i][0] != 0 && operators[i][1] != 1 {
            dp[i][2] = dp[i - 1][0] + 1;

            if i > 1 {
                dp[i][0] = dp[i - 1][1].min(dp[i - 1][3]).min(dp[i - 1][4]) + 1;
            }
        }
    }

    let ret = dp[n][0].min(dp[n][1]).min(dp[n][4]);

    if ret >= i64::MAX / 2 {
        writeln!(out, "-1").unwrap();
        return;
    }

    writeln!(out, "{ret}").unwrap();
}

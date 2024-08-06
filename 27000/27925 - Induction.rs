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

    let n = scan.token::<usize>();
    let mut foods = vec![0; n + 1];

    for i in 1..=n {
        foods[i] = scan.token::<i64>();
    }

    let mut dp = vec![vec![vec![1_000_000_007; 10]; 10]; n + 1];
    dp[0][0][0] = 0;

    for i in 0..n {
        for j in 0..10 {
            for k in 0..10 {
                let mut diff_a = (foods[i + 1] - foods[i]).abs();
                let mut diff_b = (foods[i + 1] - j as i64).abs();
                let mut diff_c = (foods[i + 1] - k as i64).abs();

                diff_a = diff_a.min(10 - diff_a);
                diff_b = diff_b.min(10 - diff_b);
                diff_c = diff_c.min(10 - diff_c);

                dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k] + diff_a);
                dp[i + 1][foods[i] as usize][k] =
                    dp[i + 1][foods[i] as usize][k].min(dp[i][j][k] + diff_b);
                dp[i + 1][foods[i] as usize][j] =
                    dp[i + 1][foods[i] as usize][j].min(dp[i][j][k] + diff_c);
            }
        }
    }

    let mut ret = i64::MAX;

    for i in 0..10 {
        for j in 0..10 {
            ret = ret.min(dp[n][i][j]);
        }
    }

    writeln!(out, "{ret}").unwrap();
}

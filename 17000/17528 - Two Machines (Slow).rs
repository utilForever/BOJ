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
    let mut tasks = vec![(0, 0); n];

    for i in 0..n {
        tasks[i] = (scan.token::<usize>(), scan.token::<usize>());
    }

    let a_sum = tasks.iter().map(|x| x.0).sum::<usize>();
    let mut dp = vec![vec![usize::MAX; a_sum + 1]; n + 1];

    dp[0][0] = 0;

    for i in 0..n {
        let (a, b) = tasks[i];

        for j in 0..=a_sum {
            if dp[i][j] == usize::MAX {
                continue;
            }

            if j + a <= a_sum {
                dp[i + 1][j + a] = dp[i + 1][j + a].min(dp[i][j]);
            }

            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + b);
        }
    }

    let mut ret = usize::MAX;

    for j in 0..=a_sum {
        if dp[n][j] == usize::MAX {
            continue;
        }

        ret = ret.min(j.max(dp[n][j]));
    }

    writeln!(out, "{ret}").unwrap();
}

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

    let (n, k, t) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut heights = vec![0; n];

    for i in 0..n {
        heights[i] = scan.token::<i64>();
    }

    let mut heights_min = vec![vec![0; n]; n];

    for i in 0..n {
        let mut height_curr = heights[i];

        for j in i..n {
            height_curr = height_curr.min(heights[j]);
            heights_min[i][j] = height_curr;
        }
    }

    let mut dp = vec![vec![0; n + 1]; k + 1];

    for i in 1..=k {
        for j in 1..=n {
            dp[i][j] = dp[i][j - 1];

            for x in 0..j.min(t) {
                let start = j - x - 1;
                dp[i][j] =
                    dp[i][j].max(dp[i - 1][start] + heights_min[start][j - 1] * (x + 1) as i64);
            }
        }
    }

    writeln!(out, "{}", dp[k][n]).unwrap();
}

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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let n = scan.token::<usize>();
    let mut matrix = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = scan.token::<i32>();
        }
    }

    let p = scan.token::<usize>();
    let mut intermediates = vec![(0, 0); p];

    for i in 0..p {
        intermediates[i] = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
    }

    intermediates.push((n - 1, n - 1));
    intermediates.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    for i in 1..p {
        if intermediates[i].1 < intermediates[i - 1].1 {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    let mut dp = vec![vec![0; n]; n];
    dp[0][0] = matrix[0][0];

    let mut r_prev = 0;
    let mut c_prev = 0;

    for (r, c) in intermediates {
        for i in r_prev..=r {
            for j in c_prev..=c {
                if i > r_prev && j > c_prev {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]) + matrix[i][j];
                } else if i > r_prev && dp[i - 1][j] > 0 {
                    dp[i][j] = dp[i - 1][j] + matrix[i][j];
                } else if j > c_prev && dp[i][j - 1] > 0 {
                    dp[i][j] = dp[i][j - 1] + matrix[i][j];
                }
            }
        }

        r_prev = r;
        c_prev = c;
    }

    writeln!(out, "{}", dp[n - 1][n - 1]).unwrap();
}

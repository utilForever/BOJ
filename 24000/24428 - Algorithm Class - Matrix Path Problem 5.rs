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
            matrix[i][j] = scan.token::<i64>();
        }
    }

    let p = scan.token::<usize>();
    let mut intermediates = vec![vec![false; n]; n];

    for _ in 0..p {
        let (r, c) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        intermediates[r][c] = true;
    }

    let mut dp = vec![vec![vec![i64::MIN; 4]; n]; n];
    dp[0][0][0] = matrix[0][0];

    for i in 0..n {
        for j in 0..n {
            if i == 0 && j == 0 {
                continue;
            }

            let offset = if intermediates[i][j] { 1 } else { 0 };

            if i > 0 {
                for k in 0..4 {
                    let prev = dp[i - 1][j][k];

                    if prev == i64::MIN {
                        continue;
                    }

                    let k_new = (k + offset).min(3);
                    dp[i][j][k_new] = dp[i][j][k_new].max(prev + matrix[i][j]);
                }
            }

            if j > 0 {
                for k in 0..4 {
                    let prev = dp[i][j - 1][k];

                    if prev == i64::MIN {
                        continue;
                    }

                    let k_new = (k + offset).min(3);
                    dp[i][j][k_new] = dp[i][j][k_new].max(prev + matrix[i][j]);
                }
            }
        }
    }

    let ret = dp[n - 1][n - 1][3];

    writeln!(out, "{}", if ret == i64::MIN { -1 } else { ret }).unwrap();
}

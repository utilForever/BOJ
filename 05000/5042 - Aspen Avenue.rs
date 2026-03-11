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

    loop {
        let line = scan.line().trim().to_string();

        if line.is_empty() {
            break;
        }

        let n = line.parse::<usize>().unwrap();
        let (l, w) = (scan.token::<f64>(), scan.token::<f64>());
        let mut positions = vec![0.0; n];

        for i in 0..n {
            positions[i] = scan.token::<f64>();
        }

        positions.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let m = n / 2;
        let x = (0..m)
            .map(|x| x as f64 * (l / (m - 1) as f64))
            .collect::<Vec<_>>();
        let mut dp = vec![vec![f64::MAX / 2.0; m + 1]; m + 1];

        dp[0][0] = 0.0;

        for left in 0..=m {
            for right in 0..=m {
                if left + right == n {
                    continue;
                }

                if left < m {
                    let cost = (positions[left + right] - x[left]).abs();
                    dp[left + 1][right] = dp[left + 1][right].min(dp[left][right] + cost);
                }

                if right < m {
                    let cost =
                        ((positions[left + right] - x[right]).abs().powi(2) + w.powi(2)).sqrt();
                    dp[left][right + 1] = dp[left][right + 1].min(dp[left][right] + cost);
                }
            }
        }

        writeln!(out, "{:.12}", dp[m][m]).unwrap();
    }
}

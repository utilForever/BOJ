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

    let (l1, l2, p1, p2) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let prob_counter1 = 1.0 / (p1 as f64);
    let prob_counter2 = 1.0 / (p2 as f64);
    let denom = prob_counter1 + prob_counter2 - (prob_counter1 * prob_counter2);

    let mut dp = vec![vec![0.0; l2 + 1]; l1 + 1];

    for i in 1..=l2 {
        dp[0][i] = 1.0;
    }

    for i in 1..=l1 {
        for j in 1..=l2 {
            let case1 =
                prob_counter1 * (1.0 - prob_counter2) * if i == 1 { 1.0 } else { dp[i - 1][j] };
            let case2 =
                prob_counter2 * (1.0 - prob_counter1) * if j == 1 { 0.0 } else { dp[i][j - 1] };
            let case3 = if j == 1 {
                0.0
            } else {
                prob_counter1 * prob_counter2 * if i == 1 { 1.0 } else { dp[i - 1][j - 1] }
            };

            dp[i][j] = (case1 + case2 + case3) / denom;
        }
    }

    writeln!(out, "{:.12}", dp[l1][l2]).unwrap();
}

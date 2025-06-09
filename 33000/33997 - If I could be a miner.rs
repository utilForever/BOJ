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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut waterway = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            waterway[i][j] = scan.token::<i64>();
        }
    }

    let mut dp = vec![0; m];
    let mut bonus_left = vec![0; m];
    let mut bonus_right = vec![0; m];

    for i in 0..n {
        let mut prefix_sum = vec![0; m + 1];

        for j in 0..m {
            prefix_sum[j + 1] = prefix_sum[j] + waterway[i][j];
        }

        let mut prefix_min = 0;
        let mut prefix_max = i64::MIN;

        for j in 0..m {
            bonus_left[j] = (prefix_sum[j + 1] - prefix_min).max(0);
            prefix_min = prefix_min.min(prefix_sum[j + 1]);
        }

        for j in (0..m).rev() {
            prefix_max = prefix_max.max(prefix_sum[j + 1]);
            bonus_right[j] = (prefix_max - prefix_sum[j]).max(0);
        }

        let mut dp_new = vec![i64::MIN; m];
        let mut best = i64::MIN;

        for j in 0..m {
            best = best.max(dp[j] - prefix_sum[j] + if j > 0 { bonus_left[j - 1] } else { 0 });
            dp_new[j] = best + prefix_sum[j + 1] + if j + 1 < m { bonus_right[j + 1] } else { 0 };
        }

        let mut best = i64::MIN;

        for j in (0..m).rev() {
            best = best
                .max(dp[j] + prefix_sum[j + 1] + if j + 1 < m { bonus_right[j + 1] } else { 0 });
            dp_new[j] =
                dp_new[j].max(best - prefix_sum[j] + if j > 0 { bonus_left[j - 1] } else { 0 });
        }

        dp = dp_new;
    }

    writeln!(out, "{}", dp.iter().max().unwrap()).unwrap();
}

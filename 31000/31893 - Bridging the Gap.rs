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

// Reference: 47th ICPC World Finals Solution Sketches
// Reference: https://jh05013.github.io/blog/wf2022/
// Reference: https://www.sciencedirect.com/science/article/pii/S0167642315000118
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut times = vec![0; n];

    for i in 0..n {
        times[i] = scan.token::<i64>();
    }

    times.sort_unstable();

    if n <= c {
        writeln!(out, "{}", times[n - 1]).unwrap();
        return;
    }

    let cost = |from: usize, to: usize| -> i64 {
        let mut ret = times[to - 1];

        for i in 1..=from {
            ret += times[i - 1];
        }

        ret
    };

    let no_pure_trip_max = (n - 2) / c;
    let mut dp = vec![vec![0; no_pure_trip_max + 1]; n];

    // Excess equals 0
    for i in 2..=c {
        dp[i - 1][0] = cost(0, i);
    }

    // Excess e > 0
    for i in 2..=c {
        for j in 1..=no_pure_trip_max {
            let idx = i.min(j + 1);
            let mut val = cost(idx, i) + dp[idx - 1][j - idx + 1];

            for k in (2..=idx - 1).rev() {
                val = val.min(cost(k, i) + dp[k - 1][j - k + 1]);
            }

            dp[i - 1][j] = val;
        }
    }

    // n = c + 1
    for i in 0..=no_pure_trip_max {
        let mut val = cost(1, c + 1) + dp[1][i];

        for j in 2..=(i + 1).min(c - 1) {
            val = val.min(cost(j, c + 1) + dp[j][i - j + 1]);
        }

        dp[c][i] = val;
    }

    // n > c + 1
    for i in c + 2..=n {
        let excess_max = (n - i) / c;

        for j in 0..=excess_max {
            let mut val = cost(0, i) + dp[i - c - 1][j + 1];

            for k in 1..=(j + 1).min(c - 1) {
                val = val.min(cost(k, i) + dp[i - (c - k) - 1][j - k + 1]);
            }

            dp[i - 1][j] = val;
        }
    }

    writeln!(out, "{}", dp[n - 1][0]).unwrap();
}

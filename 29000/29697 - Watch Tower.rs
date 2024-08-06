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

    let (n, p) = (scan.token::<usize>(), scan.token::<i64>());
    let mut costs = vec![0; n + 1];
    let mut positions = Vec::new();
    let mut dp = vec![i64::MAX; n + 1];

    for i in 1..=n {
        costs[i] = scan.token::<i64>();
    }

    let mut left = 0;
    let mut right = 0;
    let mut sum = 1;

    while sum + (right + 2) <= p {
        sum += right + 2;
        right += 1;
    }

    while right >= 0 {
        while sum + (left + 2) <= p {
            sum += left + 2;
            left += 1;
        }

        positions.push((left, right));
        positions.push((right, left));

        sum -= right + 1;
        right -= 1;
    }

    positions.sort();
    positions.dedup();

    dp[0] = 0;

    for i in 1..=n {
        for &(x, y) in positions.iter() {
            left = i as i64 - x;
            right = i as i64 + y;

            right = right.min(n as i64);
            left = left.max(1);

            dp[right as usize] = dp[right as usize].min(dp[(left - 1) as usize] + costs[i]);
            dp[i] = dp[i].min(dp[right as usize]);
        }
    }

    writeln!(out, "{}", dp[n]).unwrap();
}

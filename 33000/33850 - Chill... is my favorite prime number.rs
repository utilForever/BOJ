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

    let (n, a, b) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut grid = vec![vec![0; n]; 2];

    for i in 0..2 {
        for j in 0..n {
            grid[i][j] = scan.token::<usize>();
        }
    }

    let mut is_prime = vec![true; 200_001];
    is_prime[1] = false;

    let mut i = 2;

    while i * i <= 200_000 {
        if !is_prime[i] {
            i += 1;
            continue;
        }

        for j in (i * i..=200_000).step_by(i) {
            is_prime[j] = false;
        }

        i += 1;
    }

    let mut vertical = vec![0; n];
    let mut horizontal = vec![0; n - 1];

    for i in 0..n {
        vertical[i] = if is_prime[grid[0][i] + grid[1][i]] {
            a
        } else {
            b
        };
    }

    for i in 1..n {
        let sum1 = if is_prime[grid[0][i] + grid[0][i - 1]] {
            a
        } else {
            b
        };

        let sum2 = if is_prime[grid[1][i] + grid[1][i - 1]] {
            a
        } else {
            b
        };

        horizontal[i - 1] = sum1 + sum2;
    }

    let mut dp = vec![0; n + 1];
    dp[1] = vertical[0];

    for i in 2..=n {
        dp[i] = (dp[i - 1] + vertical[i - 1]).max(dp[i - 2] + horizontal[i - 2]);
    }

    writeln!(out, "{}", dp[n]).unwrap();
}

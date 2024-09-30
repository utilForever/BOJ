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

const MOD: usize = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut stones = vec![0; n];

    for i in 0..n {
        stones[i] = scan.token::<usize>();
    }

    let stones_total = stones.iter().sum::<usize>();

    if stones_total % 2 != 0 {
        writeln!(out, "0").unwrap();
        return;
    }

    let target = stones_total / 2;

    let mut dp = vec![vec![0; target + 1]; n + 1];
    dp[0][0] = 1;

    // DP[i][s] = The number of ways to choose stones from first i stones summing to s
    for i in 0..n {
        let stone = stones[i];

        for k in (1..=i + 1).rev() {
            for s in (0..=target).rev() {
                if s < stone || dp[k - 1][s - stone] == 0 {
                    continue;
                }

                dp[k][s] = (dp[k][s] + dp[k - 1][s - stone]) % MOD;
            }
        }
    }

    let mut factorial = vec![1; n + 1];

    for i in 1..=n {
        factorial[i] = (factorial[i - 1] * i) % MOD;
    }

    let mut ret = 0;

    // Since the picking order is determined by the difference, we need to consider valid k
    // Unfortunately, we cannot determine valid k directly due to the dependency on the stones' sizes
    // However, since the initial difference is 0 and the total sum is even, we can consider all possible k
    for k in 1..=n {
        if dp[k][target] == 0 {
            continue;
        }

        let ways = factorial[k] * factorial[n - k] % MOD;
        let ways = (ways * dp[k][target]) % MOD;

        ret = (ret + ways) % MOD;
    }

    writeln!(out, "{ret}").unwrap();
}

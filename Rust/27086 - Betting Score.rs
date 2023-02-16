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

const MOD: i64 = 998_244_353;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, x, y) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut scores = vec![0; n];
    let mut cnt_scores = vec![0; 200001];
    let mut prefix_sum = vec![0; 200001];

    for i in 0..n {
        scores[i] = scan.token::<i64>();
    }

    for i in 0..n {
        cnt_scores[scores[i] as usize] += 1;
    }

    for i in 1..=200000 {
        prefix_sum[i] = prefix_sum[i - 1] + cnt_scores[i];
    }

    let mut dp = vec![0; 200001];

    for i in (0..=200000).rev() {
        dp[i] = if i + x <= 200000 {
            dp[i + x] + cnt_scores[i + x]
        } else {
            0
        };
        dp[i] = (dp[i] + n as i64 - prefix_sum[i]) % MOD;
    }

    let mut ret = 0;

    for i in 0..n {
        ret += dp[scores[i] as usize];
        ret %= MOD;
    }

    ret = ret * y % MOD;

    writeln!(out, "{ret} {}", dp[scores[0] as usize] * y % MOD).unwrap();
}

use io::Write;
use std::{io, str, vec};

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

const MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, l, r) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    // n: the number of buildings
    // l: the number of buildings that can see on the left
    // r: the number of buildings that can see on the right
    // dp[i][j][k]: the number of ways to build i buildings with j buildings that can see on the left and k buildings that can see on the right

    // Approach: Let's start with the largest building and add smaller buildings one by one. We can add a building in three ways:
    // 1. Add a building that can see on the left
    // 2. Add a building that can see on the right
    // 3. Add a building that can't see on the left or right (i - 2 buildings can't see on the left or right)
    let mut dp = vec![vec![vec![0; r + 1]; l + 1]; n + 1];

    dp[1][1][1] = 1;

    for i in 2..=n {
        for j in 1..=l {
            for k in 1..=r {
                dp[i][j][k] = (dp[i - 1][j][k] * (i - 2) as i64 % MOD
                    + dp[i - 1][j - 1][k] % MOD
                    + dp[i - 1][j][k - 1] % MOD)
                    % MOD;
            }
        }
    }

    writeln!(out, "{}", dp[n][l][r]).unwrap();
}

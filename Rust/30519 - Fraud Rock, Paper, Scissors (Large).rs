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

static MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let lighter = scan.token::<char>();
    let smallants = scan.token::<String>();
    let smallants = smallants.chars().collect::<Vec<_>>();
    let n = smallants.len();

    let mut dp = vec![vec![0; 9]; n + 1];

    if lighter == 'R' {
        dp[0][0] = 1;
    } else if lighter == 'P' {
        dp[0][3] = 1;
    } else {
        dp[0][6] = 1;
    }

    for i in 1..=n {
        if smallants[i - 1] == 'R' {
            dp[i][0] = (dp[i - 1][0] + dp[i - 1][2]) % MOD;
            dp[i][1] = (dp[i - 1][3] + dp[i - 1][4] + dp[i - 1][5]) % MOD;
            dp[i][2] = (dp[i - 1][6] + dp[i - 1][7] + dp[i - 1][8]) % MOD;
        } else if smallants[i - 1] == 'P' {
            dp[i][3] = (dp[i - 1][3] + dp[i - 1][5]) % MOD;
            dp[i][4] = (dp[i - 1][6] + dp[i - 1][7] + dp[i - 1][8]) % MOD;
            dp[i][5] = (dp[i - 1][0] + dp[i - 1][1] + dp[i - 1][2]) % MOD;
        } else {
            dp[i][6] = (dp[i - 1][6] + dp[i - 1][8]) % MOD;
            dp[i][7] = (dp[i - 1][0] + dp[i - 1][1] + dp[i - 1][2]) % MOD;
            dp[i][8] = (dp[i - 1][3] + dp[i - 1][4] + dp[i - 1][5]) % MOD;
        }

        for j in 0..9 {
            dp[i][j] = (dp[i][j] + dp[i - 1][j]) % MOD;
        }
    }

    let mut ret = 0;

    for i in 0..9 {
        ret = (ret + dp[n][i]) % MOD;
    }

    writeln!(out, "{}", (ret - 1 + MOD) % MOD).unwrap();
}

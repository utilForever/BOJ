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

static MOD: i64 = 998_244_353;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<usize>();
    let mut dp = vec![vec![0; 8]; s + 1];

    dp[0][1] = 1;

    for i in 1..=s {
        for j in 1..=7 {
            dp[i][j] = (dp[i][j] + dp[i - 1][j - 1]) % MOD;
            dp[i][j] = (dp[i][j] + dp[i - 1][j] * 6) % MOD;
        }
    }

    let mut ret = vec![0; s + 1];

    for i in 1..=s {
        ret[i] = (ret[i] + dp[i - 1][7]) % MOD;
        ret[i] = (ret[i] + ret[i - 1] * 7) % MOD;
    }

    writeln!(out, "{}", ret[s]).unwrap();
}

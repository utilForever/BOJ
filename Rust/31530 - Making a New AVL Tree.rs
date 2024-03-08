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

static MOD: usize = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let mut dp = vec![vec![0; 1_000_001]; 8];

    for i in 1..8 {
        dp[i][0] = 1;
        dp[i][1] = 1;

        for j in 2..=1_000_000 {
            let val1 = dp[i][j - 1] * dp[i][j - 1] * ((i / 2) % 2);
            let val2 = dp[i][j - 1] * dp[i][j - 2] * ((i / 4) % 2 + i % 2);

            dp[i][j] = (val1 + val2) % MOD;
        }
    }

    for _ in 0..t {
        let (h, s_cnt) = (scan.token::<usize>(), scan.token::<i64>());
        let mut idx = 0;

        for _ in 0..s_cnt {
            let val = scan.token::<i64>() + 1;
            idx |= 1 << val;
        }

        writeln!(out, "{}", dp[idx][h]).unwrap();
    }
}

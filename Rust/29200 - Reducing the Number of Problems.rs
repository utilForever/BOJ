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

    let n = scan.token::<usize>();
    let mut nums = vec![0; n + 1];

    for i in 1..=n {
        nums[i] = scan.token::<i64>();
    }

    let mut dp = vec![vec![i64::MIN; 5]; n + 1];

    for i in 0..5 {
        dp[0][i] = 0;
    }

    for i in 1..=n {
        let mut val = 0;

        for j in 1..=i.min(4) {
            val ^= nums[i - j + 1];

            for k in 1..=4 {
                if j == k {
                    continue;
                }

                dp[i][j] = dp[i][j].max(dp[i - j][k] + val);
            }
        }
    }

    writeln!(out, "{}", dp[n].iter().max().unwrap()).unwrap();
}

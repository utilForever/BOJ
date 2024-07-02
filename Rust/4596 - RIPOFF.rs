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

    loop {
        let n = scan.token::<usize>();
        
        if n == 0 {
            break;
        }

        let (s, t) = (scan.token::<usize>(), scan.token::<usize>());
        let mut coins = vec![0; n + 2];

        for i in 1..=n {
            coins[i] = scan.token::<i64>();
        }

        let mut dp = vec![vec![i64::MIN; n + 2]; t + 1];
        
        for i in 1..=s {
            dp[1][i] = coins[i];
        }

        for i in 2..=t {
            for j in 1..=n {
                if dp[i - 1][j] == i64::MIN {
                    continue;
                }

                for dice in 1..=s {
                    let next = j + dice;

                    if next > n + 1 {
                        break;
                    }

                    dp[i][next] = dp[i][next].max(dp[i - 1][j] + coins[next]);
                }
            }
        }

        let mut ret = i64::MIN;

        for i in 1..=t {
            ret = ret.max(dp[i][n + 1]);
        }

        writeln!(out, "{}", ret).ok();
    }
}

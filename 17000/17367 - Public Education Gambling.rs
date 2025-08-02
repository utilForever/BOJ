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

fn calculate_reward(a: usize, b: usize, c: usize) -> f64 {
    if a == b && b == c {
        10000.0 + (a as f64) * 1000.0
    } else if a == b || a == c {
        1000.0 + (a as f64) * 100.0
    } else if b == c {
        1000.0 + (b as f64) * 100.0
    } else {
        (a.max(b).max(c) as f64) * 100.0
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut dp = vec![vec![vec![[0.0; 6]; 6]; 6]; n - 2];

    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                dp[0][i][j][k] = calculate_reward(i + 1, j + 1, k + 1);
            }
        }
    }

    for x in 1..n - 2 {
        for i in 0..6 {
            for j in 0..6 {
                for k in 0..6 {
                    let reward_stop = calculate_reward(i + 1, j + 1, k + 1);
                    let mut reward_continue = 0.0;

                    for y in 0..6 {
                        reward_continue += dp[x - 1][j][k][y] / 6.0;
                    }

                    dp[x][i][j][k] = reward_stop.max(reward_continue);
                }
            }
        }
    }

    let mut ret = 0.0;

    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                ret += dp[n - 3][i][j][k];
            }
        }
    }

    writeln!(out, "{:.12}", ret / 216.0).unwrap();
}

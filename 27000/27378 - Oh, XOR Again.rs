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

fn calculate(n: u64, k: usize, x: u64) -> u64 {
    const BITS: usize = 61;

    let mut dp = vec![vec![[0; 2]; k + 1]; BITS + 1];
    dp[0][0][1] = 1;

    for i in 0..BITS {
        let idx = 60 - i;
        let a = ((x >> idx) & 1) as u64;
        let b = ((n >> idx) & 1) as u64;

        let offset1 = b as usize;
        let offset2 = (1 ^ b) as usize;

        for j in 0..=k {
            if dp[i][j][0] != 0 {
                if j + offset1 <= k {
                    dp[i + 1][j + offset1][0] += dp[i][j][0];
                }

                if j + offset2 <= k {
                    dp[i + 1][j + offset2][0] += dp[i][j][0];
                }
            }

            if dp[i][j][1] != 0 {
                if a == 0 {
                    if j + offset1 <= k {
                        dp[i + 1][j + offset1][1] += dp[i][j][1];
                    }
                } else {
                    if j + offset1 <= k {
                        dp[i + 1][j + offset1][0] += dp[i][j][1];
                    }

                    if j + offset2 <= k {
                        dp[i + 1][j + offset2][1] += dp[i][j][1];
                    }
                }
            }
        }
    }

    dp[BITS][k][0] + dp[BITS][k][1]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k, a, b) = (
        scan.token::<u64>(),
        scan.token::<usize>(),
        scan.token::<u64>(),
        scan.token::<u64>(),
    );

    writeln!(out, "{}", calculate(n, k, b) - calculate(n, k, a - 1)).unwrap();
}

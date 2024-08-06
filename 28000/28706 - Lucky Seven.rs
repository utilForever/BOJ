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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut dp = vec![vec![0; 7]; n + 1];
        dp[0][1] = 1;

        for i in 1..=n {
            let (op1, v1, op2, v2) = (
                scan.token::<char>(),
                scan.token::<usize>(),
                scan.token::<char>(),
                scan.token::<usize>(),
            );

            for j in 0..7 {
                if dp[i - 1][j] == 0 {
                    continue;
                }

                if op1 == '+' {
                    let idx_next = (j + v1) % 7;
                    dp[i][idx_next] = 1;
                } else {
                    let idx_next = (if j == 0 { 7 } else { j } * v1) % 7;
                    dp[i][idx_next] = 1;
                }

                if op2 == '+' {
                    let idx_next = (j + v2) % 7;
                    dp[i][idx_next] = 1;
                } else {
                    let idx_next = (if j == 0 { 7 } else { j } * v2) % 7;
                    dp[i][idx_next] = 1;
                }
            }
        }

        writeln!(out, "{}", if dp[n][0] == 1 { "LUCKY" } else { "UNLUCKY" }).unwrap();
    }
}

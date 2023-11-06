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
    let mut moves = vec![0; n];
    let mut dp = vec![0; n];

    for i in 0..n {
        moves[i] = scan.token::<usize>();
    }

    dp[0] = 1;

    // First step
    for i in 0..n {
        if moves[i] == 0 || dp[i] == 0 {
            continue;
        }

        if i + moves[i] < n {
            dp[i + moves[i]] = dp[i + moves[i]].max(dp[i] + 1);
        }
    }

    // Second step (reverse)
    for i in (0..n - 1).rev() {
        if moves[i] == 0 || dp[i] == 0 {
            continue;
        }

        if i as i64 - moves[i] as i64 >= 0 {
            dp[i - moves[i]] = dp[i - moves[i]].max(dp[i] + 1);
        }
    }

    // Third step (reverse again)
    for i in 0..n {
        if moves[i] == 0 || dp[i] == 0 {
            continue;
        }

        if i + moves[i] < n {
            dp[i + moves[i]] = dp[i + moves[i]].max(dp[i] + 1);
        }
    }

    writeln!(out, "{}", if dp[n - 1] == 0 { -1 } else { dp[n - 1] - 1 }).unwrap();
}

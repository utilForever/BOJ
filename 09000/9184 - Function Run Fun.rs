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

fn w(dp: &mut Vec<Vec<Vec<i64>>>, a: i64, b: i64, c: i64) -> i64 {
    if a <= 0 || b <= 0 || c <= 0 {
        return 1;
    }

    if a > 20 || b > 20 || c > 20 {
        return w(dp, 20, 20, 20);
    }

    if dp[a as usize][b as usize][c as usize] != 0 {
        return dp[a as usize][b as usize][c as usize];
    }

    if a < b && b < c {
        dp[a as usize][b as usize][c as usize] =
            w(dp, a, b, c - 1) + w(dp, a, b - 1, c - 1) - w(dp, a, b - 1, c);
    }

    dp[a as usize][b as usize][c as usize] =
        w(dp, a - 1, b, c) + w(dp, a - 1, b - 1, c) + w(dp, a - 1, b, c - 1)
            - w(dp, a - 1, b - 1, c - 1);
    dp[a as usize][b as usize][c as usize]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let (a, b, c) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if a == -1 && b == -1 && c == -1 {
            break;
        }

        let mut dp = vec![vec![vec![0; 21]; 21]; 21];

        writeln!(out, "w({a}, {b}, {c}) = {}", w(&mut dp, a, b, c)).unwrap();
    }
}

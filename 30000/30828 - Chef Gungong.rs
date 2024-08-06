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
    let mut tastes = vec![0; n];

    for i in 0..n {
        tastes[i] = scan.token::<usize>();
    }

    let mut dp = vec![vec![vec![0; 512]; n]; n];
    let mut ret = vec![vec![0; n]; n];

    for i in 0..n {
        dp[i][i][tastes[i]] = 1;
        ret[i][i] = tastes[i] as i64 + 1;
    }

    for cnt in 1..n {
        for left in 0..n - cnt {
            let right = left + cnt;

            for i in 0..512 {
                let val_a = dp[left + 1][right][i].max(dp[left][right - 1][i]);
                let val_b = if dp[left + 1][right][i ^ tastes[left]] > 0 {
                    dp[left + 1][right][i ^ tastes[left]] + 1
                } else {
                    0
                };
                let val_c = if dp[left][right - 1][i ^ tastes[right]] > 0 {
                    dp[left][right - 1][i ^ tastes[right]] + 1
                } else {
                    0
                };

                dp[left][right][i] = val_a.max(val_b).max(val_c);

                if dp[left][right][i] > 0 {
                    ret[left][right] = ret[left][right].max(i as i64 + dp[left][right][i])
                }
            }
        }
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (l, r) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        writeln!(out, "{}", ret[l][r]).unwrap();
    }
}

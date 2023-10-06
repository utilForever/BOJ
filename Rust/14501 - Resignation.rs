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
    let mut periods = vec![0; n];
    let mut prices = vec![0; n];
    let mut dp = vec![0; n];

    for i in 0..n {
        (periods[i], prices[i]) = (scan.token::<i64>(), scan.token::<i64>());
    }

    for i in 0..n {
        if i + periods[i] as usize <= n {
            dp[i] = prices[i];
        }
    }

    for i in 1..n {
        for j in 0..i {
            if (i as i64 - j as i64) >= periods[j] && i + periods[i] as usize <= n {
                dp[i] = dp[i].max(dp[j] + prices[i]);
            }
        }
    }

    writeln!(out, "{}", dp.iter().max().unwrap()).unwrap();
}

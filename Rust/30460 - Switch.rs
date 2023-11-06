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
    let mut scores = vec![0; n + 2];

    for i in 0..n {
        scores[i] = scan.token::<i64>();
    }

    let mut dp = vec![0; n + 2];
    dp[0] = scores[0];
    dp[1] = scores[0] + scores[1];
    dp[2] = (scores[0] + scores[1] + scores[2]).max(2 * (scores[0] + scores[1] + scores[2]));

    for i in 3..n + 2 {
        dp[i] = (dp[i - 1] + scores[i])
            .max(dp[i - 3] + 2 * (scores[i] + scores[i - 1] + scores[i - 2]));
    }

    writeln!(out, "{}", dp[n + 1]).unwrap();
}

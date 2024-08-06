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

    let n = scan.token::<String>();
    let n = n.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![0; 3]; n.len()];

    dp[0][0] = n[0] as u16 - b'0' as u16;
    dp[0][1] = 10 - n[0] as u16 + b'0' as u16;
    dp[0][2] = 11 - n[0] as u16 + b'0' as u16;

    let mut ret = dp[0][0].min(dp[0][2]);

    for i in 1..n.len() {
        dp[i][0] = (dp[i - 1][0] + n[i] as u16 - b'0' as u16)
            .min(dp[i - 1][2] + n[i] as u16 - b'0' as u16);
        dp[i][1] = (dp[i - 1][0] + 10 - n[i] as u16 + b'0' as u16)
            .min(dp[i - 1][1] + 9 - n[i] as u16 + b'0' as u16)
            .min(dp[i - 1][2] + 10 - n[i] as u16 + b'0' as u16);
        dp[i][2] = (dp[i - 1][0] + 11 - n[i] as u16 + b'0' as u16)
            .min(dp[i - 1][1] + 10 - n[i] as u16 + b'0' as u16)
            .min(dp[i - 1][2] + 11 - n[i] as u16 + b'0' as u16);

        ret = dp[i][0].min(dp[i][2]);
    }

    writeln!(out, "{ret}",).unwrap();
}

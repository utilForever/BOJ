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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (h, y) = (scan.token::<i64>(), scan.token::<usize>());
    let mut dp = vec![0; y + 1];

    dp[0] = h;

    for i in 1..=y {
        dp[i] = ((dp[i - 1] as f64 * 1.05).floor() as i64)
            .max(if i >= 3 {
                (dp[i - 3] as f64 * 1.2).floor() as i64
            } else {
                0
            })
            .max(if i >= 5 {
                (dp[i - 5] as f64 * 1.35).floor() as i64
            } else {
                0
            });
    }

    writeln!(out, "{}", dp[y]).unwrap();
}

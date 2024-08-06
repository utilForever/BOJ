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

    let (h1, b1) = (scan.token::<i64>(), scan.token::<i64>());
    let (h2, b2) = (scan.token::<i64>(), scan.token::<i64>());
    let score1 = h1 * 3 + b1;
    let score2 = h2 * 3 + b2;

    if score1 == score2 {
        writeln!(out, "NO SCORE").unwrap();
    } else if score1 > score2 {
        writeln!(out, "1 {}", score1 - score2).unwrap();
    } else {
        writeln!(out, "2 {}", score2 - score1).unwrap();
    }
}

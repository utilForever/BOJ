use io::Write;
use std::{cmp, io, str};

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

    let n = scan.token();
    let mut lines = vec![(0, 0); n];

    for i in 0..n {
        lines[i] = (scan.token(), scan.token());
    }

    lines.sort();

    let mut ans = 0;
    let (mut left, mut right) = (-1_000_000_001, -1_000_000_001);

    for i in 0..n {
        if right < lines[i].0 {
            ans += right - left;
            left = lines[i].0;
            right = lines[i].1;
        } else {
            right = cmp::max(right, lines[i].1);
        }
    }

    ans += right - left;

    writeln!(out, "{}", ans).unwrap();
}

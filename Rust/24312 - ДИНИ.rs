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

    let (a, b, c, d) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut ret = i64::MAX;

    for i in 0..=1 {
        for j in 0..=1 {
            for k in 0..=1 {
                for l in 0..=1 {
                    let sum_left = if i == 0 { a } else { 0 }
                        + if j == 0 { b } else { 0 }
                        + if k == 0 { c } else { 0 }
                        + if l == 0 { d } else { 0 };
                    let sum_right = if i == 1 { a } else { 0 }
                        + if j == 1 { b } else { 0 }
                        + if k == 1 { c } else { 0 }
                        + if l == 1 { d } else { 0 };

                    ret = ret.min((sum_left - sum_right).abs());
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

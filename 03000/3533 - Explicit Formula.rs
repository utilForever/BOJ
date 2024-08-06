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

    let (x1, x2, x3, x4, x5, x6, x7, x8, x9, x10) = (
        scan.token::<i64>() == 1,
        scan.token::<i64>() == 1,
        scan.token::<i64>() == 1,
        scan.token::<i64>() == 1,
        scan.token::<i64>() == 1,
        scan.token::<i64>() == 1,
        scan.token::<i64>() == 1,
        scan.token::<i64>() == 1,
        scan.token::<i64>() == 1,
        scan.token::<i64>() == 1,
    );

    let ret = (x1 || x2)
        ^ (x1 || x3)
        ^ (x1 || x4)
        ^ (x1 || x5)
        ^ (x1 || x6)
        ^ (x1 || x7)
        ^ (x1 || x8)
        ^ (x1 || x9)
        ^ (x1 || x10)
        ^ (x2 || x3)
        ^ (x2 || x4)
        ^ (x2 || x5)
        ^ (x2 || x6)
        ^ (x2 || x7)
        ^ (x2 || x8)
        ^ (x2 || x9)
        ^ (x2 || x10)
        ^ (x3 || x4)
        ^ (x3 || x5)
        ^ (x3 || x6)
        ^ (x3 || x7)
        ^ (x3 || x8)
        ^ (x3 || x9)
        ^ (x3 || x10)
        ^ (x4 || x5)
        ^ (x4 || x6)
        ^ (x4 || x7)
        ^ (x4 || x8)
        ^ (x4 || x9)
        ^ (x4 || x10)
        ^ (x5 || x6)
        ^ (x5 || x7)
        ^ (x5 || x8)
        ^ (x5 || x9)
        ^ (x5 || x10)
        ^ (x6 || x7)
        ^ (x6 || x8)
        ^ (x6 || x9)
        ^ (x6 || x10)
        ^ (x7 || x8)
        ^ (x7 || x9)
        ^ (x7 || x10)
        ^ (x8 || x9)
        ^ (x8 || x10)
        ^ (x9 || x10)
        ^ (x1 || x2 || x3)
        ^ (x1 || x2 || x4)
        ^ (x1 || x2 || x5)
        ^ (x1 || x2 || x6)
        ^ (x1 || x2 || x7)
        ^ (x1 || x2 || x8)
        ^ (x1 || x2 || x9)
        ^ (x1 || x2 || x10)
        ^ (x1 || x3 || x4)
        ^ (x1 || x3 || x5)
        ^ (x1 || x3 || x6)
        ^ (x1 || x3 || x7)
        ^ (x1 || x3 || x8)
        ^ (x1 || x3 || x9)
        ^ (x1 || x3 || x10)
        ^ (x1 || x4 || x5)
        ^ (x1 || x4 || x6)
        ^ (x1 || x4 || x7)
        ^ (x1 || x4 || x8)
        ^ (x1 || x4 || x9)
        ^ (x1 || x4 || x10)
        ^ (x1 || x5 || x6)
        ^ (x1 || x5 || x7)
        ^ (x1 || x5 || x8)
        ^ (x1 || x5 || x9)
        ^ (x1 || x5 || x10)
        ^ (x1 || x6 || x7)
        ^ (x1 || x6 || x8)
        ^ (x1 || x6 || x9)
        ^ (x1 || x6 || x10)
        ^ (x1 || x7 || x8)
        ^ (x1 || x7 || x9)
        ^ (x1 || x7 || x10)
        ^ (x1 || x8 || x9)
        ^ (x1 || x8 || x10)
        ^ (x1 || x9 || x10)
        ^ (x2 || x3 || x4)
        ^ (x2 || x3 || x5)
        ^ (x2 || x3 || x6)
        ^ (x2 || x3 || x7)
        ^ (x2 || x3 || x8)
        ^ (x2 || x3 || x9)
        ^ (x2 || x3 || x10)
        ^ (x2 || x4 || x5)
        ^ (x2 || x4 || x6)
        ^ (x2 || x4 || x7)
        ^ (x2 || x4 || x8)
        ^ (x2 || x4 || x9)
        ^ (x2 || x4 || x10)
        ^ (x2 || x5 || x6)
        ^ (x2 || x5 || x7)
        ^ (x2 || x5 || x8)
        ^ (x2 || x5 || x9)
        ^ (x2 || x5 || x10)
        ^ (x2 || x6 || x7)
        ^ (x2 || x6 || x8)
        ^ (x2 || x6 || x9)
        ^ (x2 || x6 || x10)
        ^ (x2 || x7 || x8)
        ^ (x2 || x7 || x9)
        ^ (x2 || x7 || x10)
        ^ (x2 || x8 || x9)
        ^ (x2 || x8 || x10)
        ^ (x2 || x9 || x10)
        ^ (x3 || x4 || x5)
        ^ (x3 || x4 || x6)
        ^ (x3 || x4 || x7)
        ^ (x3 || x4 || x8)
        ^ (x3 || x4 || x9)
        ^ (x3 || x4 || x10)
        ^ (x3 || x5 || x6)
        ^ (x3 || x5 || x7)
        ^ (x3 || x5 || x8)
        ^ (x3 || x5 || x9)
        ^ (x3 || x5 || x10)
        ^ (x3 || x6 || x7)
        ^ (x3 || x6 || x8)
        ^ (x3 || x6 || x9)
        ^ (x3 || x6 || x10)
        ^ (x3 || x7 || x8)
        ^ (x3 || x7 || x9)
        ^ (x3 || x7 || x10)
        ^ (x3 || x8 || x9)
        ^ (x3 || x8 || x10)
        ^ (x3 || x9 || x10)
        ^ (x4 || x5 || x6)
        ^ (x4 || x5 || x7)
        ^ (x4 || x5 || x8)
        ^ (x4 || x5 || x9)
        ^ (x4 || x5 || x10)
        ^ (x4 || x6 || x7)
        ^ (x4 || x6 || x8)
        ^ (x4 || x6 || x9)
        ^ (x4 || x6 || x10)
        ^ (x4 || x7 || x8)
        ^ (x4 || x7 || x9)
        ^ (x4 || x7 || x10)
        ^ (x4 || x8 || x9)
        ^ (x4 || x8 || x10)
        ^ (x4 || x9 || x10)
        ^ (x5 || x6 || x7)
        ^ (x5 || x6 || x8)
        ^ (x5 || x6 || x9)
        ^ (x5 || x6 || x10)
        ^ (x5 || x7 || x8)
        ^ (x5 || x7 || x9)
        ^ (x5 || x7 || x10)
        ^ (x5 || x8 || x9)
        ^ (x5 || x8 || x10)
        ^ (x5 || x9 || x10)
        ^ (x6 || x7 || x8)
        ^ (x6 || x7 || x9)
        ^ (x6 || x7 || x10)
        ^ (x6 || x8 || x9)
        ^ (x6 || x8 || x10)
        ^ (x6 || x9 || x10)
        ^ (x7 || x8 || x9)
        ^ (x7 || x8 || x10)
        ^ (x7 || x9 || x10)
        ^ (x8 || x9 || x10);

    writeln!(out, "{}", if ret { 1 } else { 0 }).unwrap();
}

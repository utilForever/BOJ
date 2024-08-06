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

    let (x_a, y_a) = (scan.token::<i64>(), scan.token::<i64>());
    let (x_b, y_b) = (scan.token::<i64>(), scan.token::<i64>());
    let (a, b) = (scan.token::<i64>(), scan.token::<i64>());

    let dist_x = (x_a - x_b).abs();
    let dist_y = (y_a - y_b).abs();
    let ret = dist_x / a;

    if ret == 0 || ret * b < dist_y {
        writeln!(out, "-1").unwrap();
    } else {
        writeln!(out, "{dist_x} {ret}").unwrap();
        writeln!(out, "{dist_y} {ret}").unwrap();
    }
}

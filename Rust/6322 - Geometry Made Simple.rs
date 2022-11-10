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

    let mut idx = 1;

    loop {
        let (a, b, c) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if a == 0 && b == 0 && c == 0 {
            break;
        }

        writeln!(out, "Triangle #{idx}").unwrap();

        if a == -1 {
            if c * c - b * b > 0 {
                writeln!(out, "a = {:.3}", ((c * c - b * b) as f64).sqrt()).unwrap();
            } else {
                writeln!(out, "Impossible.").unwrap();
            }
        } else if b == -1 {
            if c * c - a * a > 0 {
                writeln!(out, "b = {:.3}", ((c * c - a * a) as f64).sqrt()).unwrap();
            } else {
                writeln!(out, "Impossible.").unwrap();
            }
        } else {
            writeln!(out, "c = {:.3}", ((a * a + b * b) as f64).sqrt()).unwrap();
        }

        writeln!(out).unwrap();

        idx += 1;
    }
}

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

    let a = scan.token::<i64>();
    let (b1, b2) = (scan.token::<i64>(), scan.token::<i64>());
    let (c1, mut c2) = (scan.token::<i64>(), scan.token::<i64>());

    let ret_a = if a == 1 { 1 } else { 5 * a };
    let ret_b = if b1 == 0 || c2 == 0 {
        if b2 == 0 {
            0
        } else if b2 == 1 {
            (c1 + c2).min(2)
        } else {
            (5 * b2).min(c1 + c2)
        }
    } else if b1 == 1 || c2 == 1 {
        if c2 == 1 {
            if b2 == 1 {
                1 + c1.min(4)
            } else {
                1 + (5 * b2).min(c1)
            }
        } else {
            if b2 == 0 || c1 + c2 < 3 {
                2
            } else if c1 + c2 == 3 {
                3
            } else {
                let val = (5 * b2).min(c1 + c2 - 2);
                val + (c1 + c2 - 2 - val).min(c2 - 2).min(3) + 2
            }
        }
    } else {
        let val = (5 * b1).min(c2);
        val + (5 * b2).min(c1 + c2 - val)
    };

    writeln!(out, "{}", ret_a.min(ret_b)).unwrap();
}

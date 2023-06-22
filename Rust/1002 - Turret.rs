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

    let t = scan.token::<usize>();

    for _ in 0..t {
        let (x1, y1, r1, x2, y2, r2) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        let dist = (x1 - x2).pow(2) + (y1 - y2).pow(2);
        let radius_inner = (r1 - r2) * (r1 - r2);
        let radius_outer = (r1 + r2) * (r1 + r2);

        if x1 == x2 && y1 == y2 && r1 == r2 && r1 != 0 {
            writeln!(out, "-1").unwrap();
        } else if dist == radius_inner
            || dist == radius_outer
            || (x1 == x2 && y1 == y2 && r1 == r2 && r1 == 0)
        {
            writeln!(out, "1").unwrap();
        } else if dist > radius_inner && dist < radius_outer {
            writeln!(out, "2").unwrap();
        } else {
            writeln!(out, "0").unwrap();
        }
    }
}

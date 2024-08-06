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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let coordinate = scan.token::<i64>();
        let (a, b) = (scan.token::<f64>(), scan.token::<f64>());

        if coordinate == 1 {
            writeln!(
                out,
                "{:.6} {:.6}",
                a.hypot(b),
                if b.atan2(a) >= 0.0 {
                    b.atan2(a)
                } else {
                    b.atan2(a) + std::f64::consts::TAU
                }
            )
            .unwrap();
        } else {
            writeln!(out, "{:.6} {:.6}", a * b.cos(), a * b.sin()).unwrap();
        }
    }
}

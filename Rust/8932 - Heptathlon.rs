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
        let (a, b, c, d, e, f, g) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );

        let total = (9.23076 * (26.7 - a).powf(1.835)) as i64
            + (1.84523 * (b - 75.0).powf(1.348)) as i64
            + (56.0211 * (c - 1.5).powf(1.05)) as i64
            + (4.99087 * (42.5 - d).powf(1.81)) as i64
            + (0.188807 * (e - 210.0).powf(1.41)) as i64
            + (15.9803 * (f - 3.8).powf(1.04)) as i64
            + (0.11193 * (254.0 - g).powf(1.88)) as i64;

        writeln!(out, "{total}").unwrap();
    }
}

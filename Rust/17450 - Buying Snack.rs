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

    let (price_s, weight_s) = (scan.token::<f64>(), scan.token::<f64>());
    let (price_n, weight_n) = (scan.token::<f64>(), scan.token::<f64>());
    let (price_u, weight_u) = (scan.token::<f64>(), scan.token::<f64>());

    let mut ret = Vec::new();
    ret.push(((10.0 * weight_s) / if 10.0 * price_s >= 5000.0 { 10.0 * price_s - 500.0 } else { 10.0 * price_s }, "S"));
    ret.push(((10.0 * weight_n) / if 10.0 * price_n >= 5000.0 { 10.0 * price_n - 500.0 } else { 10.0 * price_n }, "N"));
    ret.push(((10.0 * weight_u) / if 10.0 * price_u >= 5000.0 { 10.0 * price_u - 500.0 } else { 10.0 * price_u }, "U"));

    ret.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    writeln!(out, "{}", ret[0].1).unwrap();
}

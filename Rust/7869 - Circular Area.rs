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

    let (x1, y1, r1, x2, y2, r2) = (
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );
    let d = (x1 - x2).hypot(y1 - y2);

    if r1 + r2 <= d {
        writeln!(out, "0.000").unwrap();
        return;
    }

    if (r1 - r2).abs() >= d {
        writeln!(out, "{:.3}", std::f64::consts::PI * r1.min(r2) * r1.min(r2)).unwrap();
        return;
    }

    let theta1 = 2.0 * ((r1 * r1 + d * d - r2 * r2) / (2.0 * r1 * d)).acos();
    let theta2 = 2.0 * ((r2 * r2 + d * d - r1 * r1) / (2.0 * r2 * d)).acos();
    let area1 = 0.5 * theta1 * r1 * r1 - 0.5 * r1 * r1 * theta1.sin();
    let area2 = 0.5 * theta2 * r2 * r2 - 0.5 * r2 * r2 * theta2.sin();

    writeln!(out, "{:.3}", area1 + area2).unwrap();
}

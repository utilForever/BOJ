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

    let n = scan.token::<usize>();
    let mut points = vec![(0.0, 0.0); n];

    for i in 0..n {
        points[i] = (scan.token::<f64>(), scan.token::<f64>());
    }

    points.push(points[0]);

    let mut inertia = 0.0;

    // The moment of inertia of a polygon about the z-axis is given by the formula:
    // I = integral(integral((x^2 + y^2) dm)) dA
    //   = ∑(x1y2 - x2y1)(x1^2 + x1x2 + x2^2 + y1^2 + y1y2 + y2^2) / 12
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];

        let cross = x1 * y2 - x2 * y1;
        let term = x1 * x1 + x1 * x2 + x2 * x2 + y1 * y1 + y1 * y2 + y2 * y2;
        inertia += cross * term;
    }

    inertia /= 12.0;
    inertia = inertia.abs();

    writeln!(out, "{:.1}", (inertia * 10.0).round() / 10.0).unwrap();
}

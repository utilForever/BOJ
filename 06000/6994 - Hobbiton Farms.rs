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

// Reference: https://www.geeksforgeeks.org/check-if-any-point-overlaps-the-given-circle-and-rectangle/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let check_overlap = |r: f64, xc: f64, yc: f64, x1: f64, y1: f64, x2: f64, y2: f64| {
        let nearest_x = x1.max(xc.min(x2));
        let nearest_y = y1.max(yc.min(y2));

        let dx = nearest_x - xc;
        let dy = nearest_y - yc;

        dx * dx + dy * dy <= r * r
    };

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (r, xc, yc, xl, yl, xu, yu) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );

        write!(out, "The given circle and rectangle ").unwrap();
        writeln!(
            out,
            "{}.",
            if check_overlap(r, xc, yc, xl, yl, xu, yu) {
                "overlap"
            } else {
                "do not overlap"
            }
        )
        .unwrap();
    }
}

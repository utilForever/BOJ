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

    let mut points = vec![(0, 0); 3];

    for i in 0..3 {
        points[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let calculate_dist = |p1: (i64, i64), p2: (i64, i64)| -> f64 {
        let (x1, y1) = p1;
        let (x2, y2) = p2;

        (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt()
    };

    writeln!(
        out,
        "{:.6}",
        calculate_dist(points[0], points[2]) + calculate_dist(points[1], points[2]) * 2.0
    )
    .unwrap();
}

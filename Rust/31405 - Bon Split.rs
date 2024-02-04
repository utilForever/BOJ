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

#[derive(Clone, Copy, Default)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn area(p: Point, q: Point, r: Point) -> f64 {
        ((p.x * q.y + q.x * r.y + r.x * p.y) - (p.y * q.x + q.y * r.x + r.y * p.x)).abs() as f64
            / 2.0
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![Point::default(); n];

    for i in 0..n {
        points[i] = Point::new(scan.token::<i64>(), scan.token::<i64>());
    }

    let mut areas = Vec::new();
    areas.push(0.0);

    for i in 1..n - 1 {
        areas.push(Point::area(points[0], points[i], points[i + 1]) + areas[i - 1]);
    }

    let area_half = areas.last().unwrap() / 2.0;
    let mut idx = 0;

    while areas[idx] <= area_half {
        idx += 1;
    }

    let diff = areas[idx] - areas[idx - 1];
    let ratio = (area_half - areas[idx - 1]) / diff;

    writeln!(out, "YES").unwrap();
    writeln!(out, "1 0").unwrap();
    writeln!(out, "{} {:.12}", idx + 1, ratio).unwrap();
}

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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn is_point_on_segment(p: Point, a: Point, b: Point) -> bool {
    let cross = (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
    let dot = (p.x - a.x) * (b.x - a.x) + (p.y - a.y) * (b.y - a.y);
    let squared_length_ab = (b.x - a.x).powi(2) + (b.y - a.y).powi(2);

    if cross.abs() > 1e-10 || dot < 0.0 || dot > squared_length_ab {
        false
    } else {
        true
    }
}

fn is_point_inside_polygon(point: Point, polygon: &[Point]) -> bool {
    let n = polygon.len();
    let mut cnt_intersect = 0;

    for i in 0..n {
        let a = polygon[i];
        let b = polygon[(i + 1) % n];

        if is_point_on_segment(point, a, b) {
            return true;
        }

        if (a.y > point.y) != (b.y > point.y) {
            let x_intersect = (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x;

            if x_intersect > point.x {
                cnt_intersect += 1;
            }
        }
    }

    cnt_intersect % 2 == 1
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut polygon = vec![Point::default(); n];

    for i in 0..n {
        polygon[i] = Point {
            x: scan.token::<f64>(),
            y: scan.token::<f64>(),
        };
    }

    for _ in 0..3 {
        let point = Point {
            x: scan.token::<f64>(),
            y: scan.token::<f64>(),
        };

        writeln!(
            out,
            "{}",
            if is_point_inside_polygon(point, &polygon) {
                1
            } else {
                0
            }
        )
        .unwrap();
    }
}

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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const EPS: f64 = 1e-3;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
    
    fn cross(self, other: Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

fn orientation(p: Point, q: Point, r: Point) -> f64 {
    (q.sub(p)).cross(r.sub(p))
}

fn contains(outer: &[Point; 4], inner: &[Point; 4]) -> bool {
    for i in 0..4 {
        let a = outer[i];
        let b = outer[(i + 1) % 4];

        for &pt in inner.iter() {
            if orientation(a, b, pt) < -EPS {
                return false;
            }
        }
    }

    true
}

fn split(p: Point, q: Point, a: &[Point; 4], b: &[Point; 4]) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if orientation(p, q, a[i]) * orientation(p, q, b[j]) > 0.0 {
                return false;
            }
        }
    }

    true
}

fn intersects(a: &[Point; 4], b: &[Point; 4]) -> bool {
    for i in 0..4 {
        if split(a[i], a[(i + 1) % 4], a, b) {
            return false;
        }

        if split(b[i], b[(i + 1) % 4], a, b) {
            return false;
        }
    }

    true
}

fn square_union(squares: &Vec<[Point; 4]>, min: Point, max: Point) -> f64 {
    let area = (max.x - min.x) * (max.y - min.y);

    if max.x - min.x < EPS || max.y - min.y < EPS {
        return 0.0;
    }

    let square = [
        Point::new(min.x, min.y),
        Point::new(max.x, min.y),
        Point::new(max.x, max.y),
        Point::new(min.x, max.y),
    ];

    if squares.iter().any(|s| contains(s, &square)) {
        return area;
    }

    if !squares.iter().any(|s| intersects(s, &square)) {
        return 0.0;
    }

    let mid = Point::new((min.x + max.x) / 2.0, (min.y + max.y) / 2.0);
    let area1 = square_union(&squares, min, mid);
    let area2 = square_union(&squares, Point::new(mid.x, min.y), Point::new(max.x, mid.y));
    let area3 = square_union(&squares, Point::new(min.x, mid.y), Point::new(mid.x, max.y));
    let area4 = square_union(&squares, mid, max);

    return area1 + area2 + area3 + area4;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut squares: Vec<[Point; 4]> = Vec::with_capacity(n);

    for _ in 0..n {
        let (center, vertex) = (
            Point::new(scan.token::<f64>(), scan.token::<f64>()),
            Point::new(scan.token::<f64>(), scan.token::<f64>()),
        );
        let p = Point::new(vertex.x - center.x, vertex.y - center.y);
        let q = Point::new(-p.y, p.x);
        let poly = [
            Point::new(center.x + p.x, center.y + p.y),
            Point::new(center.x + q.x, center.y + q.y),
            Point::new(center.x - p.x, center.y - p.y),
            Point::new(center.x - q.x, center.y - q.y),
        ];

        squares.push(poly);
    }

    let mut min = Point::new(f64::MAX, f64::MAX);
    let mut max = Point::new(f64::MIN, f64::MIN);

    for i in 0..n {
        for j in 0..4 {
            min.x = min.x.min(squares[i][j].x);
            min.y = min.y.min(squares[i][j].y);
            max.x = max.x.max(squares[i][j].x);
            max.y = max.y.max(squares[i][j].y);
        }
    }

    writeln!(out, "{}", square_union(&squares, min, max).round() as i64).unwrap();
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

fn cross(a: Point, b: Point, c: Point) -> i64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn convex_hull(mut points: Vec<Point>) -> Vec<Point> {
    if points.len() <= 1 {
        return points;
    }

    points.sort_by(|a, b| {
        if a.x != b.x {
            a.x.cmp(&b.x)
        } else {
            a.y.cmp(&b.y)
        }
    });

    let mut lower = Vec::new();

    for &point in points.iter() {
        while lower.len() >= 2 && cross(lower[lower.len() - 2], lower[lower.len() - 1], point) <= 0
        {
            lower.pop();
        }

        lower.push(point);
    }

    let mut upper = Vec::new();

    for &point in points.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len() - 2], upper[upper.len() - 1], point) <= 0
        {
            upper.pop();
        }

        upper.push(point);
    }

    lower.pop();
    upper.pop();

    lower.extend(upper);
    lower
}

fn minkowski_sum(a: &[Point], b: &[Point]) -> Vec<Point> {
    let n = a.len();
    let m = b.len();
    let mut c = Vec::with_capacity(n + m);

    let mut i = 0;
    let mut j = 0;

    let mut da = Vec::with_capacity(n);
    let mut db = Vec::with_capacity(m);

    for k in 0..n {
        da.push(Point::new(
            a[(k + 1) % n].x - a[k].x,
            a[(k + 1) % n].y - a[k].y,
        ));
    }

    for k in 0..m {
        db.push(Point::new(
            b[(k + 1) % m].x - b[k].x,
            b[(k + 1) % m].y - b[k].y,
        ));
    }

    c.push(Point::new(a[0].x + b[0].x, a[0].y + b[0].y));

    while i < da.len() || j < db.len() {
        let d: Point;

        if j == db.len() || (i < da.len() && cross(Point::new(0, 0), da[i], db[j]) >= 0) {
            d = da[i];
            i += 1;
        } else {
            d = db[j];
            j += 1;
        }

        c.push(Point::new(
            c.last().unwrap().x + d.x,
            c.last().unwrap().y + d.y,
        ));
    }

    c.pop();
    c
}

fn polygon_area(poly: &[Point]) -> i64 {
    let n = poly.len();
    let mut area = 0;

    for i in 0..n {
        let j = (i + 1) % n;
        area += poly[i].x * poly[j].y - poly[j].x * poly[i].y;
    }

    area.abs()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut polygon_a = Vec::with_capacity(n);

    for _ in 0..n {
        let x = scan.token::<i64>();
        let y = scan.token::<i64>();
        polygon_a.push(Point::new(x, y));
    }

    let mut polygon_b = Vec::with_capacity(m);

    for _ in 0..m {
        let x = scan.token::<i64>();
        let y = scan.token::<i64>();
        polygon_b.push(Point::new(x, y));
    }

    let convex_a = convex_hull(polygon_a);
    let convex_b = convex_hull(polygon_b);
    let minkowski = minkowski_sum(&convex_a, &convex_b);
    let area = polygon_area(&minkowski);

    writeln!(out, "{area}").unwrap();
}

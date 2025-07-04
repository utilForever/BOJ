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

const EPS: f64 = 1e-9;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl Point {
    #[inline(always)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }

    #[inline(always)]
    pub fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    #[inline(always)]
    pub fn cross2(&self, p1: &Point, p2: &Point) -> f64 {
        (*p1 - *self).cross(&(*p2 - *self))
    }

    #[inline(always)]
    pub fn dist(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[inline(always)]
    pub fn dist2(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    #[inline(always)]
    pub fn normalize(&self) -> Point {
        let d = self.dist();

        Point {
            x: self.x / d,
            y: self.y / d,
        }
    }

    #[inline(always)]
    pub fn perp(&self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    #[inline(always)]
    pub fn same(&self, other: &Point) -> bool {
        (self.x - other.x).abs() < EPS && (self.y - other.y).abs() < EPS
    }
}

impl Point {
    fn intersect_line(a: &Point, b: &Point, c: &Point, d: &Point) -> Point {
        let cross = (*b - *a).cross(&(*d - *c));

        if cross.abs() < EPS {
            return Point::new(0.0, 0.0);
        }

        let p = c.cross2(b, d);
        let q = c.cross2(d, a);

        (*a * p + *b * q) / cross
    }

    fn is_inside(p: Point, a: Point, b: Point) -> bool {
        (b - a).cross(&(p - a)) >= -EPS
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, scalar: f64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Point;
    fn div(self, rhs: f64) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

fn clip_polygon(points: &Vec<Point>, clipper: &[Point]) -> Vec<Point> {
    let mut clipped = points.clone();

    for i in 0..clipper.len() {
        let mut clipped_new = Vec::new();
        let a = clipper[i];
        let b = clipper[(i + 1) % clipper.len()];

        for j in 0..clipped.len() {
            let curr = clipped[j];
            let prev = clipped[(j + clipped.len() - 1) % clipped.len()];
            let inside_curr = Point::is_inside(curr, a, b);
            let inside_prev = Point::is_inside(prev, a, b);

            if inside_curr {
                if !inside_prev {
                    clipped_new.push(Point::intersect_line(&prev, &curr, &a, &b));
                }

                clipped_new.push(curr);
            } else if inside_prev {
                clipped_new.push(Point::intersect_line(&prev, &curr, &a, &b));
            }
        }

        clipped = clipped_new;
    }

    clipped
}

fn polygon_area(poly: &[Point]) -> f64 {
    if poly.len() < 3 {
        return 0.0;
    }

    let mut ret = 0.0;

    for i in 0..poly.len() {
        let j = (i + 1) % poly.len();
        ret += poly[i].cross(&poly[j]);
    }

    ret.abs() * 0.5
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut polygon1 = Vec::with_capacity(n);
    let mut polygon2 = Vec::with_capacity(m);

    for _ in 0..n {
        polygon1.push(Point::new(
            scan.token::<f64>(),
            scan.token::<f64>(),
        ));
    }

    for _ in 0..m {
        polygon2.push(Point::new(
            scan.token::<f64>(),
            scan.token::<f64>(),
        ));
    }

    let intersect = clip_polygon(&polygon1, &polygon2);
    let area = polygon_area(&intersect);

    writeln!(out, "{:.12}", area).unwrap();
}

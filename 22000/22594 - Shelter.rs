use io::Write;
use std::{
    io,
    ops::{Add, Div, Mul, Sub},
    str,
};

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

const EPS: f64 = 1e-9;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    #[inline(always)]
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    #[inline(always)]
    fn cross2(&self, p1: &Point, p2: &Point) -> f64 {
        (*p1 - *self).cross(&(*p2 - *self))
    }

    #[inline(always)]
    fn rotate90(&self) -> Self {
        Point::new(-self.y, self.x)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

fn sign(x: f64) -> i64 {
    if x < -EPS {
        -1
    } else if x > EPS {
        1
    } else {
        0
    }
}

fn intersect_lines(a: &Point, b: &Point, c: &Point, d: &Point) -> Point {
    let cross = (*b - *a).cross(&(*d - *c));

    // Check if lines are parallel (cross product is zero)
    if cross.abs() < EPS {
        return Point { x: 0.0, y: 0.0 };
    }

    // Calculate the intersection point using a parameteric approach and cross products
    let p = c.cross2(b, d);
    let q = c.cross2(d, a);

    // Return the computed intersection point
    (*a * p + *b * q) / cross
}

// "Cuts" the polygon `points` with a line from p1 to p2,
// returning the portion on the left side.
fn convex_cut(points: Vec<Point>, p1: Point, p2: Point) -> Vec<Point> {
    let n = points.len();
    let mut ret = Vec::new();

    for i in 0..n {
        let d1 = sign((p2 - p1).cross(&(points[i] - p1)));
        let d2 = sign((p2 - p1).cross(&(points[(i + 1) % n] - p1)));

        // If current point is on the left side (or on the line), include it
        if d1 >= 0 {
            ret.push(points[i]);
        }

        // If there is an intersection between [current, next], compute and include it
        if d1 * d2 < 0 {
            ret.push(intersect_lines(&p1, &p2, &points[i], &points[(i + 1) % n]));
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (m, n) = (scan.token::<usize>(), scan.token::<usize>());
    let mut towns = Vec::with_capacity(m);
    let mut shelters = Vec::with_capacity(n);

    for _ in 0..m {
        let (x, y) = (scan.token::<f64>(), scan.token::<f64>());
        towns.push(Point::new(x, y));
    }

    for _ in 0..n {
        let (x, y) = (scan.token::<f64>(), scan.token::<f64>());
        shelters.push(Point::new(x, y));
    }

    let mut area = 0.0;
    let mut ret = 0.0;

    for i in 0..n {
        let mut towns_copy = towns.clone();

        for j in 0..n {
            if i == j {
                continue;
            }

            let p1 = (shelters[i] + shelters[j]) * 0.5;
            let p2 = p1 + (shelters[j] - shelters[i]).rotate90();

            towns_copy = convex_cut(towns_copy, p1, p2);
        }

        for j in 0..towns_copy.len() {
            let p1 = towns_copy[j] - shelters[i];
            let p2 = towns_copy[(j + 1) % towns_copy.len()] - shelters[i];

            area += p1.cross(&p2);
            ret += ((p1.x * p1.x + p2.x * p2.x) * (p1.x + p2.x)) * (p2.y - p1.y)
                - ((p1.y * p1.y + p2.y * p2.y) * (p1.y + p2.y)) * (p2.x - p1.x);
        }
    }

    writeln!(out, "{:.9}", ret / (area * 6.0)).unwrap();
}

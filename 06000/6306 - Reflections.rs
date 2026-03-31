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

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

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
    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

const EPS: f64 = 1e-9;

fn raycast(
    circles: &Vec<Circle>,
    origin: Point,
    direction: Point,
    idx_ignore: Option<usize>,
) -> Option<(usize, Point)> {
    let a = direction.norm2();

    for (idx, circle) in circles.iter().enumerate() {
        if Some(idx) == idx_ignore {
            continue;
        }

        let rel = origin.sub(circle.center);
        let rr = circle.radius * circle.radius;

        if (rel.norm2() - rr).abs() <= EPS && rel.dot(&direction) < -EPS {
            return Some((idx, origin));
        }
    }

    let mut ret_t = f64::INFINITY;
    let mut ret_idx = None;

    for (idx, circle) in circles.iter().enumerate() {
        if Some(idx) == idx_ignore {
            continue;
        }

        let rel = origin.sub(circle.center);
        let b = 2.0 * rel.dot(&direction);
        let c = rel.norm2() - circle.radius * circle.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < -EPS {
            continue;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let t = if t1 > EPS {
            t1
        } else if t2 > EPS {
            t2
        } else {
            continue;
        };

        if t < ret_t {
            ret_t = t;
            ret_idx = Some(idx);
        }
    }

    ret_idx.map(|idx| (idx, origin.add(direction.mul(ret_t))))
}

fn reflect(circle: &Circle, hit: Point, direction: Point) -> Point {
    let n = hit.sub(circle.center);
    direction.sub(n.mul(2.0 * direction.dot(&n) / n.norm2()))
}

fn simulate(circles: &Vec<Circle>, mut origin: Point, mut direction: Point) -> (Vec<usize>, bool) {
    let mut hits = Vec::new();
    let mut idx_ignore = None;

    for _ in 0..11 {
        let Some(hit) = raycast(circles, origin, direction, idx_ignore) else {
            return (hits, false);
        };

        hits.push(hit.0 + 1);
        origin = hit.1;
        direction = reflect(&circles[hit.0], hit.1, direction);
        idx_ignore = Some(hit.0);
    }

    (hits, true)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut t = 1;

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut circles = Vec::with_capacity(n);

        for _ in 0..n {
            let (x, y, r) = (
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
            );
            circles.push(Circle::new(Point::new(x, y), r));
        }

        let (x, y, dx, dy) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
        let (hits, is_overflow) = simulate(&circles, Point::new(x, y), Point::new(dx, dy));

        writeln!(out, "Scene {t}").unwrap();

        if is_overflow {
            for i in 0..10 {
                write!(out, "{} ", hits[i]).unwrap();
            }

            writeln!(out, "...").unwrap();
        } else {
            for hit in hits.iter() {
                write!(out, "{hit} ").unwrap();
            }

            writeln!(out, "inf").unwrap();
        }

        writeln!(out).unwrap();

        t += 1;
    }
}

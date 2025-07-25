use io::Write;
use std::collections::VecDeque;
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

struct Rng([u64; 4]);

impl Rng {
    fn split_mix(v: u64) -> u64 {
        let mut z = v.wrapping_add(0x9e3779b97f4a7c15);

        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    fn new() -> Self {
        let mut seed = 0;
        unsafe { std::arch::x86_64::_rdrand64_step(&mut seed) };

        let mut prev = seed;

        Self(std::array::from_fn(|_| {
            prev = Self::split_mix(prev);
            prev
        }))
    }

    fn next(&mut self, n: u64) -> u64 {
        let [x, y, z, c] = &mut self.0;
        let t = x.wrapping_shl(58) + *c;

        *c = *x >> 6;
        *x = x.wrapping_add(t);

        if *x < t {
            *c += 1;
        }

        *z = z.wrapping_mul(6906969069).wrapping_add(1234567);
        *y ^= y.wrapping_shl(13);
        *y ^= *y >> 17;
        *y ^= y.wrapping_shl(43);

        let base = x.wrapping_add(*y).wrapping_add(*z);
        ((base as u128 * n as u128) >> 64) as u64
    }
}

#[derive(Default, Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    pub fn welzl(points: &Vec<Point>) -> Self {
        let mut points = points.clone();
        let n = points.len();

        Circle::welzl_internal(&mut points, Vec::new(), n)
    }

    fn welzl_internal(points: &mut [Point], mut r: Vec<Point>, n: usize) -> Self {
        if n == 0 || r.len() == 3 {
            return Self::minimum_enclosing_circle(&r);
        }

        let idx = Rng::new().next(n as u64) as usize;
        let p = points[idx];

        points.swap(idx, n - 1);

        let circle = Self::welzl_internal(points, r.clone(), n - 1);

        if circle.is_inside(&p) {
            return circle;
        }

        r.push(p);

        Self::welzl_internal(points, r, n - 1)
    }

    fn minimum_enclosing_circle(points: &[Point]) -> Self {
        let n = points.len();

        assert!(n <= 3);

        if n == 0 {
            Self {
                center: Point { x: 0.0, y: 0.0 },
                radius: 0.0,
            }
        } else if n == 1 {
            Self {
                center: points[0],
                radius: 0.0,
            }
        } else if n == 2 {
            Self::from_two_points(points[0], points[1])
        } else {
            for i in 0..3 {
                for j in i + 1..3 {
                    let circle = Self::from_two_points(points[i], points[j]);

                    if circle.is_valid(points) {
                        return circle;
                    }
                }
            }

            Self::from_three_points(points[0], points[1], points[2])
        }
    }

    fn from_two_points(a: Point, b: Point) -> Self {
        let center = Point {
            x: (a.x + b.x) / 2.0,
            y: (a.y + b.y) / 2.0,
        };

        Self {
            center,
            radius: a.dist(&b) / 2.0,
        }
    }

    fn from_three_points(a: Point, b: Point, c: Point) -> Self {
        let mut center = Circle::center(b.x - a.x, b.y - a.y, c.x - a.x, c.y - a.y);
        center.x += a.x;
        center.y += a.y;

        Self {
            center,
            radius: center.dist(&a),
        }
    }

    fn is_valid(&self, points: &[Point]) -> bool {
        points.iter().all(|p| self.is_inside(p))
    }

    fn is_inside(&self, point: &Point) -> bool {
        self.center.dist(point) <= self.radius
    }

    fn center(bx: f64, by: f64, cx: f64, cy: f64) -> Point {
        let b = bx * bx + by * by;
        let c = cx * cx + cy * cy;
        let d = bx * cy - by * cx;

        Point {
            x: (cy * b - by * c) / (2.0 * d),
            y: (bx * c - cx * b) / (2.0 * d),
        }
    }
}

const EPS: f64 = 1e-9;

fn area_lens(d: f64, r: f64) -> f64 {
    let alpha = (d / (2.0 * r)).clamp(-1.0, 1.0).acos();
    2.0 * r * r * alpha - 0.5 * d * (4.0 * r * r - d * d).sqrt()
}

fn intersect_circle(a: Point, b: Point, r: f64) -> Option<(Point, Point)> {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let d2 = dx * dx + dy * dy;
    let d = d2.sqrt();

    if d > 2.0 * r + EPS {
        return None;
    }

    let mid = Point::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0);
    let h = (r * r - d2 * 0.25).sqrt();

    let u = Point::new(dx / d, dy / d);
    let v = Point::new(-u.y, u.x);

    Some((
        Point::new(mid.x + v.x * h, mid.y + v.y * h),
        Point::new(mid.x - v.x * h, mid.y - v.y * h),
    ))
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, r) = (scan.token::<usize>(), scan.token::<f64>());
        let mut points = Vec::with_capacity(n);

        for _ in 0..n {
            let (x, y) = (scan.token::<f64>(), scan.token::<f64>());
            points.push(Point::new(x, y));
        }

        if n == 1 {
            writeln!(out, "{:.12}", std::f64::consts::PI * r * r).unwrap();
            continue;
        }

        if n == 2 {
            let d = points[0].dist(&points[1]);

            writeln!(
                out,
                "{:.12}",
                if d >= 2.0 * r - EPS {
                    0.0
                } else {
                    area_lens(d, r)
                }
            )
            .unwrap();
            continue;
        }

        let circle = Circle::welzl(&points);

        if circle.radius > r + EPS {
            writeln!(out, "{:.12}", 0.0).unwrap();
            continue;
        }

        let mut prev = std::iter::once(n - 1).chain(0..n - 1).collect::<Vec<_>>();
        let mut next = (1..n).chain(std::iter::once(0)).collect::<Vec<_>>();
        let mut alive = vec![true; n];
        let mut queue = (0..n).collect::<VecDeque<_>>();

        while let Some(b) = queue.pop_front() {
            if !alive[b] {
                continue;
            }

            let a = prev[b];
            let c = next[b];

            if a == c {
                break;
            }

            if let Some((p1, p2)) = intersect_circle(points[a], points[c], r) {
                if p1.dist(&points[b]) <= r + EPS && p2.dist(&points[b]) <= r + EPS {
                    alive[b] = false;
                    next[a] = c;
                    prev[c] = a;
                    queue.push_back(a);
                    queue.push_back(c);
                }
            } else {
                alive.iter_mut().for_each(|x| *x = false);
                break;
            }
        }

        let vertices_alive = (0..n).filter(|&i| alive[i]).collect::<Vec<_>>();

        if vertices_alive.is_empty() {
            writeln!(out, "{:.12}", 0.0).unwrap();
            continue;
        }

        if vertices_alive.len() == 1 {
            writeln!(out, "{:.12}", std::f64::consts::PI * r * r).unwrap();
            continue;
        }

        if vertices_alive.len() == 2 {
            let d = points[vertices_alive[0]].dist(&points[vertices_alive[1]]);

            writeln!(
                out,
                "{:.12}",
                if d >= 2.0 * r - EPS {
                    0.0
                } else {
                    area_lens(d, r)
                }
            )
            .unwrap();
            continue;
        }

        let m = vertices_alive.len();
        let mut points_area = Vec::with_capacity(m);

        for i in 0..m {
            let a = vertices_alive[i];
            let b = vertices_alive[(i + 1) % m];
            let (p1, p2) = intersect_circle(points[a], points[b], r).unwrap();
            let choosed = if (points[b] - points[a]).cross(&(p1 - points[a])) > 0.0 {
                p1
            } else {
                p2
            };

            points_area.push(choosed);
        }

        let mut ret = 0.0;

        for i in 0..m {
            let curr = points_area[i];
            let next = points_area[(i + 1) % m];

            ret += 0.5 * curr.cross(&next);

            let c = points[vertices_alive[(i + 1) % m]];
            let v1 = curr - c;
            let v2 = next - c;
            let mut theta = v1.cross(&v2).atan2(v1.dot(&v2));

            if theta < 0.0 {
                theta += std::f64::consts::TAU;
            }

            if theta > std::f64::consts::PI + EPS {
                theta = std::f64::consts::TAU - theta;
            }

            ret += 0.5 * r * r * (theta - theta.sin());
        }

        if ret.abs() < EPS {
            ret = 0.0;
        }

        writeln!(out, "{:.12}", ret).unwrap();
    }
}

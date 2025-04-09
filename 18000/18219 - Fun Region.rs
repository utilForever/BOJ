use std::io::{self, Write};
use std::ops::{Add, Div, Mul, Sub};
use std::str;

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

const EPS: f64 = 1e-6;

#[inline(always)]
fn sign(x: f64) -> i64 {
    if x < -EPS {
        -1
    } else if x > EPS {
        1
    } else {
        0
    }
}

#[derive(Debug, Default, Copy, Clone, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
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

    fn intersect_segment(a: &Point, b: &Point, c: &Point, d: &Point) -> Option<Point> {
        let oa = c.cross2(d, a);
        let ob = c.cross2(d, b);
        let oc = a.cross2(b, c);
        let od = a.cross2(b, d);

        if sign(oa) * sign(ob) <= 0 && sign(oc) * sign(od) <= 0 {
            Some((*a * ob - *b * oa) / (ob - oa))
        } else {
            None
        }
    }

    fn is_on_left(a: &Point, b: &Point, c: &Point) -> bool {
        a.cross2(b, c) > EPS
    }

    fn is_on_right(a: &Point, b: &Point, c: &Point) -> bool {
        a.cross2(b, c) < -EPS
    }

    fn is_on_segment(a: &Point, b: &Point, p: &Point) -> bool {
        p.cross2(a, b).abs() < EPS && (*a - *p).dot(&(*b - *p)) < EPS
    }

    fn is_parallel(a: &Point, b: &Point, c: &Point, d: &Point) -> bool {
        (*a - *b).cross(&(*c - *d)).abs() < EPS
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

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        sign(self.x - other.x) == 0 && sign(self.y - other.y) == 0
    }
}

#[derive(Clone)]
pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn len(&self) -> usize {
        self.points.len()
    }

    fn last(&self) -> &Point {
        self.points.last().unwrap()
    }

    fn is_inside(&self, p: &Point, strict: bool) -> bool {
        let n = self.len();
        let mut cnt = 0;

        for i in 0..n {
            let q = self.points[(i + 1) % n];

            if Point::is_on_segment(&self.points[i], &q, p) {
                return !strict;
            }

            let cond1 = if p.y < self.points[i].y { 1 } else { 0 };
            let cond2 = if p.y < q.y { 1 } else { 0 };

            if (cond1 - cond2) as f64 * p.cross2(&self.points[i], &q) > 0.0 {
                cnt += 1;
            }
        }

        cnt % 2 == 1
    }

    fn rotate_by_index(&self, start: usize) -> Polygon {
        let n = self.points.len();
        Polygon::new((0..n).map(|i| self.points[(start + i) % n]).collect())
    }

    fn clip_half_plane(&self, a: &Point, b: &Point) -> Polygon {
        let n = self.points.len();
        let mut idx_start = None;

        for i in 0..n {
            let next = (i + 1) % n;
            if Point::intersect_segment(&self.points[i], &self.points[next], a, b).is_some() {
                if Point::is_on_left(a, b, &self.points[i]) {
                    idx_start = Some(i);
                    break;
                } else if Point::is_on_left(a, b, &self.points[next]) {
                    idx_start = Some(next);
                    break;
                }
            }
        }

        if idx_start.is_none() {
            return self.clone();
        }

        let idx_start = idx_start.unwrap();
        let rotated = self.rotate_by_index(idx_start);

        let m = rotated.len();
        let mut flag_skip = false;
        let mut clipped = Vec::new();

        for i in 1..=m {
            let curr = rotated[i % m];
            let prev = rotated[(i + m - 1) % m];

            if Point::is_on_segment(a, b, &curr) {
                clipped.push(curr);
                continue;
            }

            if let Some(intersect_pt) = Point::intersect_segment(a, b, &prev, &curr) {
                clipped.push(intersect_pt);
                flag_skip = !Point::is_on_left(a, b, &curr);
            }

            if !flag_skip {
                clipped.push(curr);
            }
        }

        let mut ret: Vec<Point> = Vec::new();

        for point in clipped {
            if ret.is_empty() || !ret.last().unwrap().same(&point) {
                ret.push(point);
            }
        }

        Polygon::new(ret)
    }

    fn area(&self) -> f64 {
        let n = self.len();

        self.points.iter().enumerate().fold(0.0, |acc, (i, &p)| {
            let q = self.points[(i + 1) % n];
            acc + p.cross(&q) / 2.0
        })
    }
}

fn generate_half_plane_constraints(poly: &Polygon) -> Vec<Polygon> {
    let n = poly.len();
    let mut ret = Vec::new();

    for i in 0..n {
        let p = poly[i];
        let q = poly[(i + 1) % n];
        let r = poly[(i + 2) % n];

        if Point::is_on_right(&p, &q, &r) {
            let mut side = vec![q, r];
            let mut intersection_closest_dist = f64::MAX;
            let mut intersection_closest_point = Point::new(0.0, 0.0);

            for j in i + 1..i + n {
                let s = poly[j % n];
                let t = poly[(j - 1) % n];

                if !Point::is_on_left(&p, &q, &s) && !Point::is_on_left(&p, &q, &t) {
                    continue;
                }

                if Point::is_parallel(&p, &q, &s, &t) {
                    continue;
                }

                let p_intersect = Point::intersect_line(&p, &q, &s, &t);

                if !Point::is_on_segment(&t, &s, &p_intersect) {
                    continue;
                }

                let intersection_dist = (q - p).dot(&(p_intersect - p));

                if intersection_dist > (q - p).dist2() + EPS
                    && intersection_dist < intersection_closest_dist
                {
                    intersection_closest_dist = intersection_dist;
                    intersection_closest_point = p_intersect;
                }
            }

            for j in i + 3..3 * n {
                let s = poly[j % n];
                let t = poly[(j - 1) % n];

                if Point::is_on_segment(&s, &t, &intersection_closest_point) {
                    side.push(intersection_closest_point);
                    break;
                } else {
                    side.push(s);
                }
            }

            ret.push(Polygon::new(side));
        }
    }

    ret
}

impl std::ops::Index<usize> for Polygon {
    type Output = Point;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

fn clip_by_constraint(poly: &Polygon, side: &Polygon) -> Option<Polygon> {
    let a = side[0];
    let b = *side.last();

    let mut found_left = false;
    let mut candidate_right = Point::new(0.0, 0.0);

    for &point in poly.points.iter() {
        if Point::is_on_left(&a, &b, &point) {
            found_left = true;
            break;
        }

        if Point::is_on_right(&a, &b, &point) {
            candidate_right = point;
        }
    }

    if found_left {
        Some(poly.clip_half_plane(&a, &b))
    } else if side.is_inside(&candidate_right, false) {
        None
    } else {
        Some(poly.clone())
    }
}

fn compute_fun_region(mut poly: Polygon, constraints: &Vec<Polygon>) -> Option<Polygon> {
    for constraint in constraints {
        match clip_by_constraint(&poly, constraint) {
            Some(poly_new) => poly = poly_new,
            None => return None,
        }
    }

    Some(poly)
}

// Reference: ICPC 2019 Asia Yokohama Regional Editorial
// Thanks for stasio6 to provide the important idea of the solution
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![Point::default(); n];

    for i in 0..n {
        let x = scan.token::<f64>();
        let y = scan.token::<f64>();

        points[i] = Point::new(x, y);
    }

    let polygon = Polygon::new(points);
    let constraints = generate_half_plane_constraints(&polygon);

    match compute_fun_region(polygon.clone(), &constraints) {
        Some(fun_region) => writeln!(out, "{:.12}", fun_region.area()).unwrap(),
        None => writeln!(out, "0.0").unwrap(),
    }
}

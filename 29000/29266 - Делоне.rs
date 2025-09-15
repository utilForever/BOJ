use io::Write;
use std::{collections::HashMap, io, str};

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

pub const EPSILON: f64 = f64::EPSILON * 2.0;
pub const INVALID_INDEX: usize = usize::max_value();

pub trait Coord: Sync + Send + Clone {
    fn from_xy(x: f64, y: f64) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;

    fn magnitude2(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y()
    }
}

pub trait Vector<C: Coord> {
    fn vector(p: &C, q: &C) -> C {
        C::from_xy(q.x() - p.x(), q.y() - p.y())
    }

    fn determinant(p: &C, q: &C) -> f64 {
        p.x() * q.y() - p.y() * q.x()
    }

    fn dist2(p: &C, q: &C) -> f64 {
        let d = Self::vector(p, q);

        d.x() * d.x() + d.y() * d.y()
    }

    fn equals(p: &C, q: &C) -> bool {
        (p.x() - q.x()).abs() <= EPSILON && (p.y() - q.y()).abs() <= EPSILON
    }

    fn equals_with_span(p: &C, q: &C, span: f64) -> bool {
        let dist = Self::dist2(p, q) / span;
        dist < 1e-20
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

impl Coord for Point {
    #[inline(always)]
    fn from_xy(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    #[inline(always)]
    fn x(&self) -> f64 {
        self.x
    }

    #[inline(always)]
    fn y(&self) -> f64 {
        self.y
    }
}

impl Vector<Point> for Point {}

fn in_circle<C: Coord + Vector<C>>(p: &C, a: &C, b: &C, c: &C) -> bool {
    let d = C::vector(p, a);
    let e = C::vector(p, b);
    let f = C::vector(p, c);

    let ap = d.x() * d.x() + d.y() * d.y();
    let bp = e.x() * e.x() + e.y() * e.y();
    let cp = f.x() * f.x() + f.y() * f.y();

    let res = d.x() * (e.y() * cp - bp * f.y()) - d.y() * (e.x() * cp - bp * f.x())
        + ap * (e.x() * f.y() - e.y() * f.x());

    res < 0.0
}

#[inline]
fn inside_circumcircle(p: &Point, a: &Point, b: &Point, c: &Point) -> bool {
    let (aa, mut bb, mut cc) = (a, b, c);

    if (*b - *a).cross(&(*c - *a)) > 0.0 {
        std::mem::swap(&mut bb, &mut cc);
    }

    in_circle::<Point>(p, aa, bb, cc)
}

#[inline]
fn add_edge(map: &mut HashMap<(usize, usize), (usize, usize)>, a: usize, b: usize, c: usize) {
    let key = if a < b { (a, b) } else { (b, a) };

    if let Some(entry) = map.get_mut(&key) {
        if entry.0 == INVALID_INDEX {
            entry.0 = c;
        } else if entry.1 == INVALID_INDEX {
            entry.1 = c;
        } else {
            // Do nothing
        }
    } else {
        map.insert(key, (c, INVALID_INDEX));
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = UnsafeScanner::new(io::BufReader::new(stdin.lock()));
    let mut out = io::BufWriter::new(io::stdout());

    let t = scanner.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scanner.token::<usize>(), scanner.token::<usize>());
        let mut points = Vec::with_capacity(n);
        for _ in 0..n {
            let (x, y) = (scanner.token::<f64>(), scanner.token::<f64>());
            points.push(Point::new(x, y));
        }

        let mut triangles = Vec::with_capacity(m);

        for _ in 0..m {
            let (a, b, c) = (
                scanner.token::<usize>() - 1,
                scanner.token::<usize>() - 1,
                scanner.token::<usize>() - 1,
            );
            triangles.push((a, b, c));
        }

        let mut edge_map: HashMap<(usize, usize), (usize, usize)> = HashMap::with_capacity(3 * m);

        for &(a, b, c) in triangles.iter() {
            add_edge(&mut edge_map, a, b, c);
            add_edge(&mut edge_map, b, c, a);
            add_edge(&mut edge_map, c, a, b);
        }

        let mut check = true;

        for (&(u, v), &(w, z)) in &edge_map {
            if w == INVALID_INDEX || z == INVALID_INDEX {
                continue;
            }

            let cond1 = inside_circumcircle(&points[z], &points[u], &points[v], &points[w]);
            let cond2 = inside_circumcircle(&points[w], &points[u], &points[v], &points[z]);

            if cond1 || cond2 {
                check = false;
                break;
            }
        }

        writeln!(out, "{}", if check { "YES" } else { "NO" }).unwrap();
    }
}

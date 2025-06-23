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

const EPS: f64 = 1e-9;

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

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    #[inline(always)]
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    #[inline(always)]
    pub fn cross2(&self, p1: &Point, p2: &Point) -> f64 {
        (*p1 - *self).cross(&(*p2 - *self))
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

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        sign(self.x - other.x) == 0 && sign(self.y - other.y) == 0
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.y * other.x == self.x * other.y {
            (other.x * other.x + other.y * other.y)
                .partial_cmp(&(self.x * self.x + self.y * self.y))
        } else {
            (self.y * other.x).partial_cmp(&(self.x * other.y))
        }
    }
}

fn remain_visible(
    points: &Vec<Point>,
    rev: &Vec<usize>,
    idx_new: usize,
    idx_stack: usize,
    n: usize,
) -> bool {
    let prev = if idx_stack == 0 { n - 1 } else { idx_stack - 1 };
    let next = if idx_stack == n - 1 { 0 } else { idx_stack + 1 };

    if rev[idx_new] > rev[prev] && points[prev].cross2(&points[idx_new], &points[idx_stack]) >= 0.0
    {
        // Case 1: idx_new is after prev in angular order and new prev is ccw: hidden
        false
    } else if rev[idx_new] > rev[next]
        && points[next].cross2(&points[idx_new], &points[idx_stack]) >= 0.0
    {
        // Case 2: Symmetric with next neighbor
        false
    } else {
        true
    }
}

fn candidate_visible(
    points: &Vec<Point>,
    rev: &Vec<usize>,
    idx_candidate: usize,
    idx_stack: usize,
    n: usize,
) -> bool {
    let prev = if idx_stack == 0 { n - 1 } else { idx_stack - 1 };
    let next = if idx_stack == n - 1 { 0 } else { idx_stack + 1 };

    if rev[idx_candidate] < rev[prev]
        && points[idx_stack].cross2(&points[idx_candidate], &points[prev]) >= 0.0
    {
        false
    } else if rev[idx_candidate] < rev[next]
        && points[idx_stack].cross2(&points[idx_candidate], &points[next]) >= 0.0
    {
        false
    } else {
        true
    }
}

// Reference: sorohue's code
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = Vec::with_capacity(n + 1);

    for _ in 0..n {
        points.push(Point::new(scan.token::<f64>(), scan.token::<f64>()));
    }

    points.push(points[0]);

    let mut points_sorted = points[..n]
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<Vec<_>>();

    points_sorted.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    points_sorted.push(points_sorted[0]);

    let mut rev = vec![0; n];

    for i in 0..n {
        rev[points_sorted[i].1] = i;
    }

    let mut stack = Vec::new();

    for i in 0..n {
        // Remove previously visible vertices that became hidden by current
        while let Some(&idx) = stack.last() {
            if remain_visible(&points, &rev, idx, points_sorted[i].1, n) {
                break;
            }

            stack.pop();
        }

        // Check colinearity
        if points_sorted[i].0.x * points_sorted[i + 1].0.y
            == points_sorted[i].0.y * points_sorted[i + 1].0.x
        {
            continue;
        }

        // Determine if current vertex itself is visible
        if stack.is_empty()
            || candidate_visible(&points, &rev, points_sorted[i].1, *stack.last().unwrap(), n)
        {
            stack.push(points_sorted[i].1);
        }
    }

    stack.sort_unstable();

    writeln!(out, "{}", stack.len()).unwrap();

    for val in stack {
        write!(out, "{} ", val + 1).unwrap();
    }

    writeln!(out).unwrap();
}

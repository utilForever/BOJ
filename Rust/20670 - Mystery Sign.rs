use io::Write;
use std::{io, ops::Sub, str};

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

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn cross(p1: Point, p2: Point, p3: Point) -> i64 {
    let p12 = p2 - p1;
    let p13 = p3 - p1;

    (p12.x as i64 * p13.y as i64) - (p12.y as i64 * p13.x as i64)
}

fn is_in_poly(poly: &Vec<Point>, p: Point) -> bool {
    let len = poly.len();
    let origin = Point::new(0, 0);
    let left = poly[len - 1] - poly[0];
    let right = poly[1] - poly[0];
    let point = p - poly[0];

    if cross(origin, left, point) > 0 {
        return false;
    }
    if cross(origin, right, point) < 0 {
        return false;
    }

    let mut l = 1;
    let mut r = len - 1;

    while l + 1 < r {
        let m = (l + r) / 2;
        let mid = poly[m] - poly[0];

        if cross(origin, mid, point) > 0 {
            l = m;
        } else {
            r = m;
        }
    }

    let p1 = p - poly[l];
    let p2 = poly[l + 1] - p;

    cross(origin, p1, p2) <= 0
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    let mut poly_a = vec![Point::new(0, 0); n];
    let mut poly_b = vec![Point::new(0, 0); m];
    let mut poly_sign = vec![Point::new(0, 0); k];

    for i in 0..n {
        let (x, y) = (scan.token::<i32>(), scan.token::<i32>());
        poly_a[i] = Point::new(x, y);
    }
    for i in 0..m {
        let (x, y) = (scan.token::<i32>(), scan.token::<i32>());
        poly_b[i] = Point::new(x, y);
    }
    for i in 0..k {
        let (x, y) = (scan.token::<i32>(), scan.token::<i32>());
        poly_sign[i] = Point::new(x, y);
    }

    let mut ans = 0;
    for i in 0..k {
        if !is_in_poly(&poly_a, poly_sign[i]) || is_in_poly(&poly_b, poly_sign[i]) {
            ans += 1;
        }
    }

    if ans == 0 {
        writeln!(out, "YES").unwrap();
    } else {
        writeln!(out, "{}", ans).unwrap();
    }
}

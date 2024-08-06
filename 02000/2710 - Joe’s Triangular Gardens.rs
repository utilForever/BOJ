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

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn sqrt(&mut self) -> Self {
        let dist = self.x.hypot(self.y).sqrt();
        let theta = self.y.atan2(self.x) / 2.0;

        Self {
            x: dist * theta.cos(),
            y: dist * theta.sin(),
        }
    }

    fn dist(&self, other: Self) -> f64 {
        (self.x - other.x).hypot(self.y - other.y)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<Point> for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x * rhs.x - self.y * rhs.y,
            self.x * rhs.y + self.y * rhs.x,
        )
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

// Reference: https://en.wikipedia.org/wiki/Marden%27s_theorem
// NOTE: The focis of ellipse are the roots of p'(z).
//       p(Z) = Z^3 - (a + b + c) * Z^2 + (a * b + b * c + c * a) * Z + a * b * c
//            = Z^3 + B * Z^2 + C * Z + D
//       p'(Z) = 3 * Z^2 -2 * (a + b + c) * Z + (a * b + b * c + c * a)
// The roots ofp'(Z) are below:
// Z = (-b +- sqrt(b * b - 4 * a * c)) / 2 * a
//   = (2 * (a + b + c) +- sqrt(4 * (a + b + c) * (a + b + c) - 4 * 3 * (a * b + b * c + c * a))) / 6
//   = (a + b + c) / 3 +- sqrt(a * a + b * b + c * c - a * b - b * c - c * a) / 3
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();

    for _ in 0..n {
        let a = Point::new(scan.token::<f64>(), scan.token::<f64>());
        let b = Point::new(scan.token::<f64>(), scan.token::<f64>());
        let c = Point::new(scan.token::<f64>(), scan.token::<f64>());

        let center = (a + b + c) / 3.0;
        let linear_eccentricity = (a * a + b * b + c * c - a * b - b * c - c * a).sqrt() / 3.0;
        let mut foci1 = center + linear_eccentricity;
        let mut foci2 = center - linear_eccentricity;

        if foci1.x > foci2.x || (foci1.x == foci2.x && foci1.y > foci2.y) {
            std::mem::swap(&mut foci1, &mut foci2);
        }

        let mid = (a + b) / 2.0;
        let length = mid.dist(foci1) + mid.dist(foci2);

        writeln!(
            out,
            "{:.2} {:.2} {:.2} {:.2} {:.2}",
            foci1.x, foci1.y, foci2.x, foci2.y, length
        )
        .unwrap();
    }
}

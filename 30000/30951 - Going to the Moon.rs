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

#[derive(Copy, Clone)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn length(&self) -> f64 {
        self.x.hypot(self.y)
    }

    fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    fn multiply(&self, scalar: f64) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }

    fn dot(self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn projection(v: Vec2, u: Vec2) -> Option<Vec2> {
        let dot = v.dot(u);
        let length_squared = v.length_squared();
        let scalar = dot / length_squared;

        if scalar < 0.0 || scalar > 1.0 {
            None
        } else {
            Some(v.multiply(scalar))
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let calculate_dist = |a: &Vec2, b: &Vec2, c: &Vec2, r: f64, ratio: f64| -> f64 {
        let d = Vec2::new(a.x + ratio * (b.x - a.x), a.y + ratio * (b.y - a.y));
        let cd = Vec2::new(d.x - c.x, d.y - c.y);
        let e = Vec2::new(
            c.x + (cd.x / cd.length()) * r,
            c.y + (cd.y / cd.length()) * r,
        );

        (e - *a).length() + (*b - e).length()
    };

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (a, b, c, r) = (
            Vec2::new(scan.token::<f64>(), scan.token::<f64>()),
            Vec2::new(scan.token::<f64>(), scan.token::<f64>()),
            Vec2::new(scan.token::<f64>(), scan.token::<f64>()),
            scan.token::<f64>(),
        );

        if (a - c).length() <= r || (b - c).length() <= r {
            writeln!(out, "{:.6}", (a - b).length()).unwrap();
            continue;
        }

        let projected = Vec2::projection(b - a, c - a);

        if let Some(projected) = projected {
            let c_prime = Vec2::new(a.x + projected.x, a.y + projected.y);

            if (c - c_prime).length() <= r {
                writeln!(out, "{:.6}", (a - b).length()).unwrap();
                continue;
            }
        }

        let mut ret = f64::MAX;
        let mut left = 0.0;
        let mut right = 1.0;

        for _ in 0..20 {
            let p1 = (2.0 * left + right) / 3.0;
            let p2 = (left + 2.0 * right) / 3.0;

            let dist_p1 = calculate_dist(&a, &b, &c, r, p1);
            let dist_p2 = calculate_dist(&a, &b, &c, r, p2);
            ret = ret.min(dist_p1).min(dist_p2);

            if dist_p1 <= dist_p2 {
                right = p2;
            } else {
                left = p1;
            }
        }

        writeln!(out, "{:.6}", ret).unwrap();
    }
}

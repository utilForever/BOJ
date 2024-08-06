use io::Write;
use std::{io, str};

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
        (self.x - other.x).hypot(self.y - other.y)
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

// Reference: https://www.geeksforgeeks.org/minimum-enclosing-circle-using-welzls-algorithm/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![Point::default(); n];

    for i in 0..n {
        points[i] = Point::new(scan.token::<f64>(), scan.token::<f64>());
    }

    let ret = Circle::welzl(&points);

    writeln!(
        out,
        "{:.3} {:.3} {:.3}",
        ret.center.x, ret.center.y, ret.radius
    )
    .unwrap();
}

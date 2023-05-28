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
    z: f64,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn magnitude_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    fn dist_squared(&self, other: &Point) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)
    }

    fn dist(&self, other: &Point) -> f64 {
        self.dist_squared(other).sqrt()
    }

    fn scale(&self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }

    fn cross(&self, other: &Point) -> Self {
        let a = self.y * other.z - self.z * other.y;
        let b = self.z * other.x - self.x * other.z;
        let c = self.x * other.y - self.y * other.x;

        Self { x: a, y: b, z: c }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn welzl(points: &Vec<Point>) -> Self {
        let mut points = points.clone();
        let n = points.len();

        Sphere::welzl_internal(&mut points, Vec::new(), n)
    }

    fn welzl_internal(points: &mut [Point], mut r: Vec<Point>, n: usize) -> Self {
        if n == 0 || r.len() == 4 {
            return Self::minimum_enclosing_sphere(&r);
        }

        let idx = Rng::new().next(n as u64) as usize;
        let p = points[idx];

        points.swap(idx, n - 1);

        let sphere = Self::welzl_internal(points, r.clone(), n - 1);

        if sphere.is_inside(&p) {
            return sphere;
        }

        r.push(p);

        Self::welzl_internal(points, r, n - 1)
    }

    fn minimum_enclosing_sphere(points: &[Point]) -> Self {
        let n = points.len();

        assert!(n <= 4);

        if n == 0 {
            Self {
                center: Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                radius: 0.0,
            }
        } else if n == 1 {
            Self {
                center: points[0],
                radius: 0.0,
            }
        } else if n == 2 {
            Self::from_two_points(points[0], points[1])
        } else if n == 3 {
            Self::from_three_points(points[0], points[1], points[2])
        } else {
            Self::from_four_points(points[0], points[1], points[2], points[3])
        }
    }

    fn from_two_points(a: Point, b: Point) -> Self {
        let center = Point {
            x: (a.x + b.x) / 2.0,
            y: (a.y + b.y) / 2.0,
            z: (a.z + b.z) / 2.0,
        };

        Self {
            center,
            radius: a.dist(&b) / 2.0,
        }
    }

    fn from_three_points(a: Point, b: Point, c: Point) -> Self {
        let p = c - a;
        let q = b - a;
        let cross_pq = p.cross(&q);
        let top =
            (q.scale(p.magnitude_squared()) - p.scale(q.magnitude_squared())).cross(&cross_pq);
        let bottom = 0.5 / cross_pq.magnitude_squared();
        let center = top.scale(bottom);

        Self {
            center: Point {
                x: center.x + a.x,
                y: center.y + a.y,
                z: center.z + a.z,
            },
            radius: center.magnitude(),
        }
    }

    fn from_four_points(a: Point, b: Point, c: Point, d: Point) -> Self {
        let p = b - a;
        let q = c - a;
        let r = d - a;
        let length_p = p.magnitude_squared();
        let length_q = q.magnitude_squared();
        let length_r = r.magnitude_squared();
        let determinant = p.x * (q.y * r.z - r.y * q.z) - q.x * (p.y * r.z - r.y * p.z)
            + r.x * (p.y * q.z - q.y * p.z);
        let f = 0.5 / determinant;
        let offset_x = f
            * (length_p * (q.y * r.z - r.y * q.z) - length_q * (p.y * r.z - r.y * p.z)
                + length_r * (p.y * q.z - q.y * p.z));
        let offset_y = f
            * (-length_p * (q.x * r.z - r.x * q.z) + length_q * (p.x * r.z - r.x * p.z)
                - length_r * (p.x * q.z - q.x * p.z));
        let offset_z = f
            * (length_p * (q.x * r.y - r.x * q.y) - length_q * (p.x * r.y - r.x * p.y)
                + length_r * (p.x * q.y - q.x * p.y));

        Self {
            center: Point {
                x: offset_x + a.x,
                y: offset_y + a.y,
                z: offset_z + a.z,
            },
            radius: (offset_x.powi(2) + offset_y.powi(2) + offset_z.powi(2)).sqrt(),
        }
    }

    fn is_inside(&self, point: &Point) -> bool {
        self.center.dist(point) <= self.radius
    }
}

// Reference: https://www.geeksforgeeks.org/minimum-enclosing-circle-using-welzls-algorithm/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut points = vec![Point::default(); n];

        for i in 0..n {
            points[i] = Point::new(
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
            );
        }

        let ret = Sphere::welzl(&points);

        writeln!(out, "{:.5}", ret.radius).unwrap();
    }
}

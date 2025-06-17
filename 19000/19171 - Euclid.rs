use io::Write;
use std::{
    io,
    ops::{Add, Mul, Sub},
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

#[derive(Clone, Copy, Debug, Default)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    #[inline]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    fn norm(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    fn dist(self, other: Vec3) -> f64 {
        (self - other).norm()
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

struct Objective {
    points: [Vec3; 3],
}

impl Objective {
    fn new(points: &[Vec3]) -> Self {
        assert!(points.len() == 3, "Objective requires exactly 3 points");

        Self {
            points: [points[0], points[1], points[2]],
        }
    }

    fn value(&self, p: Vec3) -> f64 {
        self.points.iter().map(|&q| p.dist(q)).sum()
    }

    fn gradient(&self, p: Vec3, eps: f64) -> Vec3 {
        let mut ret = Vec3::default();

        for &q in self.points.iter() {
            let d = p.dist(q).max(eps);
            ret = ret.add(p.sub(q) * (1.0 / d));
        }

        ret
    }
}

struct GradientDescent {
    rho: f64,
    beta: f64,
    eps: f64,
    iter_max: usize,
}

impl GradientDescent {
    fn run(&self, obj: &Objective, mut p: Vec3) -> Vec3 {
        let mut f_cur: f64 = obj.value(p);

        for _ in 0..self.iter_max {
            let g = obj.gradient(p, self.eps);
            let g_norm = g.norm();

            if g_norm < self.eps {
                break;
            }

            let dir = g * (1.0 / g_norm);
            let mut alpha = 100000.0;

            loop {
                let p_next = p.sub(dir * alpha);
                let f_next = obj.value(p_next);

                if f_next <= f_cur - self.rho * alpha * g_norm || alpha < self.eps {
                    p = p_next;
                    f_cur = f_next;
                    break;
                }

                alpha *= self.beta;
            }

            if g_norm * alpha < self.eps {
                break;
            }
        }

        p
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut points = Vec::with_capacity(3);

    for _ in 0..3 {
        let (x, y, z) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
        points.push(Vec3::new(x, y, z));
    }

    let obj = Objective::new(&points);
    let centroid = points.iter().fold(Vec3::default(), |acc, &p| acc.add(p)) * (1.0 / 3.0);
    let gd = GradientDescent {
        rho: 1e-4,
        beta: 0.95,
        eps: 1e-4,
        iter_max: 5000,
    };
    let ret = gd.run(&obj, centroid);

    writeln!(out, "{:.12}", obj.value(ret)).unwrap();
}

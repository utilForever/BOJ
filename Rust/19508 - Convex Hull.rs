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

#[derive(Copy, Clone, Default)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn scale(&self, k: f64) -> Self {
        Self::new(self.x * k, self.y * k, self.z * k)
    }

    fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: &Vector3) -> Vector3 {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn abs(&self) -> f64 {
        self.dot(self).sqrt()
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

struct Face {

}

struct ConvexHull3 {
    points: Vec<Vector3>,
}

impl ConvexHull3 {
    fn new(points: Vec<Vector3>) -> Self {
        Self { points }
    }

    fn calculate(&self) -> Vec<Vector3> {
        let n = self.points.len();
        let check = self.prepare();
        let mut faces = Vec::new();
        let mut faces_new = vec![None; n];

        if !check {
            return Vec::new();
        }

        let mut conflict = vec![Vec::new(); n];
    }

    fn prepare(&mut self) -> bool {
        let n = self.points.len();

        for _ in 0..n {
            let idx1 = Rng::new().next(n as u64) as usize;
            let idx2 = Rng::new().next(n as u64) as usize;

            self.points.swap(idx1, idx2);
        }

        let mut idxes = Vec::new();
        idxes.push(0);

        for i in 1..n {
            if idxes.len() == 1 {
                if (self.points[idxes[0]] - self.points[i]).abs() > 1e-9 {
                    idxes.push(i);
                } else if idxes.len() == 2 {
                    if (self.points[idxes[1]] - self.points[idxes[0]])
                        .cross(&(self.points[i] - self.points[idxes[0]]))
                        .abs()
                        > 1e-9
                    {
                        idxes.push(i);
                    }
                } else if (self.points[i] - self.points[idxes[0]]).dot(
                    &(self.points[idxes[1]] - self.points[idxes[0]])
                        .cross(&(self.points[idxes[2]] - self.points[idxes[0]])),
                ) > 1e-9
                {
                    idxes[1] = i;
                    break;
                }
            }
        }

        if idxes.len() != 4 {
            return false;
        }

        let mut additional_points = Vec::new();

        for i in idxes.iter() {
            additional_points.push(self.points[*i]);
        }

        idxes.reverse();

        for i in idxes.iter() {
            self.points.remove(*i);
        }

        self.points.append(&mut additional_points);

        true
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut points = vec![Vector3::default(); n];

    for i in 0..n {
        points[i] = Vector3::new(
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
    }

    let convex_hull = ConvexHull3::new(points);
    let faces = convex_hull.calculate();
}

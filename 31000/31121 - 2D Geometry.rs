use io::Write;
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hasher},
    iter::repeat_with,
};
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

#[derive(Clone, Debug)]
pub struct Xorshift {
    y: u64,
}

impl Xorshift {
    pub fn new_with_seed(seed: u64) -> Self {
        Xorshift { y: seed }
    }

    pub fn new() -> Self {
        Xorshift::new_with_seed(RandomState::new().build_hasher().finish())
    }

    pub fn rand64(&mut self) -> u64 {
        self.y ^= self.y << 5;
        self.y ^= self.y >> 17;
        self.y ^= self.y << 11;
        self.y
    }

    pub fn rand(&mut self, k: u64) -> u64 {
        self.rand64() % k
    }

    pub fn rands(&mut self, k: u64, n: usize) -> Vec<u64> {
        repeat_with(|| self.rand(k)).take(n).collect()
    }

    pub fn randf(&mut self) -> f64 {
        const UPPER_MASK: u64 = 0x3FF0_0000_0000_0000;
        const LOWER_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;
        let x = self.rand64();
        let tmp = UPPER_MASK | (x & LOWER_MASK);
        let result: f64 = f64::from_bits(tmp);
        f64::from_bits(f64::to_bits(result - 1.0) ^ (x >> 63))
    }

    pub fn gen_bool(&mut self, p: f64) -> bool {
        self.randf() < p
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let mut n = slice.len();
        while n > 1 {
            let i = self.rand(n as _) as usize;
            n -= 1;
            slice.swap(i, n);
        }
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    #[inline(always)]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<i64> for Point {
    type Output = Point;

    fn mul(self, scalar: i64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Div<i64> for Point {
    type Output = Point;

    fn div(self, rhs: i64) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

fn count_on_line(points: &Vec<Point>, i: usize, j: usize) -> i64 {
    let p0 = points[i];
    let p1 = points[j];
    let dx1 = p1.x - p0.x;
    let dy1 = p1.y - p0.y;

    let mut cnt = 0;

    for &point in points {
        let dx2 = point.x - p0.x;
        let dy2 = point.y - p0.y;

        if dx1 * dy2 - dy1 * dx2 == 0 {
            cnt += 1;
        }
    }

    cnt
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let line = scan.line().trim().to_string();

        if line.is_empty() {
            break;
        }

        let n = line.parse::<usize>().unwrap();
        let mut points = Vec::with_capacity(n);

        for _ in 0..n {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            points.push(Point { x, y });
        }

        let mut rng = Xorshift::new();
        let mut ret = n as i64 % 3;

        for _ in 0..100 {
            let i = rng.rand(n as u64) as usize;
            let j = rng.rand(n as u64) as usize;

            if i == j {
                continue;
            }

            let cnt = count_on_line(&points, i, j);
            ret = ret.max(3 * cnt - 2 * n as i64);
        }

        writeln!(out, "{ret}").unwrap();
    }
}

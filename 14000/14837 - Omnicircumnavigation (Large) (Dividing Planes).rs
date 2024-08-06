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

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point3 {
    x: i128,
    y: i128,
    z: i128,
}

impl Point3 {
    fn new(x: i128, y: i128, z: i128) -> Self {
        Self { x, y, z }
    }

    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0
    }

    fn cross(&self, other: &Point3) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn dot(&self, other: &Point3) -> i128 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl std::ops::Add for Point3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Point3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

fn check_omnicircumnavigation(points: &Vec<Point3>) -> bool {
    let mut points_new = Vec::new();

    // Check colinear
    for i in 0..points.len() {
        let mut found = false;

        for j in i + 1..points.len() {
            // It is colinear
            if points[i].cross(&points[j]).is_zero() {
                // Angle is 180 degree
                if points[i].dot(&points[j]) < 0 {
                    return true;
                } else {
                    found = true;
                    break;
                }
            }
        }

        // Insert non-colinear point only
        if !found {
            points_new.push(points[i]);
        }
    }

    for i in 0..points_new.len() {
        let mut point_leftmost = None;

        // Find leftmost point
        for j in 0..points_new.len() {
            if j == i || (point_leftmost.is_some() && j == point_leftmost.unwrap()) {
                continue;
            }

            if point_leftmost.is_none()
                || points_new[i]
                    .cross(&points_new[point_leftmost.unwrap()])
                    .dot(&points_new[j])
                    > 0
            {
                point_leftmost = Some(j);
            }
        }

        let point_leftmost = point_leftmost.unwrap();
        let mut is_omnicircumnavigation = false;

        for j in 0..points_new.len() {
            if j == i || j == point_leftmost {
                continue;
            }

            // Check coplanar
            let coplanar = points_new[i]
                .cross(&points_new[point_leftmost])
                .dot(&points_new[j]);

            // It is coplanar
            if coplanar == 0 {
                // [(0, 0), points_new[i], points_new[point_leftmost]]
                let angle1 = points_new[i].cross(&points_new[point_leftmost]);
                // [(0, 0), points_new[i], points_new[j]]
                let angle2 = points_new[i].cross(&points_new[j]);

                // Angle is 180 degree
                if angle1.dot(&angle2) < 0 {
                    is_omnicircumnavigation = true;
                    break;
                }
            } else if coplanar > 0 {
                is_omnicircumnavigation = true;
                break;
            }
        }

        if !is_omnicircumnavigation {
            return false;
        }
    }

    true
}

// Reference: Google Code Jam World Finals 2017 Analysis
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut points = Vec::with_capacity(n);

        for _ in 0..n {
            let x = scan.token::<i128>();
            let y = scan.token::<i128>();
            let z = scan.token::<i128>();
            points.push(Point3::new(x, y, z));
        }

        writeln!(
            out,
            "Case #{i}: {}",
            if check_omnicircumnavigation(&points) {
                "YES"
            } else {
                "NO"
            }
        )
        .unwrap();
    }
}

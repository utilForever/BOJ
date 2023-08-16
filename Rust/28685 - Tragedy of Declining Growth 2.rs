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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn ccw(p1: Point, p2: Point, p3: Point) -> i64 {
        let (x1, y1) = (p1.x, p1.y);
        let (x2, y2) = (p2.x, p2.y);
        let (x3, y3) = (p3.x, p3.y);

        (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct ConvexHull {
    points: Vec<Point>,
}

impl ConvexHull {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn make_upper(&mut self) -> Vec<Point> {
        let mut ret = Vec::new();

        for p in self.points.iter() {
            while ret.len() >= 2 && Point::ccw(ret[ret.len() - 2], ret[ret.len() - 1], *p) <= 0 {
                ret.pop();
            }

            ret.push(*p);
        }

        ret
    }

    fn make_lower(&mut self) -> Vec<Point> {
        let mut ret = Vec::new();

        for p in self.points.iter() {
            while ret.len() >= 2 && Point::ccw(ret[ret.len() - 2], ret[ret.len() - 1], *p) >= 0 {
                ret.pop();
            }

            ret.push(*p);
        }

        ret
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points_upper = vec![Point::default(); n + 1];
    let mut points_lower = vec![Point::default(); n + 1];
    let mut y = 0;

    points_upper[0] = Point::new(0, 1);

    for i in 1..=n {
        let dy = scan.token::<i64>();
        y += dy;

        points_upper[i] = Point::new(i as i64, y + 1);
        points_lower[i] = Point::new(i as i64, y);
    }

    let mut convex_hull_upper = ConvexHull::new(points_upper);
    let mut convex_hull_lower = ConvexHull::new(points_lower);

    let hull_upper = convex_hull_upper.make_upper();
    let hull_lower = convex_hull_lower.make_lower();

    let mut idx1 = 0;
    let mut idx2 = 0;

    while idx2 < hull_upper.len() - 1 {
        while hull_lower[idx1 + 1].x <= hull_upper[idx2].x {
            idx1 += 1;
        }

        if hull_lower[idx1].x == hull_upper[idx2].x {
            idx2 += 1;
            continue;
        }

        if Point::ccw(hull_lower[idx1], hull_upper[idx2], hull_lower[idx1 + 1]) >= 0 {
            writeln!(out, "Impossible").unwrap();
            return;
        }

        idx2 += 1;
    }

    let mut idx1 = 0;
    let mut idx2 = 0;

    while idx1 < hull_lower.len() - 1 {
        while hull_upper[idx2 + 1].x <= hull_lower[idx1].x {
            idx2 += 1;
        }

        if hull_upper[idx2].x == hull_lower[idx1].x {
            idx1 += 1;
            continue;
        }

        if Point::ccw(hull_upper[idx2], hull_lower[idx1], hull_upper[idx2 + 1]) <= 0 {
            writeln!(out, "Impossible").unwrap();
            return;
        }

        idx1 += 1;
    }

    let mut idx1 = 0;
    let mut idx2 = hull_upper.len() - 1;

    loop {
        while Point::ccw(hull_lower[idx1], hull_lower[idx1 + 1], hull_upper[idx2]) < 0 {
            idx1 += 1;
        }

        while Point::ccw(hull_lower[idx1], hull_upper[idx2 - 1], hull_upper[idx2]) > 0 {
            idx2 -= 1;
        }

        if Point::ccw(hull_lower[idx1], hull_lower[idx1 + 1], hull_upper[idx2]) >= 0 {
            break;
        }
    }

    writeln!(
        out,
        "{:.6}",
        (hull_upper[idx2].y - hull_lower[idx1].y) as f64
            / (hull_upper[idx2].x - hull_lower[idx1].x) as f64
    )
    .unwrap();
}

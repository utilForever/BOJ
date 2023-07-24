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

// Reference: https://00ad-8e71-00ff-055d.tistory.com/98
fn is_intersect(hull_upper: &Vec<Point>, hull_lower: &Vec<Point>) -> bool {
    for i in 0..hull_lower.len() - 1 {
        for j in 0..hull_upper.len() - 1 {
            let a = hull_lower[i];
            let b = hull_lower[i + 1];
            let c = hull_upper[j];
            let d = hull_upper[j + 1];

            let c1 = Point::ccw(a, b, c);
            let c2 = Point::ccw(a, b, d);
            let c3 = Point::ccw(c, d, a);
            let c4 = Point::ccw(c, d, b);

            if c1 * c2 > 0 || c3 * c4 > 0 {
                continue;
            } else if c1 != 0 || c2 != 0 || c3 != 0 || c4 != 0 {
                return true;
            } else if a.x.min(b.x) > c.x.max(d.x)
                || a.y.min(b.y) > c.y.max(d.y)
                || c.x.min(d.x) > a.x.max(b.x)
                || c.y.min(d.y) > a.y.max(b.y)
            {
                continue;
            } else {
                return true;
            }
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (r, c, k) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let start_upper = Point::new(0, 1);
    let start_lower = Point::new(1, 0);
    let end_upper = Point::new(r - 1, c);
    let end_lower = Point::new(r, c - 1);

    let mut points_upper = Vec::new();
    let mut points_lower = Vec::new();

    points_upper.push(start_upper);
    points_upper.push(end_upper);
    points_lower.push(start_lower);
    points_lower.push(end_lower);

    for _ in 0..k {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        let point_upper = Point::new(x - 1, y);
        let point_lower = Point::new(x, y - 1);

        if Point::ccw(start_lower, end_lower, point_upper) >= 0
            && Point::ccw(start_lower, end_lower, point_lower) <= 0
        {
            points_lower.push(point_upper);
        } else if Point::ccw(start_upper, end_upper, point_upper) >= 0
            && Point::ccw(start_upper, end_upper, point_lower) <= 0
        {
            points_upper.push(point_lower);
        }
    }

    points_upper.sort();
    points_lower.sort();

    let mut convex_hull_upper = ConvexHull::new(points_upper);
    let mut convex_hull_lower = ConvexHull::new(points_lower);

    let hull_upper = convex_hull_upper.make_upper();
    let hull_lower = convex_hull_lower.make_lower();

    writeln!(
        out,
        "{}",
        if is_intersect(&hull_upper, &hull_lower) {
            "0"
        } else {
            "1"
        }
    )
    .unwrap();
}

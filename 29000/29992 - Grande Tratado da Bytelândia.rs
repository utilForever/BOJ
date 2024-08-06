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
    idx: usize,
}

impl Point {
    fn new(x: i64, y: i64, idx: usize) -> Self {
        Self { x, y, idx }
    }

    fn ccw(p1: Point, p2: Point, p3: Point) -> i64 {
        let (x1, y1) = (p1.x, p1.y);
        let (x2, y2) = (p2.x, p2.y);
        let (x3, y3) = (p3.x, p3.y);

        (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)
    }
}

struct ConvexHull {
    points: Vec<Point>,
    hull: Vec<Point>,
}

impl ConvexHull {
    fn new(points: Vec<Point>) -> Self {
        Self {
            points,
            hull: Vec::new(),
        }
    }

    fn make(&mut self, exclude_colinear: bool) {
        let mut upper = Vec::new();
        let mut lower = Vec::new();

        for p in self.points.iter() {
            while upper.len() >= 2
                && Point::ccw(upper[upper.len() - 1], *p, upper[upper.len() - 2])
                    < exclude_colinear as i64
            {
                upper.pop();
            }

            upper.push(*p);
        }

        for p in self.points.iter().rev() {
            while lower.len() >= 2
                && Point::ccw(lower[lower.len() - 1], *p, lower[lower.len() - 2])
                    < exclude_colinear as i64
            {
                lower.pop();
            }

            lower.push(*p);
        }

        upper.pop();
        lower.pop();

        self.hull = upper.into_iter().chain(lower.into_iter()).collect();
    }

    fn hull(&self) -> &Vec<Point> {
        &self.hull
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![Point::default(); n];

    for i in 0..n {
        points[i] = Point::new(scan.token::<i64>(), scan.token::<i64>(), i + 1);
    }

    points.sort();

    let mut convex_hull = ConvexHull::new(points.clone());
    convex_hull.make(false);

    let hull = convex_hull.hull();
    let mut ret = hull.iter().map(|p| p.idx).collect::<Vec<_>>();

    ret.sort();
    ret.dedup();

    for idx in ret {
        write!(out, "{idx} ").unwrap();
    }

    writeln!(out).unwrap();
}

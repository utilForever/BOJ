use io::Write;
use std::{io, str, ops::Sub};

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

    fn dist_squared(p1: &Point, p2: &Point) -> i64 {
        (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)
    }
}

impl Sub for Point {
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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut points = vec![Point::new(0, 0); n * 4];
    
        for i in 0..n {
            let (x, y, w) = (scan.token::<i64>(), scan.token::<i64>(), scan.token::<i64>());
            points[i * 4] = Point::new(x, y);
            points[i * 4 + 1] = Point::new(x + w, y);
            points[i * 4 + 2] = Point::new(x, y + w);
            points[i * 4 + 3] = Point::new(x + w, y + w);
        }
    
        points.sort();

        let mut convex_hull = ConvexHull::new(points.clone());
        convex_hull.make(true);

        let hull = convex_hull.hull().clone();
    
        let mut ret = 0;
        let mut c = 1;
    
        for a in 0..hull.len() {
            let b = (a + 1) % hull.len();
    
            loop {
                let d = (c + 1) % hull.len();
    
                let zero = Point::new(0, 0);
                let ab = hull[b].clone() - hull[a].clone();
                let cd = hull[d].clone() - hull[c].clone();
    
                if Point::ccw(zero, ab, cd) > 0 {
                    c = d;
                } else {
                    break;
                }
            }
    
            let dist = Point::dist_squared(&hull[a], &hull[c]);
            ret = ret.max(dist);
        }
    
        writeln!(out, "{ret}").unwrap();
    }
}

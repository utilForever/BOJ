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

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn ccw(p1: Point, p2: Point, p3: Point) -> f64 {
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
            while upper.len() >= 2 {
                let ccw = Point::ccw(upper[upper.len() - 1], *p, upper[upper.len() - 2]);
                let check_collinearity = if exclude_colinear {
                    ccw < 1e-6
                } else {
                    ccw < 0.0
                };

                if !check_collinearity {
                    break;
                }

                upper.pop();
            }

            upper.push(*p);
        }

        for p in self.points.iter().rev() {
            while lower.len() >= 2 {
                let ccw = Point::ccw(lower[lower.len() - 1], *p, lower[lower.len() - 2]);
                let check_collinearity = if exclude_colinear {
                    ccw < 1e-6
                } else {
                    ccw < 0.0
                };

                if !check_collinearity {
                    break;
                }

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

    fn area(&self) -> f64 {
        if self.hull.len() < 3 {
            return 0.0;
        }

        let mut ret = 0.0;
        let a = self.hull[0];

        for i in 1..self.hull.len() - 1 {
            let b = self.hull[i];
            let c = self.hull[i + 1];

            ret += Point::ccw(a, b, c);
        }

        ret.abs() / 2.0
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let (id, n) = (scan.token::<String>(), scan.token::<usize>());

        if id == "ZZ" && n == 0 {
            break;
        }

        let mut points = vec![Point::default(); n];

        for i in 0..n {
            points[i] = Point::new(scan.token::<f64>(), scan.token::<f64>());
        }

        points.sort_by(|a, b| {
            if a.x == b.x {
                a.y.partial_cmp(&b.y).unwrap()
            } else {
                a.x.partial_cmp(&b.x).unwrap()
            }
        });

        let mut convex_hull_outer = ConvexHull::new(points.clone());
        convex_hull_outer.make(true);

        for p in convex_hull_outer.hull().iter() {
            points.retain(|x| x != p);
        }

        let mut convex_hull_inner = ConvexHull::new(points);
        convex_hull_inner.make(true);

        let area_outer = convex_hull_outer.area();
        let area_inner = convex_hull_inner.area();

        writeln!(out, "ProblemID {id}: {:.4}", area_outer - area_inner).unwrap();
    }
}

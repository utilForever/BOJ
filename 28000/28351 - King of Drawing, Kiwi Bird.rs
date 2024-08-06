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

#[derive(Debug, Default, Copy, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    fn slope(&self) -> f64 {
        let (x1, y1) = (self.p1.x, self.p1.y);
        let (x2, y2) = (self.p2.x, self.p2.y);

        (y2 - y1) as f64 / (x2 - x1) as f64
    }

    fn is_constrained(&self, points: &Vec<Point>) -> bool {
        let mut count = 0;

        for p in points.iter() {
            if Point::ccw(self.p1, self.p2, *p) == 0 {
                count += 1;
            }
        }

        count > 2
    }
}

struct Triangle {
    a: Line,
    b: Line,
    c: Line,
}

impl Triangle {
    fn new(a: Line, b: Line, c: Line) -> Self {
        Self { a, b, c }
    }

    fn is_valid(&self) -> bool {
        let v1 = Point::new(self.a.p2.x - self.a.p1.x, self.a.p2.y - self.a.p1.y);
        let v2 = Point::new(self.b.p2.x - self.b.p1.x, self.b.p2.y - self.b.p1.y);
        let v3 = Point::new(self.c.p2.x - self.c.p1.x, self.c.p2.y - self.c.p1.y);

        let cond1 = Point::ccw(v1, v2, Point::new(0, 0)) > 0;
        let cond2 = Point::ccw(v2, v3, Point::new(0, 0)) > 0;
        let cond3 = Point::ccw(v3, v1, Point::new(0, 0)) > 0;

        cond1 && cond2 && cond3
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

enum NumTriangles {
    Zero,
    One(usize, usize, usize),
    Two,
    Inf,
}

fn calculate_triangles(
    lines: &Vec<Line>,
    lines_constrained: &Vec<bool>,
    idxes: &Vec<usize>,
) -> NumTriangles {
    if idxes.len() > 3 {
        return NumTriangles::Zero;
    }

    if idxes.len() == 3 {
        let triangle = Triangle::new(lines[idxes[0]], lines[idxes[1]], lines[idxes[2]]);

        if triangle.is_valid() {
            return NumTriangles::One(idxes[0], idxes[1], idxes[2]);
        } else {
            return NumTriangles::Zero;
        }
    }

    if lines.len() < 4 {
        return NumTriangles::Inf;
    }

    if lines.len() == 4 {
        if idxes.len() < 2 {
            return NumTriangles::Inf;
        }

        let v1 = Point::new(
            lines[idxes[0]].p2.x - lines[idxes[0]].p1.x,
            lines[idxes[0]].p2.y - lines[idxes[0]].p1.y,
        );
        let v2 = Point::new(
            lines[idxes[1]].p2.x - lines[idxes[1]].p1.x,
            lines[idxes[1]].p2.y - lines[idxes[1]].p1.y,
        );

        let cond1 = idxes[0] == (idxes[1] + 1) % 4;
        let cond2 = (idxes[0] + 1) % 4 == idxes[1];
        let cond3 = Point::ccw(v1, v2, Point::new(0, 0)) != 0;

        if cond1 || cond2 || cond3 {
            return NumTriangles::Inf;
        } else {
            return NumTriangles::Zero;
        }
    }

    if lines.len() == 5 {
        if idxes.len() < 2 {
            return NumTriangles::Inf;
        }

        let mut idx_pair = (idxes[0], idxes[1]);
        if (idx_pair.1 + 5 - idx_pair.0) % 5 > 2 {
            idx_pair = (idx_pair.1, idx_pair.0);
        }

        let v1 = Point::new(
            lines[idx_pair.0].p2.x - lines[idx_pair.0].p1.x,
            lines[idx_pair.0].p2.y - lines[idx_pair.0].p1.y,
        );
        let v2 = Point::new(
            lines[idx_pair.1].p2.x - lines[idx_pair.1].p1.x,
            lines[idx_pair.1].p2.y - lines[idx_pair.1].p1.y,
        );

        if (idx_pair.1 + 5 - idx_pair.0) % 5 == 1 {
            let third = (idx_pair.0 + 3) % 5;
            let triangle = Triangle::new(lines[idx_pair.0], lines[idx_pair.1], lines[third]);

            if triangle.is_valid() {
                return NumTriangles::One(idx_pair.0, idx_pair.1, third);
            } else {
                return NumTriangles::Zero;
            }
        } else if Point::ccw(v1, v2, Point::new(0, 0)) > 0 {
            return NumTriangles::Inf;
        } else {
            return NumTriangles::Zero;
        }
    }

    let mut ret = NumTriangles::Zero;

    for idx in 0..=1 {
        let mut is_constrained = false;

        if idx == 0 {
            for i in (1..6).step_by(2) {
                if lines_constrained[i] {
                    is_constrained = true;
                    break;
                }
            }
        } else {
            for i in (0..6).step_by(2) {
                if lines_constrained[i] {
                    is_constrained = true;
                    break;
                }
            }
        }

        if is_constrained {
            continue;
        }

        let triangle = Triangle::new(lines[idx], lines[idx + 2], lines[idx + 4]);

        if triangle.is_valid() {
            match ret {
                NumTriangles::Zero => {
                    ret = NumTriangles::One(idx, idx + 2, idx + 4);
                }
                NumTriangles::One(_, _, _) => {
                    ret = NumTriangles::Two;
                }
                _ => unreachable!(),
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![Point::default(); n];

    for i in 0..n {
        points[i] = Point::new(scan.token::<i64>(), scan.token::<i64>());
    }

    if n < 4 {
        writeln!(out, "-1").unwrap();
        return;
    }

    points.sort();

    let mut convex_hull = ConvexHull::new(points.clone());
    convex_hull.make(false);

    if convex_hull.hull().len() < points.len() {
        writeln!(out, "0").unwrap();
        return;
    }

    convex_hull.make(true);

    if convex_hull.hull().len() > 6 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut hull = convex_hull.hull().clone();
    hull.push(hull[0]);

    let lines = hull
        .windows(2)
        .map(|p| Line::new(p[0], p[1]))
        .collect::<Vec<_>>();
    let lines_constrained = lines
        .iter()
        .map(|l| l.is_constrained(&points))
        .collect::<Vec<_>>();
    let idxes = lines_constrained
        .iter()
        .enumerate()
        .filter(|(_, &val)| val)
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();

    let ret = calculate_triangles(&lines, &lines_constrained, &idxes);

    match ret {
        NumTriangles::Zero => {
            writeln!(out, "0").unwrap();
        }
        NumTriangles::One(a, b, c) => {
            writeln!(out, "1").unwrap();
            writeln!(
                out,
                "{:.6}\n{:.6}\n{:.6}",
                lines[a].slope(),
                lines[b].slope(),
                lines[c].slope()
            )
            .unwrap();
        }
        NumTriangles::Two => {
            writeln!(out, "2").unwrap();
        }
        NumTriangles::Inf => {
            writeln!(out, "-1").unwrap();
        }
    }
}

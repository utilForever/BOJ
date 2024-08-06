use io::Write;
use std::{collections::BTreeSet, io, ops::Sub, str};

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

    fn dist(p1: &Point, p2: &Point) -> f64 {
        ((p1.x - p2.x) as f64).hypot((p1.y - p2.y) as f64)
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

#[derive(Default)]
struct ConvexHull {
    hull_upper: BTreeSet<Point>,
    hull_lower: BTreeSet<Point>,
    area: f64,
    perimeter: f64,
}

#[derive(Clone, Copy)]
enum HullType {
    Upper,
    Lower,
}

impl ConvexHull {
    fn make_hull(&mut self, hull_type: HullType, point: Point) {
        let hull = match hull_type {
            HullType::Upper => &mut self.hull_upper,
            HullType::Lower => &mut self.hull_lower,
        };

        let check_direction = |p1: Point, p2: Point, p3: Point| -> bool {
            let ccw = Point::ccw(p1, p2, p3);

            match hull_type {
                HullType::Upper => ccw > 0,
                HullType::Lower => ccw < 0,
            }
        };
        let check_clockwise = |hull_type: HullType, flag: bool| -> bool {
            match hull_type {
                HullType::Upper => flag == true,
                HullType::Lower => flag == false,
            }
        };

        let prev = hull.range(..point).next_back();
        let next = hull.range(point..).next();

        if let (Some(&p1), Some(&p2)) = (prev, next) {
            if !check_direction(p1, point, p2) {
                return;
            }

            self.perimeter += ConvexHull::perimeter(p1, p2, false);
            self.area += ConvexHull::area(p1, p2, check_clockwise(hull_type, false));
        }

        if let Some(&p) = prev {
            self.perimeter += ConvexHull::perimeter(p, point, true);
            self.area += ConvexHull::area(p, point, check_clockwise(hull_type, true));
        }

        if let Some(&p) = next {
            self.perimeter += ConvexHull::perimeter(point, p, true);
            self.area += ConvexHull::area(point, p, check_clockwise(hull_type, true));
        }

        loop {
            let mut prev = hull.range(..point);
            let p2 = prev.next_back();
            let p1 = prev.next_back();

            if let (Some(&p1), Some(&p2)) = (p1, p2) {
                if check_direction(p1, p2, point) {
                    break;
                }

                self.perimeter += ConvexHull::perimeter(p1, p2, false);
                self.area += ConvexHull::area(p1, p2, check_clockwise(hull_type, false));

                self.perimeter += ConvexHull::perimeter(p1, point, true);
                self.area += ConvexHull::area(p1, point, check_clockwise(hull_type, true));

                self.perimeter += ConvexHull::perimeter(p2, point, false);
                self.area += ConvexHull::area(p2, point, check_clockwise(hull_type, false));

                hull.remove(&p2);
            } else {
                break;
            }
        }

        loop {
            let mut next = hull.range(point..);
            let p1 = next.next();
            let p2 = next.next();

            if let (Some(&p1), Some(&p2)) = (p1, p2) {
                if check_direction(point, p1, p2) {
                    break;
                }

                self.perimeter += ConvexHull::perimeter(point, p1, false);
                self.area += ConvexHull::area(point, p1, check_clockwise(hull_type, false));

                self.perimeter += ConvexHull::perimeter(point, p2, true);
                self.area += ConvexHull::area(point, p2, check_clockwise(hull_type, true));

                self.perimeter += ConvexHull::perimeter(p1, p2, false);
                self.area += ConvexHull::area(p1, p2, check_clockwise(hull_type, false));

                hull.remove(&p1);
            } else {
                break;
            }
        }

        hull.insert(point);
    }

    fn add_point(&mut self, point: Point) {
        if self.hull_upper.is_empty() {
            self.hull_upper.insert(point);
            self.hull_lower.insert(point);

            return;
        }

        if self.hull_upper.len() == 1 {
            let p = *self.hull_upper.iter().next().unwrap();
            self.perimeter += 2.0 * Point::dist(&p, &point);

            self.hull_upper.insert(point);
            self.hull_lower.insert(point);

            return;
        }

        self.make_hull(HullType::Upper, point);
        self.make_hull(HullType::Lower, point);
    }

    fn perimeter(p1: Point, p2: Point, flag: bool) -> f64 {
        if flag {
            Point::dist(&p1, &p2)
        } else {
            -Point::dist(&p1, &p2)
        }
    }

    fn area(p1: Point, p2: Point, flag: bool) -> f64 {
        if flag {
            Point::ccw(p1, p2, Point::new(0, 0)) as f64 / 2.0
        } else {
            -Point::ccw(p1, p2, Point::new(0, 0)) as f64 / 2.0
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut convex_hull = ConvexHull::default();

    for _ in 0..n {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        convex_hull.add_point(Point::new(x, y));
    }

    let q = scan.token::<usize>();

    for _ in 0..q {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        convex_hull.add_point(Point::new(x, y));

        writeln!(out, "{:.12} {:.1}", convex_hull.perimeter, convex_hull.area).unwrap();
    }
}

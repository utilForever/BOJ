use io::Write;
use std::{cmp::Ordering, io, ops::Sub, str};

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

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Point {
    fn new(x: i32, y: i32, dx: i32, dy: i32) -> Self {
        Self { x, y, dx, dy }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            dx: 0,
            dy: 0,
        }
    }
}

fn calculate_ccw(p1: Point, p2: Point, p3: Point) -> i64 {
    let (x1, y1) = (p1.x as i64, p1.y as i64);
    let (x2, y2) = (p2.x as i64, p2.y as i64);
    let (x3, y3) = (p3.x as i64, p3.y as i64);

    let res = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1);
    if res > 0 {
        1
    } else if res < 0 {
        -1
    } else {
        0
    }
}

fn next_to_top(stack: &mut Vec<Point>) -> Point {
    let top = stack.pop().unwrap();
    let next = stack.pop().unwrap();

    stack.push(next.clone());
    stack.push(top);

    next
}

fn get_convex_hull(points: &mut Vec<Point>, num_points: usize) -> Vec<Point> {
    points.sort_by(|a, b| {
        if a.dx as i64 * b.dy as i64 != a.dy as i64 * b.dx as i64 {
            return (a.dx as i64 * b.dy as i64)
                .cmp(&(a.dy as i64 * b.dx as i64))
                .reverse();
        }

        if a.y != b.y {
            return a.y.cmp(&b.y);
        }

        a.x.cmp(&b.x)
    });

    for i in 1..num_points {
        points[i].dx = points[i].x - points[0].x;
        points[i].dy = points[i].y - points[0].y;
    }

    let first_point = points.remove(0);
    points.sort_by(|a, b| {
        if a.dx as i64 * b.dy as i64 != a.dy as i64 * b.dx as i64 {
            return (a.dx as i64 * b.dy as i64)
                .cmp(&(a.dy as i64 * b.dx as i64))
                .reverse();
        }

        if a.y != b.y {
            return a.y.cmp(&b.y);
        }

        a.x.cmp(&b.x)
    });
    points.insert(0, first_point);

    let mut stack = Vec::new();
    stack.push(points[0].clone());
    stack.push(if points.len() == 1 { points[0].clone() } else { points[1].clone() });

    for i in 2..num_points {
        while stack.len() >= 2
            && calculate_ccw(
                stack.last().unwrap().clone(),
                next_to_top(&mut stack),
                points[i].clone(),
            ) >= 0
        {
            stack.pop();
        }

        stack.push(points[i].clone());
    }

    let mut convex_hull = vec![Point::new(0, 0, 0, 0); stack.len()];
    let mut index = stack.len() - 1;

    while !stack.is_empty() {
        convex_hull[index] = stack.pop().unwrap();
        index -= 1;
    }

    convex_hull
}

fn get_segments(convex: &Vec<Point>) -> Vec<(Point, Point)> {
    let mut segments = Vec::new();

    for i in 0..convex.len() {
        segments.push((convex[i].clone(), convex[(i + 1) % convex.len()].clone()));
    }

    segments
}

fn is_segment_intersects(mut a: Point, mut b: Point, mut c: Point, mut d: Point) -> bool {
    let ab = calculate_ccw(a.clone(), b.clone(), c.clone())
        * calculate_ccw(a.clone(), b.clone(), d.clone());
    let cd = calculate_ccw(c.clone(), d.clone(), a.clone())
        * calculate_ccw(c.clone(), d.clone(), b.clone());

    let cmp = |a: Point, b: Point| {
        if a.dx as i64 * b.dy as i64 != a.dy as i64 * b.dx as i64 {
            return (a.dx as i64 * b.dy as i64)
                .cmp(&(a.dy as i64 * b.dx as i64))
                .reverse();
        }

        if a.y != b.y {
            return a.y.cmp(&b.y);
        }

        a.x.cmp(&b.x)
    };

    if ab == 0 && cd == 0 {
        if cmp(b.clone(), a.clone()) == Ordering::Less {
            std::mem::swap(&mut a, &mut b);
        }

        if cmp(d.clone(), c.clone()) == Ordering::Less {
            std::mem::swap(&mut c, &mut d);
        }

        return !(cmp(b, c) == Ordering::Less || cmp(d, a) == Ordering::Less);
    }

    ab <= 0 && cd <= 0
}

fn can_separate(
    blacks: &Vec<Point>,
    whites: &Vec<Point>,
    black_segments: &Vec<(Point, Point)>,
    white_segments: &Vec<(Point, Point)>,
) -> bool {
    for i in 0..black_segments.len() {
        for j in 0..white_segments.len() {
            if is_segment_intersects(
                black_segments[i].0.clone(),
                black_segments[i].1.clone(),
                white_segments[j].0.clone(),
                white_segments[j].1.clone(),
            ) {
                return false;
            }
        }
    }

    if !white_segments.is_empty() {
        let x = Point::new(blacks[0].x - 1, 0, 0, 0);
        let mut cnt = 0;

        for i in 0..white_segments.len() {
            if is_segment_intersects(
                blacks[0].clone(),
                x.clone(),
                white_segments[i].0.clone(),
                white_segments[i].1.clone(),
            ) {
                cnt += 1;
            }
        }

        if cnt % 2 == 1 {
            return false;
        }
    }

    if !black_segments.is_empty() {
        let x = Point::new(whites[0].x - 1, 0, 0, 0);
        let mut cnt = 0;

        for i in 0..black_segments.len() {
            if is_segment_intersects(
                whites[0].clone(),
                x.clone(),
                black_segments[i].0.clone(),
                black_segments[i].1.clone(),
            ) {
                cnt += 1;
            }
        }

        if cnt % 2 == 1 {
            return false;
        }
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token();

    for _ in 0..t {
        let (n, m) = (scan.token(), scan.token());

        let mut blacks = Vec::new();
        let mut whites = Vec::new();

        for _ in 0..n {
            blacks.push(Point::new(scan.token(), scan.token(), 0, 0));
        }
        for _ in 0..m {
            whites.push(Point::new(scan.token(), scan.token(), 0, 0));
        }

        let black_convex = get_convex_hull(&mut blacks, n);
        let white_convex = get_convex_hull(&mut whites, m);

        let black_segments = get_segments(&black_convex);
        let white_segments = get_segments(&white_convex);

        writeln!(
            out,
            "{}",
            if can_separate(&blacks, &whites, &black_segments, &white_segments) {
                "YES"
            } else {
                "NO"
            }
        )
        .unwrap();
    }
}

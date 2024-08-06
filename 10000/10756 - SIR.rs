use io::Write;
use std::{io, ops::Sub, str};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl Point {
    fn new(x: i64, y: i64, dx: i64, dy: i64) -> Self {
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

fn cross(p1: Point, p2: Point, p3: Point) -> i64 {
    (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x)
}

fn calculate_ccw(p1: Point, p2: Point, p3: Point) -> i64 {
    let (x1, y1) = (p1.x, p1.y);
    let (x2, y2) = (p2.x, p2.y);
    let (x3, y3) = (p3.x, p3.y);

    let ret = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1);

    if ret > 0 {
        1
    } else if ret < 0 {
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

// Reference: Croatian Olympiad in Informatics 2015 Editorial
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut cheeses = vec![Point::new(0, 0, 0, 0); n];

    for i in 0..n {
        cheeses[i] = Point::new(scan.token::<i64>(), scan.token::<i64>(), 0, 0);
    }

    let m = scan.token::<usize>();
    let mut peppers = vec![Point::new(0, 0, 0, 0); m];

    for i in 0..m {
        peppers[i] = Point::new(scan.token::<i64>(), scan.token::<i64>(), 0, 0);
    }

    peppers.sort();

    for i in 1..m {
        peppers[i].dx = peppers[i].x - peppers[0].x;
        peppers[i].dy = peppers[i].y - peppers[0].y;
    }

    let first_point = peppers.remove(0);
    peppers.sort_by(|a, b| {
        if a.dx * b.dy != a.dy * b.dx {
            return (a.dx * b.dy).cmp(&(a.dy * b.dx)).reverse();
        }

        if a.y != b.y {
            return a.y.cmp(&b.y);
        }

        a.x.cmp(&b.x)
    });
    peppers.insert(0, first_point);

    let mut stack = Vec::new();
    stack.push(peppers[0].clone());

    for i in 1..m {
        while stack.len() >= 2
            && calculate_ccw(
                stack.last().unwrap().clone(),
                next_to_top(&mut stack),
                peppers[i].clone(),
            ) >= 0
        {
            stack.pop();
        }

        stack.push(peppers[i].clone());
    }

    let mut convex_hull = vec![Point::new(0, 0, 0, 0); stack.len()];
    let mut index = stack.len() - 1;

    while !stack.is_empty() {
        convex_hull[index] = stack.pop().unwrap();
        index -= 1;
    }

    let mut right = 0;
    let mut idx = 0;
    let mut area = 0;
    let mut ret = 0;

    for i in 0..convex_hull.len() {
        let vec1 = convex_hull[i] - cheeses[0];
        let vec2 = convex_hull[idx] - cheeses[0];
        
        if vec1.x * vec2.y - vec1.y * vec2.x > 0 {
            idx = i;
        }
    }

    for left in 0..n {
        while calculate_ccw(
            cheeses[left],  
            convex_hull[idx],
            convex_hull[(idx + 1) % convex_hull.len()],
        ) < 0
        {
            idx = (idx + 1) % convex_hull.len();
        }

        while calculate_ccw(cheeses[left], cheeses[right], convex_hull[idx]) > 0 || left == right {
            area += cross(cheeses[left], cheeses[(right + n - 1) % n], cheeses[right]);
            right = (right + 1) % n;
        }

        ret = ret.max(area);
        area -= cross(
            cheeses[left],
            cheeses[(left + 1) % n],
            cheeses[(right + n - 1) % n],
        );
    }

    writeln!(out, "{ret}").unwrap();
}

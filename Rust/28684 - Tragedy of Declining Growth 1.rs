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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![Point::default(); n + 1];

    let mut y = 0;

    for i in 1..=n {
        let dy = scan.token::<i64>();
        y += dy;

        points[i] = Point::new(i as i64, y);
    }

    let mut convex_hull = Vec::new();

    for p in points.iter() {
        while convex_hull.len() >= 2
            && Point::ccw(
                convex_hull[convex_hull.len() - 2],
                convex_hull[convex_hull.len() - 1],
                *p,
            ) > 0
        {
            convex_hull.pop();
        }

        convex_hull.push(*p);
    }

    for i in 1..convex_hull.len() {
        for j in convex_hull[i - 1].x + 1..convex_hull[i].x {
            if Point::ccw(
                convex_hull[i - 1],
                Point::new(points[j as usize].x, points[j as usize].y + 1),
                convex_hull[i],
            ) >= 0
            {
                writeln!(out, "Provable").unwrap();
                return;
            }
        }
    }

    writeln!(out, "Not Provable").unwrap();
}

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

#[derive(Debug, Copy, Clone)]
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

fn calculate_ccw(p1: Point, p2: Point, p3: Point) -> i64 {
    let (x1, y1) = (p1.x as i64, p1.y as i64);
    let (x2, y2) = (p2.x as i64, p2.y as i64);
    let (x3, y3) = (p3.x as i64, p3.y as i64);
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

fn get_area(a: Point, b: Point, c: Point) -> i64 {
    ((a.x * b.y + b.x * c.y + c.x * a.y) - (a.y * b.x + b.y * c.x + c.y * a.x)).abs()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut points = vec![Point::new(0, 0, 0, 0); n];

        for i in 0..n {
            points[i] = Point::new(scan.token::<i64>(), scan.token::<i64>(), 0, 0);
        }

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

        for i in 1..n {
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
        stack.push(points[1].clone());

        for i in 2..n {
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

        if convex_hull.len() == 3 {
            let ret = get_area(convex_hull[0], convex_hull[1], convex_hull[2]);

            if ret % 2 == 0 {
                writeln!(out, "{}", ret / 2).unwrap();
            } else {
                writeln!(out, "{}.5", ret / 2).unwrap();
            }

            continue;
        }

        let n = convex_hull.len();
        let mut ret = 0;

        for i in 0..n - 2 {
            let mut p = (i + 1) % n;
            let mut q = (i + 3) % n;

            for j in i + 2..n {
                while (p + 1) % n != j
                    && get_area(convex_hull[i], convex_hull[p], convex_hull[j])
                        <= get_area(convex_hull[i], convex_hull[(p + 1) % n], convex_hull[j])
                {
                    p = (p + 1) % n;
                }

                while (q + 1) % n != i
                    && get_area(convex_hull[i], convex_hull[q], convex_hull[j])
                        <= get_area(convex_hull[i], convex_hull[(q + 1) % n], convex_hull[j])
                {
                    q = (q + 1) % n;
                }

                ret = ret.max(
                    get_area(convex_hull[i], convex_hull[p], convex_hull[j])
                        + get_area(convex_hull[j], convex_hull[q], convex_hull[i]),
                );
            }
        }

        if ret % 2 == 0 {
            writeln!(out, "{}", ret / 2).unwrap();
        } else {
            writeln!(out, "{}.5", ret / 2).unwrap();
        }
    }
}

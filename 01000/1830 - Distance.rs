use io::Write;
use std::{cmp, io, ops::Sub, str};

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

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
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
    let (x1, y1) = (p1.x, p1.y);
    let (x2, y2) = (p2.x, p2.y);
    let (x3, y3) = (p3.x, p3.y);

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

fn get_dist(p1: &Point, p2: &Point) -> i64 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn get_dist_square(p1: &Point, p2: &Point) -> i64 {
    (p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y)
}

fn convert(points: &mut Vec<Point>, n: usize, is_restore: bool) {
    for i in 0..n {
        let (x, y) = (points[i].x, points[i].y);
        points[i].x = x + y;
        points[i].y = x - y;

        if is_restore {
            points[i].x /= 2;
            points[i].y /= 2;
        }
    }
}

fn get_farthermost_euclidean(mut points: Vec<Point>, n: usize) -> i64 {
    points.sort_by(|a, b| {
        if a.dx * b.dy != a.dy * b.dx {
            return (a.dx * b.dy).cmp(&(a.dy * b.dx)).reverse();
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
        if a.dx * b.dy != a.dy * b.dx {
            return (a.dx * b.dy).cmp(&(a.dy * b.dx)).reverse();
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

    let mut max_dist = 0;
    let mut max_dist_points = (Point::new(0, 0, 0, 0), Point::new(0, 0, 0, 0));
    let mut c = 1;

    for a in 0..convex_hull.len() {
        let b = (a + 1) % convex_hull.len();

        loop {
            let d = (c + 1) % convex_hull.len();

            let zero = Point::new(0, 0, 0, 0);
            let ab = convex_hull[b].clone() - convex_hull[a].clone();
            let cd = convex_hull[d].clone() - convex_hull[c].clone();

            if calculate_ccw(zero, ab, cd) > 0 {
                c = d;
            } else {
                break;
            }
        }

        let dist = get_dist_square(&convex_hull[a], &convex_hull[c]);
        if dist > max_dist {
            max_dist = dist;
            max_dist_points = (convex_hull[a].clone(), convex_hull[c].clone());
        }
    }

    get_dist_square(&max_dist_points.0, &max_dist_points.1)
}

fn get_closest_euclidean(points: &Vec<Point>, x: i64, y: i64) -> i64 {
    if x == y {
        return i64::MAX;
    }

    let mid = (x + y) / 2;
    let left = get_closest_euclidean(points, x, mid);
    let right = get_closest_euclidean(points, mid + 1, y);
    let mut ans = cmp::min(left, right);

    let mut closest_points = Vec::new();

    for i in x..=y {
        let dist_x = points[i as usize].x - points[mid as usize].x;

        if dist_x * dist_x < ans {
            closest_points.push(points[i as usize].clone());
        }
    }

    closest_points.sort_by(|a, b| {
        return a.y.cmp(&b.y);
    });

    let size = closest_points.len();

    for i in 0..(size - 1) {
        for j in (i + 1)..size {
            let dist_y = closest_points[i as usize].y - closest_points[j as usize].y;

            if dist_y * dist_y < ans {
                let dist =
                    get_dist_square(&closest_points[i as usize], &closest_points[j as usize]);

                if dist < ans {
                    ans = dist;
                }
            } else {
                break;
            }
        }
    }

    ans
}

fn get_closest_manhattan(points: &Vec<Point>, x: i64, y: i64) -> i64 {
    if x == y {
        return i64::MAX;
    }

    let mid = (x + y) / 2;
    let left = get_closest_manhattan(points, x, mid);
    let right = get_closest_manhattan(points, mid + 1, y);
    let mut ans = cmp::min(left, right);

    let mut closest_points = Vec::new();

    for i in x..=y {
        let dist_x = (points[i as usize].x - points[mid as usize].x).abs();

        if dist_x < ans {
            closest_points.push(points[i as usize].clone());
        }
    }

    closest_points.sort_by(|a, b| {
        return a.y.cmp(&b.y);
    });

    let size = closest_points.len();

    for i in 0..(size - 1) {
        for j in (i + 1)..size {
            let dist_y = (closest_points[i as usize].y - closest_points[j as usize].y).abs();

            if dist_y < ans {
                let dist = get_dist(&closest_points[i as usize], &closest_points[j as usize]);

                if dist < ans {
                    ans = dist;
                }
            } else {
                break;
            }
        }
    }

    ans
}

fn get_farthermost_chebyshev(points: &Vec<Point>, n: usize) -> i64 {
    let mut x1 = i64::MAX;
    let mut x2 = i64::MIN;
    let mut y1 = i64::MAX;
    let mut y2 = i64::MIN;

    for i in 0..n {
        x1 = cmp::min(x1, points[i].x);
        x2 = cmp::max(x2, points[i].x);
        y1 = cmp::min(y1, points[i].y);
        y2 = cmp::max(y2, points[i].y);
    }

    cmp::max(x2 - x1, y2 - y1)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token();

    let mut points = Vec::new();

    for _ in 0..n {
        points.push(Point::new(scan.token(), scan.token(), 0, 0));
    }

    writeln!(out, "{}", get_farthermost_euclidean(points.clone(), n)).unwrap();
    points.sort();
    writeln!(out, "{}", get_closest_euclidean(&points, 0, n as i64 - 1)).unwrap();

    convert(&mut points, n, false);
    writeln!(out, "{}", get_farthermost_chebyshev(&points, n)).unwrap();
    convert(&mut points, n, true);
    points.sort();
    writeln!(out, "{}", get_closest_manhattan(&points, 0, n as i64 - 1)).unwrap();

    writeln!(out, "{}", get_farthermost_chebyshev(&points, n)).unwrap();
    convert(&mut points, n, false);
    points.sort();
    writeln!(
        out,
        "{}",
        get_closest_manhattan(&points, 0, n as i64 - 1) / 2
    )
    .unwrap();
}

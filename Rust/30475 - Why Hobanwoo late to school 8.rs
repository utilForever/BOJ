use io::Write;
use std::{cmp::Reverse, collections::BinaryHeap, io, str};

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

        (x2 - x1) * (y3 - y2) - (x3 - x2) * (y2 - y1)
    }
}

#[derive(Default, Clone)]
struct Planet {
    point: Point,
    time_convex_hull: i64,
    time_non_convex_hull: i64,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let mut planets = vec![Planet::default(); n + 1];

        for i in 1..=n {
            planets[i] = Planet {
                point: Point::new(scan.token::<i64>(), scan.token::<i64>(), i),
                time_convex_hull: scan.token::<i64>(),
                time_non_convex_hull: scan.token::<i64>(),
            };
        }

        let mut dp = vec![i64::MAX; n + 1];
        let mut cnt = vec![2; n + 1];

        dp[0] = 0;

        let mut upper = Vec::new();
        let mut lower = Vec::new();

        upper.push(Point::new(0, 0, 0));
        lower.push(Point::new(0, 0, 0));

        let mut pq_convex_hull: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        let mut pq_non_convex_hull: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

        pq_convex_hull.push(Reverse((0, 0)));

        for i in 1..=n {
            let (x, y, a, b) = (
                planets[i].point.x,
                planets[i].point.y,
                planets[i].time_convex_hull,
                planets[i].time_non_convex_hull,
            );

            while upper.len() >= 2
                && Point::ccw(
                    Point::new(x, y, i),
                    upper[upper.len() - 1],
                    upper[upper.len() - 2],
                ) < 0
            {
                let point = upper.pop().unwrap();

                cnt[point.idx] -= 1;

                if cnt[point.idx] == 0 {
                    pq_non_convex_hull.push(Reverse((dp[point.idx], point.idx)));
                }
            }

            while lower.len() >= 2
                && Point::ccw(
                    Point::new(x, y, i),
                    lower[lower.len() - 1],
                    lower[lower.len() - 2],
                ) > 0
            {
                let point = lower.pop().unwrap();

                cnt[point.idx] -= 1;

                if cnt[point.idx] == 0 {
                    pq_non_convex_hull.push(Reverse((dp[point.idx], point.idx)));
                }
            }

            while !pq_convex_hull.is_empty()
                && (cnt[pq_convex_hull.peek().unwrap().0 .1] == 0
                    || i as i64 - pq_convex_hull.peek().unwrap().0 .1 as i64 > m as i64)
            {
                pq_convex_hull.pop();
            }

            while !pq_non_convex_hull.is_empty()
                && i as i64 - pq_non_convex_hull.peek().unwrap().0 .1 as i64 > m as i64
            {
                pq_non_convex_hull.pop();
            }

            let val_convex_hull = if !pq_convex_hull.is_empty() {
                pq_convex_hull.peek().unwrap().0 .0 + a
            } else {
                i64::MAX
            };
            let val_non_convex_hull = if !pq_non_convex_hull.is_empty() {
                pq_non_convex_hull.peek().unwrap().0 .0 + b
            } else {
                i64::MAX
            };

            dp[i] = val_convex_hull.min(val_non_convex_hull);

            upper.push(Point::new(x, y, i));
            lower.push(Point::new(x, y, i));

            pq_convex_hull.push(Reverse((dp[i], i)));
        }

        writeln!(out, "{}", dp[n]).unwrap();
    }
}

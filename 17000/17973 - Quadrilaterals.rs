use io::Write;
use std::{cmp, io, str};

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Clone)]
struct Line {
    i: usize,
    j: usize,
    dx: i64,
    dy: i64,
}

impl Line {
    fn new(i: usize, j: usize, p: Point, q: Point) -> Self {
        Self {
            i,
            j,
            dx: q.x - p.x,
            dy: q.y - p.y,
        }
    }
}

fn calculate_ccw(p1: Point, p2: Point, p3: Point) -> i64 {
    let (x1, y1) = (p1.x as i64, p1.y as i64);
    let (x2, y2) = (p2.x as i64, p2.y as i64);
    let (x3, y3) = (p3.x as i64, p3.y as i64);

    (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)
}

// Reference: https://justicehui.github.io/hard-algorithm/2022/03/30/rotate-sweep-line/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![Point::new(); n];
    let mut positions = vec![0; n];

    for i in 0..n {
        (points[i].x, points[i].y) = (scan.token::<i64>(), scan.token::<i64>());
    }

    points.sort();

    for i in 0..n {
        positions[i] = i;
    }

    let mut lines = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            lines.push(Line::new(i, j, points[i], points[j]));
        }
    }

    lines.sort_by(|p, q| {
        let left = p.dy * q.dx;
        let right = q.dy * p.dx;

        if left > right {
            std::cmp::Ordering::Greater
        } else if left < right {
            std::cmp::Ordering::Less
        } else {
            (p.i, p.j).partial_cmp(&(q.i, q.j)).unwrap()
        }
    });

    let mut area_min = std::i64::MAX;
    let mut ret1 = 0;
    let mut ret2 = 0;

    for i in 0..lines.len() {
        let (mut u, mut v) = (lines[i].i, lines[i].j);
        positions.swap(u, v);
        points.swap(positions[u], positions[v]);

        if positions[u] > positions[v] {
            std::mem::swap(&mut u, &mut v);
        }

        let pos_min = cmp::min(positions[u], positions[v]);
        ret1 += pos_min * (n - pos_min - 2);

        for a in 1..=2 {
            for b in 1..=2 {
                if positions[u] as i64 - a as i64 >= 0 && positions[v] + b <= n - 1 {
                    let area1 = calculate_ccw(
                        points[positions[u]],
                        points[positions[v]],
                        points[positions[u] - a],
                    )
                    .abs();
                    let area2 = calculate_ccw(
                        points[positions[u]],
                        points[positions[v]],
                        points[positions[v] + b],
                    )
                    .abs();
                    let area_total = area1 + area2;

                    if area_min > area_total {
                        area_min = area_total;
                        ret2 = 0;
                    }

                    if area_min == area_total {
                        ret2 += 1;

                        let ccw1 = calculate_ccw(
                            points[positions[u] - a],
                            points[positions[v] + b],
                            points[positions[u]],
                        ) > 0;
                        let ccw2 = calculate_ccw(
                            points[positions[u] - a],
                            points[positions[v] + b],
                            points[positions[v]],
                        ) > 0;

                        if ccw1 == ccw2 {
                            ret2 += 1;
                        }
                    }
                }
            }
        }
    }

    writeln!(out, "{}", ret1 + ret2).unwrap();
}

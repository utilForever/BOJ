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
    flag: bool,
    start: Point,
    end: Point,
    direction: Point,
}

impl Line {
    fn new(i: usize, j: usize, flag: bool, p: Point, q: Point) -> Self {
        let mut line = Line {
            i,
            j,
            flag,
            start: p,
            end: q,
            direction: Point::new(),
        };

        if line.start > line.end {
            std::mem::swap(&mut line.start, &mut line.end);
        }

        line.direction = if line.flag {
            Point {
                x: line.end.x - line.start.x,
                y: line.end.y - line.start.y,
            }
        } else {
            let point = Point {
                x: line.end.x - line.start.x,
                y: line.end.y - line.start.y,
            };

            if point.y >= 0 {
                Point {
                    x: point.y,
                    y: -point.x,
                }
            } else {
                Point {
                    x: -point.y,
                    y: point.x,
                }
            }
        };

        line
    }
}

fn get_dist(p1: Point, p2: Point) -> f64 {
    (((p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y)) as f64).sqrt()
}

fn calculate_min_dist(p1: Point, p2: Point, p3: Point) -> f64 {
    let p_a = (p2.x - p1.x, p2.y - p1.y);
    let p_b = (p3.x - p2.x, p3.y - p2.y);
    let ccw = p_a.0 * p_b.1 - p_a.1 * p_b.0;

    ccw.abs() as f64 / get_dist(p1, p2)
}

// Reference: https://justicehui.github.io/hard-algorithm/2022/03/30/rotate-sweep-line/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    let mut points = vec![Point::new(); n];
    let mut indexes = vec![0; n];
    let mut positions = vec![0; n];

    for i in 0..n {
        (points[i].x, points[i].y) = (scan.token::<i64>(), scan.token::<i64>());
    }

    points.sort();

    for i in 0..n {
        indexes[i] = i;
        positions[i] = i;
    }

    let mut lines = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            lines.push(Line::new(i, j, true, points[i], points[j]));
            lines.push(Line::new(i, j, false, points[i], points[j]));
        }
    }

    lines.sort_by(|p, q| {
        let ret = p.direction.x * q.direction.y - p.direction.y * q.direction.x;

        if ret == 0 {
            (p.flag, p.start, p.end).cmp(&(q.flag, q.start, q.end))
        } else {
            if ret > 0 {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        }
    });

    let mut max = f64::MIN;

    for i in 0..lines.len() {
        if lines[i].flag {
            let (mut u, mut v) = (lines[i].i, lines[i].j);
            positions.swap(u, v);
            indexes.swap(positions[u], positions[v]);

            if positions[u] > positions[v] {
                std::mem::swap(&mut u, &mut v);
            }

            if positions[u] > 0 {
                max = max.max(calculate_min_dist(
                    lines[i].start,
                    lines[i].end,
                    points[indexes[positions[u] - 1]],
                ));
            }

            if positions[v] < n - 1 {
                max = max.max(calculate_min_dist(
                    lines[i].start,
                    lines[i].end,
                    points[indexes[positions[v] + 1]],
                ));
            }
        } else {
            if (positions[lines[i].i] as i64 - positions[lines[i].j] as i64).abs() == 1 {
                max = max.max(get_dist(points[lines[i].i], points[lines[i].j]));
            }
        }
    }

    writeln!(out, "{:.10}", max / 2.0).unwrap();
}

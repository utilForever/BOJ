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

fn get_area(p1: Point, p2: Point, p3: Point) -> i64 {
    let cross = (p2.x - p1.x) * (p3.y - p2.y) - (p2.y - p1.y) * (p3.x - p2.x);

    cross.abs()
}

// Reference: https://justicehui.github.io/hard-algorithm/2022/03/30/rotate-sweep-line/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

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

        let mut min = i64::MAX;
        let mut max = i64::MIN;

        let mut i = 0;
        let mut j = 0;

        while i < lines.len() {
            while j < lines.len() {
                if lines[i].dx * lines[j].dy != lines[i].dy * lines[j].dx {
                    break;
                }

                j += 1;
            }

            for k in i..j {
                let (mut u, mut v) = (lines[k].i, lines[k].j);
                positions.swap(u, v);
                points.swap(positions[u], positions[v]);

                if positions[u] > positions[v] {
                    std::mem::swap(&mut u, &mut v);
                }

                if positions[u] > 0 {
                    min = min.min(get_area(
                        points[positions[u]],
                        points[positions[v]],
                        points[positions[u] - 1],
                    ));
                    max = max.max(get_area(
                        points[positions[u]],
                        points[positions[v]],
                        points[0],
                    ));
                }

                if positions[v] < n - 1 {
                    min = min.min(get_area(
                        points[positions[u]],
                        points[positions[v]],
                        points[positions[v] + 1],
                    ));
                    max = max.max(get_area(
                        points[positions[u]],
                        points[positions[v]],
                        points[n - 1],
                    ));
                }
            }

            i = j;
        }

        writeln!(out, "{:.1} {:.1}", min as f64 / 2.0, max as f64 / 2.0).ok();
    }
}

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

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn get_dist(p1: &Point, p2: &Point) -> i64 {
    (p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y)
}

fn do_brute_force(points: &Vec<Point>, x: i64, y: i64) -> i64 {
    let mut ans = -1;

    for i in x..=(y - 1) {
        for j in (i + 1)..=y {
            let dist = get_dist(&points[i as usize], &points[j as usize]);

            if ans == -1 || dist < ans {
                ans = dist;
            }
        }
    }

    ans
}

fn get_closest(points: &Vec<Point>, x: usize, y: usize) -> i64 {
    let n = y - x + 1;

    if n <= 3 {
        return do_brute_force(points, x as i64, y as i64);
    }

    let mid = (x + y) / 2;
    let left = get_closest(points, x, mid);
    let right = get_closest(points, mid + 1, y);
    let mut ans = cmp::min(left, right);

    let mut closest_points = Vec::new();

    for i in x..=y {
        let dist_x = points[i].x - points[mid].x;

        if dist_x * dist_x < ans {
            closest_points.push(points[i].clone());
        }
    }

    closest_points.sort_by(|a, b| {
        return a.y.cmp(&b.y);
    });

    let size = closest_points.len() as i64;

    for i in 0..(size - 1) {
        for j in (i + 1)..size {
            let dist_y = closest_points[i as usize].y - closest_points[j as usize].y;

            if dist_y * dist_y < ans {
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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token();

    let mut points = Vec::new();

    for _ in 0..n {
        points.push(Point {
            x: scan.token(),
            y: scan.token(),
        });
    }

    points.sort_by(|a, b| {
        if a.x == b.x {
            return a.y.cmp(&b.y);
        }

        a.x.cmp(&b.x)
    });

    writeln!(out, "{}", get_closest(&points, 0, n - 1)).unwrap();
}

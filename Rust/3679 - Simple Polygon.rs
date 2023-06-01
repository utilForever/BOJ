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

#[derive(Clone, Default)]
struct Point {
    idx: usize,
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl Point {
    fn new(idx: usize, x: i64, y: i64, dx: i64, dy: i64) -> Self {
        Self { idx, x, y, dx, dy }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            idx: self.idx,
            x: self.x - other.x,
            y: self.y - other.y,
            dx: 0,
            dy: 0,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let c = scan.token::<i64>();

    for _ in 0..c {
        let n = scan.token::<usize>();
        let mut points = vec![Point::default(); n];

        for i in 0..n {
            points[i] = Point::new(i, scan.token::<i64>(), scan.token::<i64>(), 0, 0);
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

        let mut pos_reverse = points.len() - 1;

        for i in (1..=points.len() - 1).rev() {
            if points[i].dx as i64 * points[i - 1].dy as i64
                != points[i].dy as i64 * points[i - 1].dx as i64
            {
                pos_reverse = i;
                break;
            }
        }

        points[pos_reverse..].reverse();

        for point in points {
            write!(out, "{} ", point.idx).unwrap();
        }

        writeln!(out).unwrap();
    }
}

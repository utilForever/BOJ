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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut heights = vec![Point::new(0, 0); n];

    for i in 0..n {
        heights[i] = Point::new(i as i64 + 1, scan.token::<i64>());
    }

    let mut ret = 0;

    for i in 0..n {
        let mut left = Vec::new();
        let mut right = Vec::new();

        // Left side
        if i > 0 {
            left.push(i - 1);

            for j in (0..i - 1).rev() {
                if Point::ccw(heights[i], heights[*left.last().unwrap()], heights[j]) >= 0 {
                    continue;
                }

                left.push(j);
            }
        }

        // Right side
        if i < n - 1 {
            right.push(i + 1);

            for j in i + 2..n {
                if Point::ccw(heights[i], heights[*right.last().unwrap()], heights[j]) <= 0 {
                    continue;
                }

                right.push(j);
            }
        }

        ret = ret.max(left.len() + right.len());
    }

    writeln!(out, "{ret}").unwrap();
}

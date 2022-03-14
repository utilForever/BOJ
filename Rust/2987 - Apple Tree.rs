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

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn calculate_ccw(p1: Point, p2: Point, p3: Point) -> i64 {
    let (x1, y1) = (p1.x as i64, p1.y as i64);
    let (x2, y2) = (p2.x as i64, p2.y as i64);
    let (x3, y3) = (p3.x as i64, p3.y as i64);

    (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut triangle = vec![Point::new(0, 0); 3];
    for i in 0..3 {
        triangle[i].x = scan.token();
        triangle[i].y = scan.token();
    }

    if calculate_ccw(triangle[2], triangle[1], triangle[0]) > 0 {
        triangle.swap(2, 0);
    }

    writeln!(
        out,
        "{:.1}",
        calculate_ccw(triangle[0], triangle[1], triangle[2]) as f64 / 2.0
    )
    .unwrap();

    let n = scan.token::<usize>();
    let mut ans = 0;

    for _ in 0..n {
        let (x, y) = (scan.token::<i32>(), scan.token::<i32>());
        let p = Point::new(x, y);

        if calculate_ccw(p, triangle[1], triangle[0]) > 0 {
            continue;
        }
        if calculate_ccw(p, triangle[0], triangle[2]) > 0 {
            continue;
        }
        if calculate_ccw(p, triangle[2], triangle[1]) > 0 {
            continue;
        }

        ans += 1;
    }

    writeln!(out, "{}", ans).unwrap();
}

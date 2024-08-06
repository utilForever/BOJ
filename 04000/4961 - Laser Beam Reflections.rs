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

    fn dot(&self, other: &Self) -> i64 {
        self.x * other.x + self.y * other.y
    }

    fn cross(&self, other: &Self) -> i64 {
        self.x * other.y - self.y * other.x
    }

    fn ccw(p1: Point, p2: Point, p3: Point) -> i64 {
        let (x1, y1) = (p1.x, p1.y);
        let (x2, y2) = (p2.x, p2.y);
        let (x3, y3) = (p3.x, p3.y);
    
        let ret = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1);

        if ret > 0 {
            1
        } else if ret < 0 {
            -1
        } else {
            0
        }
    }

    fn intersect(mut a: Point, mut b: Point, mut c: Point, mut d: Point) -> bool {  
        let ab = Point::ccw(a, b, c) * Point::ccw(a, b, d);
        let cd = Point::ccw(c, d, a) * Point::ccw(c, d, b);
    
        if ab == 0 && cd == 0 {
            if a > b {
                let temp = b;
                b = a;
                a = temp;
            }
            if c > d {
                let temp = d;
                d = c;
                c = temp;
            }
    
            return c <= b && a <= d;
        }
    
        ab <= 0 && cd <= 0
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut mirrors = vec![(Point::default(), Point::default()); n];

        for i in 0..n {
            let (px, py, qx, qy) = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );
            mirrors[i] = (Point::new(px, py), Point::new(qx, qy));
        }

        let target = Point::new(scan.token::<i64>(), scan.token::<i64>());
        let generator = Point::new(scan.token::<i64>(), scan.token::<i64>());        
    }
}

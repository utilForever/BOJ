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

#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct Rect {
    points: [i32; 4],
    top_left: Point,
    top_right: Point,
    bottom_right: Point,
    bottom_left: Point,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (x1, y1, x2, y2) = (
        scan.token::<i32>(),
        scan.token::<i32>(),
        scan.token::<i32>(),
        scan.token::<i32>(),
    );
    let p = Rect {
        points: [x1, y1, x2, y2],
        top_left: Point { x: x1, y: y2 },
        top_right: Point { x: x2, y: y2 },
        bottom_right: Point { x: x2, y: y1 },
        bottom_left: Point { x: x1, y: y1 },
    };

    let (x1, y1, x2, y2) = (
        scan.token::<i32>(),
        scan.token::<i32>(),
        scan.token::<i32>(),
        scan.token::<i32>(),
    );
    let q = Rect {
        points: [x1, y1, x2, y2],
        top_left: Point { x: x1, y: y2 },
        top_right: Point { x: x2, y: y2 },
        bottom_right: Point { x: x2, y: y1 },
        bottom_left: Point { x: x1, y: y1 },
    };

    if p.points[0] > q.points[2]
        || p.points[1] > q.points[3]
        || p.points[2] < q.points[0]
        || p.points[3] < q.points[1]
    {
        writeln!(out, "NULL").unwrap();
    } else if p.top_left == q.bottom_right
        || p.top_right == q.bottom_left
        || p.bottom_right == q.top_left
        || p.bottom_left == q.top_right
    {
        writeln!(out, "POINT").unwrap();
    } else if p.points[0] == q.points[2]
        || p.points[1] == q.points[3]
        || p.points[2] == q.points[0]
        || p.points[3] == q.points[1]
    {
        writeln!(out, "LINE").unwrap();
    } else {
        writeln!(out, "FACE").unwrap();
    }
}

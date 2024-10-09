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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn calculate_ccw(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> i64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;

    let res = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1);
    if res > 0 {
        1
    } else if res < 0 {
        -1
    } else {
        0
    }
}

fn is_intersect(x: ((i64, i64), (i64, i64)), y: ((i64, i64), (i64, i64))) -> bool {
    let mut a = x.0;
    let mut b = x.1;
    let mut c = y.0;
    let mut d = y.1;

    let ab = calculate_ccw(a, b, c) * calculate_ccw(a, b, d);
    let cd = calculate_ccw(c, d, a) * calculate_ccw(c, d, b);

    if ab == 0 && cd == 0 {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        if c > d {
            std::mem::swap(&mut c, &mut d);
        }

        return c <= b && a <= d;
    }

    ab <= 0 && cd <= 0
}

// Reference: https://jason9319.tistory.com/358
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (x1, y1, x2, y2) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let (x3, y3, x4, y4) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    writeln!(
        out,
        "{}",
        if is_intersect(((x1, y1), (x2, y2)), ((x3, y3), (x4, y4))) {
            1
        } else {
            0
        }
    )
    .unwrap();
}

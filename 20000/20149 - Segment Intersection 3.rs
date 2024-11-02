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

fn is_intersect(
    x: ((i64, i64), (i64, i64)),
    y: ((i64, i64), (i64, i64)),
) -> (bool, Option<(f64, f64)>) {
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

        if c <= b && a <= d {
            if (a.0 - b.0) * (c.1 - d.1) == (a.1 - b.1) * (c.0 - d.0) {
                // Parallel
                if a == d && b > c {
                    return (true, Some((a.0 as f64, a.1 as f64)));
                } else if b == c && a < d {
                    return (true, Some((b.0 as f64, b.1 as f64)));
                } else {
                    return (true, None);
                }
            } else {
                // Meet at one point
                if a == c || a == d {
                    return (true, Some((a.0 as f64, a.1 as f64)));
                } else if b == c || b == d {
                    return (true, Some((b.0 as f64, b.1 as f64)));
                } else {
                    return (true, None);
                }
            }
        } else {
            return (false, None);
        }
    }

    if ab <= 0 && cd <= 0 {
        // Crossed
        let (x1, y1) = a;
        let (x2, y2) = b;
        let (x3, y3) = c;
        let (x4, y4) = d;

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denominator == 0 {
            return (false, None);
        }

        let numerator_x = (x1 * y2 - y1 * x2) as f64 * (x3 - x4) as f64
            - (x1 - x2) as f64 * (x3 * y4 - y3 * x4) as f64;
        let numerator_y = (x1 * y2 - y1 * x2) as f64 * (y3 - y4) as f64
            - (y1 - y2) as f64 * (x3 * y4 - y3 * x4) as f64;

        let x = numerator_x / denominator as f64;
        let y = numerator_y / denominator as f64;

        return (true, Some((x, y)));
    } else {
        return (false, None);
    }
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

    let ret = is_intersect(((x1, y1), (x2, y2)), ((x3, y3), (x4, y4)));

    if ret.0 {
        writeln!(out, "1").unwrap();

        if let Some(point) = ret.1 {
            writeln!(out, "{:.9} {:.9}", point.0, point.1).unwrap();
        }
    } else {
        writeln!(out, "0").unwrap();
    }
}

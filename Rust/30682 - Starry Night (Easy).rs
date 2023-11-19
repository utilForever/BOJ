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

fn sum(n: i64) -> i64 {
    n * (n + 1) * (n + 1) / 2 - n * (n + 1) * (2 * n + 1) / 6
}

fn calculate(x: i64, y: i64, x1: i64, y1: i64, x2: i64, y2: i64, p: i64) -> i64 {
    if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
        let a = y + p - 1 - y2;
        let b = x1 - x + p - 1;
        let c = a + b - p;

        sum(p) - sum(a.max(0)) - sum(b.max(0)) + sum(c.max(0))
    } else if x < x1 || y > y2 {
        0
    } else {
        let p_sub = (p - (x - x2).max(0) - (y1 - y).max(0)).max(0);

        if p_sub == 0 {
            0
        } else {
            calculate(x.min(x2), y.max(y1), x1, y1, x2, y2, p_sub)
        }
    }
} 

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut stars = vec![(0, 0, 0); n];

    for i in 0..n {
        stars[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    let (a, b, c, d) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut ret = 0;

    for star in stars {
        let (x, y, p) = star;

        if a <= x - p && x + p <= c && b <= y - p && y + p <= d {
            ret += p + 2 * p * (p - 1) * p - 2 * (p - 1) * p * (2 * p - 1) / 3;
        } else {
            let mut val = 0;

            if a <= x && x <= c && b <= y && y <= d {
                val += p;
            }

            val += calculate(x - a - 1, y - b, 0, 0, c - a, d - b, p - 1);
            val += calculate(d - y - 1, x - a, 0, 0, d - b, c - a, p - 1);
            val += calculate(c - x - 1, d - y, 0, 0, c - a, d - b, p - 1);
            val += calculate(y - b - 1, c - x, 0, 0, d - b, c - a, p - 1);

            ret += val;
        }
    }

    writeln!(out, "{ret}").unwrap();
}

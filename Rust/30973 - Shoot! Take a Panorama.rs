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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let (x_min, y_min, x_max, y_max) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    let mut sum_l = 0;
    let mut sum_lx = 0;
    let mut sum_lxx = 0;
    let mut sum_ly = 0;
    let mut sum_lyy = 0;

    for _ in 0..n {
        let (x, y, l) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        sum_l += l;
        sum_lx += x * l;
        sum_lxx += x * x * l;
        sum_ly += y * l;
        sum_lyy += y * y * l;
    }

    let calculate_cost = |x: i64, y: i64| -> i64 {
        x * x * sum_l - 2 * x * sum_lx + sum_lxx + y * y * sum_l - 2 * y * sum_ly + sum_lyy
    };

    let mut ret = i64::MAX;

    for x in (x_min - 1)..=(x_max + 1) {
        ret = ret.min(calculate_cost(x, y_min - 1));
    }

    for y in (y_min - 1)..=(y_max + 1) {
        ret = ret.min(calculate_cost(x_min - 1, y));
    }

    for x in (x_min - 1)..=(x_max + 1) {
        ret = ret.min(calculate_cost(x, y_max + 1));
    }

    for y in (y_min - 1)..=(y_max + 1) {
        ret = ret.min(calculate_cost(x_max + 1, y));
    }

    writeln!(out, "{ret}").unwrap();
}

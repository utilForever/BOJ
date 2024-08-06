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

    let (n, b) = (scan.token::<usize>(), scan.token::<i64>());
    let mut x = vec![0; n];
    let mut y = vec![0; n];

    for i in 0..n {
        (x[i], y[i]) = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut left = -1e9;
    let mut right = 1e9;

    while left < right {
        let mid = (left + right) / 2.0;
        let mut sum = 0.0;

        for i in 0..n {
            sum += -4.0 * x[i] as f64 * (y[i] as f64 - mid * x[i] as f64 - b as f64).powi(3);
        }

        if sum < 0.0 {
            left = mid;
        } else {
            right = mid;
        }

        if (left - right).abs() < 1e-9 {
            writeln!(out, "{:.9}", left).unwrap();
            break;
        }
    }
}

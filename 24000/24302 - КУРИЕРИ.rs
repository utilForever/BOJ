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

    let (mut dist1, mut dist2) = (scan.token::<i64>(), scan.token::<i64>());
    let calculate_price1 = |dist: i64| -> f64 {
        if dist <= 5 {
            4.0
        } else if dist <= 10 {
            7.0
        } else if dist <= 20 {
            12.0
        } else if dist <= 30 {
            17.0
        } else {
            0.57 * dist as f64
        }
    };
    let calculate_price2 = |dist: i64| -> f64 {
        let rest = dist as f64;

        if dist <= 2 {
            0.9 + 0.9 * rest
        } else if dist <= 5 {
            1.0 + 0.85 * rest
        } else if dist <= 20 {
            1.25 + 0.8 * rest
        } else if dist <= 40 {
            3.25 + 0.7 * rest
        } else {
            9.25 + 0.55 * rest
        }
    };

    dist1 /= 1000;
    dist2 /= 1000;

    let ret1 = calculate_price1(dist1).min(calculate_price2(dist1));
    let ret2 = calculate_price1(dist2).min(calculate_price2(dist2));

    writeln!(out, "{:.2}", ret1 + ret2).unwrap();
}

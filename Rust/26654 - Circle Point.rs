use io::Write;
use std::{f64::consts::PI, io, str};

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

// Reference: https://www.calculat.org/en/area-perimeter/polygon/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, a, b) = (
            scan.token::<i64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );

        let radius_a = (a / (n as f64 * (PI / n as f64).cos() * (PI / n as f64).sin())).sqrt();
        let radius_b = (b / PI).sqrt();

        if radius_a <= radius_b {
            writeln!(out, "{}", n).unwrap();
        } else {
            let mut left = 0;
            let mut right = n / 2;

            while left <= right {
                let mid = (left + right) / 2;
                let ret = radius_a * (mid as f64 * (PI / n as f64)).sin();

                if ret <= radius_b {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }

            writeln!(out, "{left}").unwrap();
        }
    }
}

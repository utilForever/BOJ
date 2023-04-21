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

fn num_digits(n: u64, b: u64) -> u32 {
    (n as f64).log(b as f64).ceil() as u32
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<u64>(), scan.token::<u32>());
    let mut ret = 0;

    if n == 1 {
        writeln!(out, "1").unwrap();
        return;
    }

    for i in 0..num_digits(n, 2) {
        let mut left = 2_u64.pow(i);
        let mut right = 2_u64.pow(i + 1) - 1;

        while left <= right {
            let mid = (left + right) / 2;
            let val = (right ^ mid) * (2_u64.pow(k) - 1);

            if mid >= val {
                if mid <= n {
                    ret += right.min(n) - mid + 1;
                }

                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

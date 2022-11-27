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

    let (n, l) = (scan.token::<usize>(), scan.token::<f64>());
    let mut pos = vec![0.0; n];
    let mut weight = vec![0.0; n];

    for i in 0..n {
        pos[i] = scan.token::<f64>();
    }

    for i in 0..n {
        weight[i] = scan.token::<f64>();
    }

    let mut left = 0.0;
    let mut right = l;

    while left <= right {
        let mid = (left + right) / 2.0;
        let mut left_weight = 0.0;
        let mut right_weight = 0.0;

        for i in 0..n {
            if pos[i] < mid {
                left_weight += weight[i] * (mid - pos[i]) / l;
            } else {
                right_weight += weight[i] * (pos[i] - mid) / l;
            }
        }

        if left_weight < right_weight {
            left = mid + 0.0000001;
        } else {
            right = mid - 0.0000001;
        }
    }

    writeln!(out, "{:.10}", left).unwrap();
}

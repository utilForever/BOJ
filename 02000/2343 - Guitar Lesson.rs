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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut lengths = vec![0; n];

    for i in 0..n {
        lengths[i] = scan.token::<i64>();
    }

    let mut left = 1;
    let mut right = 1_000_000_000;

    while left <= right {
        let mid = (left + right) / 2;
        let mut is_satisfied = true;

        let mut sum = 0;
        let mut cnt = 1;

        for length in lengths.iter() {
            if *length > mid {
                is_satisfied = false;
                break;
            }

            if sum + length <= mid {
                sum += length;
            } else {
                cnt += 1;
                sum = *length;
            }
        }

        if cnt > m {
            is_satisfied = false;
        }

        if is_satisfied {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    writeln!(out, "{left}").unwrap();
}

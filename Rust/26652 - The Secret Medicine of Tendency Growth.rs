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
    let mut levels = vec![0; n];
    let mut items = vec![0; n];

    for i in 0..n {
        levels[i] = scan.token::<i64>();
    }

    for i in 0..n {
        items[i] = scan.token::<i64>();
    }

    let level_max = *levels.iter().max().unwrap();

    // First parametric search to get max level by items
    for i in 0..n {
        let mut left = 0;
        let mut right = 2_000_000_007;

        while left <= right {
            let mid = (left + right) / 2;
            let sum = mid * (mid + 1) / 2 - (levels[i] - 1) * levels[i] / 2;

            if sum > items[i] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        levels[i] = left;
    }

    // Second parametric search to get max level by m
    let mut left = 0;
    let mut right = 1_000_000_000_000;

    while left <= right {
        let mid = (left + right) / 2;
        let mut sum = 0;

        for i in 0..n {
            if levels[i] >= mid {
                continue;
            }

            sum += mid - levels[i];

            if sum > m {
                break;
            }
        }

        if sum > m {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    writeln!(out, "{}", if left - 1 < level_max { -1 } else { left - 1 }).unwrap();
}

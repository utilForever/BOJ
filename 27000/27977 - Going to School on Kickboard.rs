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

    let (l, n, k) = (
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut stations = vec![0; n + 2];

    for i in 1..=n {
        stations[i] = scan.token::<i64>();
    }

    stations[n + 1] = l;

    let mut left = 0;
    let mut right = l;

    for i in 1..=n + 1 {
        left = left.max(stations[i] - stations[i - 1]);
    }

    while left <= right {
        let mid = (left + right) / 2;
        let mut dist = 0;
        let mut cnt = 0;

        for i in 0..n + 1 {
            if dist + stations[i + 1] - stations[i] > mid {
                cnt += 1;
                dist = 0;
            }

            dist += stations[i + 1] - stations[i];
        }

        if cnt > k {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    writeln!(out, "{left}").unwrap();
}

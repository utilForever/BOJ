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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut blackholes = vec![0; n];
    let mut asteroids = vec![(0, 0); m];

    for i in 0..n {
        blackholes[i] = scan.token::<i64>();
    }

    for i in 0..m {
        asteroids[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    blackholes.sort();
    asteroids.sort_by_key(|asteroid| asteroid.0);

    let mut left = 0;
    let mut right = 200_000_000;

    while left < right {
        let mid = (left + right) / 2;
        let mut idx = 0;
        let mut is_satisfy = true;

        for &(a, w) in asteroids.iter() {
            while idx < n - 1 {
                if (blackholes[idx] - a).abs() > (blackholes[idx + 1] - a).abs() {
                    idx += 1;
                } else {
                    break;
                }
            }

            if (blackholes[idx] - a).abs() * w > mid {
                is_satisfy = false;
                break;
            }
        }

        if is_satisfy {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    writeln!(out, "{left}").unwrap();
}

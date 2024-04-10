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

    let (s, c) = (scan.token::<usize>(), scan.token::<i64>());
    let mut lengths = vec![0; s];

    for i in 0..s {
        lengths[i] = scan.token::<i64>();
    }

    let mut left = 0;
    let mut right = lengths.iter().max().unwrap() + 1;

    while left + 1 < right {
        let mid = (left + right) / 2;
        let mut cnt = 0;

        for i in 0..s {
            cnt += lengths[i] / mid;
        }

        if cnt >= c {
            left = mid;
        } else {
            right = mid;
        }
    }

    writeln!(out, "{}", lengths.iter().sum::<i64>() - left * c).unwrap();
}

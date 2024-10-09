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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (m, n) = (scan.token::<i64>(), scan.token::<usize>());
    let mut snacks = vec![0; n];

    for i in 0..n {
        snacks[i] = scan.token::<i64>();
    }

    let mut left = 1;
    let mut right = 1_000_000_000_000;
    let mut ret = 0;

    while left <= right {
        let mid = (left + right) / 2;
        let mut sum = 0;

        for i in 0..n {
            sum += snacks[i] / mid;
        }

        if sum >= m {
            ret = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}

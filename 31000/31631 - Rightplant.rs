use io::Write;
use std::{collections::VecDeque, io, str};

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

    let n = scan.token::<i64>();
    let mut left = VecDeque::new();
    let mut right = VecDeque::new();

    for i in (1..=n).rev() {
        if i % 2 == n % 2 {
            if (n - i) % 4 == 0 {
                right.push_front(i);
            } else {
                left.push_back(i);
            }
        } else {
            if left.len() < right.len() {
                left.push_back(i);
            } else {
                right.push_front(i);
            }
        }
    }

    for val in left {
        write!(out, "{val} ").unwrap();
    }

    for val in right {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}

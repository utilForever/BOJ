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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn check(powers: &Vec<i64>, n: usize, k: i64, mid: usize) -> bool {
    let mut curr = n;

    for _ in 0..k {
        if curr as i64 - mid as i64 <= 1 {
            return true;
        }

        let mut next = curr - mid;

        while powers[next] == 0 {
            next += 1;
        }

        if next == curr {
            return false;
        }

        curr = next;
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut powers = vec![0; n + 1];

    for i in 1..=n {
        powers[i] = scan.token::<i64>();
    }

    let mut left = 1;
    let mut right = n;
    let mut ret = n;

    while left <= right {
        let mid = (left + right) / 2;

        if check(&powers, n, k, mid) {
            ret = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}

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

fn f(x: i128, left: &Vec<i128>, right: &Vec<i128>) -> bool {
    let mut value = x;

    for i in 0..left.len() {
        let l = left[i];
        let r = right[i];

        if l <= x && x <= r {
            value = value ^ (((x | l) + (x & r) * (l ^ r)) % (2_i128.pow(64)));
        }
    }

    value >= 0x0123456789ABCDEF
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut left = vec![0; n];
    let mut right = vec![0; n];

    for i in 0..n {
        left[i] = scan.token::<i128>();
    }

    for i in 0..n {
        right[i] = scan.token::<i128>();
    }

    let mut l = 0;
    let mut r = 10_i128.pow(18);

    while l <= r {
        let m = (l + r) / 2;

        if !f(m, &left, &right) && f(m + 1, &left, &right) {
            writeln!(out, "{m}").unwrap();
            return;
        } else if f(m, &left, &right) {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    writeln!(out, "-1").unwrap();
}

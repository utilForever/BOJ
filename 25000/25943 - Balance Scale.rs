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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let mut left = 0;
    let mut right = 0;

    for i in 0..n {
        if i == 0 {
            left += scan.token::<i64>();
        } else if i == 1 {
            right += scan.token::<i64>();
        } else {
            if left > right {
                right += scan.token::<i64>();
            } else {
                left += scan.token::<i64>();
            }
        }
    }

    let mut diff = (left - right).abs();
    let mut ret = 0;

    ret += diff / 100;
    diff %= 100;

    ret += diff / 50;
    diff %= 50;

    ret += diff / 20;
    diff %= 20;

    ret += diff / 10;
    diff %= 10;

    ret += diff / 5;
    diff %= 5;

    ret += diff / 2;
    diff %= 2;

    ret += diff;

    writeln!(out, "{ret}").unwrap();
}

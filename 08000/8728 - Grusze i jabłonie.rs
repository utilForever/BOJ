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

    let n = scan.token::<usize>();
    let mut trees = vec![0; n];

    for i in 0..n {
        trees[i] = scan.token::<i64>();
    }

    let mut apple_left = n - 1;
    let mut apple_right = 0;
    let mut pear_left = n - 1;
    let mut pear_right = 0;

    for i in 0..n {
        if trees[i] == 1 {
            apple_left = apple_left.min(i);
            apple_right = apple_right.max(i);
        } else {
            pear_left = pear_left.min(i);
            pear_right = pear_right.max(i);
        }
    }

    let ret = (apple_right as i64 - pear_left as i64)
        .abs()
        .max((pear_right as i64 - apple_left as i64).abs());

    writeln!(out, "{ret}").unwrap();
}

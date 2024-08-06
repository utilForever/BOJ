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

    let s1 = scan.token::<String>();
    let s2 = scan.token::<String>();
    let s3 = scan.token::<String>();

    let mut ret = 0;

    if let Some(num1) = s1.parse::<i64>().ok() {
        ret = num1 + 3;
    }

    if let Some(num2) = s2.parse::<i64>().ok() {
        ret = num2 + 2;
    }

    if let Some(num3) = s3.parse::<i64>().ok() {
        ret = num3 + 1;
    }

    if ret % 3 == 0 && ret % 5 == 0 {
        writeln!(out, "FizzBuzz").unwrap();
    } else if ret % 3 == 0 {
        writeln!(out, "Fizz").unwrap();
    } else if ret % 5 == 0 {
        writeln!(out, "Buzz").unwrap();
    } else {
        writeln!(out, "{ret}").unwrap();
    }
}

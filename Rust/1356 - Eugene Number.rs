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

    let n = scan.token::<i64>();
    let mut val = n;
    let mut digits = Vec::new();

    while val > 0 {
        digits.push(val % 10);
        val /= 10;
    }

    digits.reverse();

    let mut left = 1;
    let mut ret = false;

    for i in 0..digits.len() - 1 {
        left *= digits[i];

        let right = if i + 1 >= digits.len() {
            0
        } else {
            digits[i + 1..].iter().fold(1, |acc, x| acc * x)
        };

        if left == right {
            ret = true;
            break;
        }
    }

    writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
}

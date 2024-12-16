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

    let (s, e) = (scan.token::<i64>(), scan.token::<i64>());

    for num in s..=e {
        let mut digits = Vec::new();
        let mut val = num;

        while val > 0 {
            digits.push(val % 10);
            val /= 10;
        }

        let mut is_palindrome = true;

        for i in 0..(digits.len() / 2) {
            if digits[i] != digits[digits.len() - i - 1] {
                is_palindrome = false;
                break;
            }
        }

        if is_palindrome {
            writeln!(out, "Palindrome!").unwrap();
        } else {
            writeln!(out, "{num}").unwrap();
        }
    }
}

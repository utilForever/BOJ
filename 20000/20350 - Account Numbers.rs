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

    let s = scan.token::<String>();
    let s = format!("{}{}", &s[4..], &s[..4]);
    let mut rem = 0;

    for c in s.chars() {
        if c.is_digit(10) {
            rem = (rem * 10 + c.to_digit(10).unwrap()) % 97;
        } else {
            let num = c as u8 - b'A' + 10;

            for digit in num.to_string().chars() {
                rem = (rem * 10 + digit.to_digit(10).unwrap()) % 97;
            }
        }
    }

    writeln!(out, "{}", if rem == 1 { "correct" } else { "incorrect" }).unwrap();
}

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

    let isbn = scan.token::<String>();
    let isbn = isbn.chars().collect::<Vec<_>>();

    for i in 0..=9 {
        let mut val = 0;

        for j in 0..isbn.len() - 1 {
            val += if isbn[j].is_digit(10) {
                let num = isbn[j].to_digit(10).unwrap() as i64;

                if j % 2 == 0 {
                    num
                } else {
                    num * 3
                }
            } else {
                if j % 2 == 0 {
                    i
                } else {
                    i * 3
                }
            }
        }

        let m = if val % 10 == 0 { 0 } else { 10 - (val % 10) };

        if m == isbn.last().unwrap().to_digit(10).unwrap() as i64 {
            writeln!(out, "{i}").unwrap();
            return;
        }
    }
}

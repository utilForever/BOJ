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

    let a = scan.token::<String>();
    let a = a.chars().collect::<Vec<_>>();
    let op = scan.token::<String>();
    let b = scan.token::<String>();
    let b = b.chars().collect::<Vec<_>>();

    if op == "+" {
        if a.len() == b.len() {
            write!(out, "2").unwrap();

            for _ in 0..a.len() - 1 {
                write!(out, "0").unwrap();
            }
        } else {
            let (a, b) = if a.len() > b.len() {
                (a, b)
            } else {
                (b, a)
            };

            write!(out, "1").unwrap();

            for _ in 0..a.len() - b.len() - 1 {
                write!(out, "0").unwrap();
            }

            write!(out, "1").unwrap();

            for _ in 0..b.len() - 1 {
                write!(out, "0").unwrap();
            }
        }
    } else {
        write!(out, "1").unwrap();

        for _ in 1..a.len() + b.len() - 1 {
            write!(out, "0").unwrap();
        }
    }
}

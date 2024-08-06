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

    let (a, b) = (scan.token::<String>(), scan.token::<String>());
    let a = a.chars().collect::<Vec<_>>();
    let b = b.chars().collect::<Vec<_>>();

    // a & b
    for i in 0..a.len() {
        if a[i] == '1' && b[i] == '1' {
            write!(out, "1").unwrap();
        } else {
            write!(out, "0").unwrap();
        }
    }

    writeln!(out).unwrap();

    // a | b
    for i in 0..a.len() {
        if a[i] == '1' || b[i] == '1' {
            write!(out, "1").unwrap();
        } else {
            write!(out, "0").unwrap();
        }
    }

    writeln!(out).unwrap();

    // a ^ b
    for i in 0..a.len() {
        if a[i] == b[i] {
            write!(out, "0").unwrap();
        } else {
            write!(out, "1").unwrap();
        }
    }

    writeln!(out).unwrap();

    // ~a
    for i in 0..a.len() {
        if a[i] == '1' {
            write!(out, "0").unwrap();
        } else {
            write!(out, "1").unwrap();
        }
    }

    writeln!(out).unwrap();

    // ~b
    for i in 0..b.len() {
        if b[i] == '1' {
            write!(out, "0").unwrap();
        } else {
            write!(out, "1").unwrap();
        }
    }

    writeln!(out).unwrap();
}

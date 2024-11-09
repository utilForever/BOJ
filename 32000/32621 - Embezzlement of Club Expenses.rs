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
    let partials = s.split('+').collect::<Vec<&str>>();

    if partials.len() != 2 {
        writeln!(out, "No Money").unwrap();
        return;
    }

    let (left, right) = (partials[0], partials[1]);

    if left.is_empty() || right.is_empty() {
        writeln!(out, "No Money").unwrap();
        return;
    }

    if left.starts_with('0') || right.starts_with('0') {
        writeln!(out, "No Money").unwrap();
        return;
    }

    let left = left.parse::<i64>();
    let right = right.parse::<i64>();

    if left.is_err() || right.is_err() {
        writeln!(out, "No Money").unwrap();
        return;
    }

    let left = left.unwrap();
    let right = right.unwrap();

    if left != right {
        writeln!(out, "No Money").unwrap();
        return;
    }

    writeln!(out, "CUTE").unwrap();
}

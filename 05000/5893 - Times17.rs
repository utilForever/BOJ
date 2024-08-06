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

    let mut n = scan.token::<String>();
    let m = n.clone();

    n.push_str("0000");

    let mut n = n.bytes().collect::<Vec<_>>();
    let mut m = m.bytes().collect::<Vec<_>>();
    let mut idx = 0;
    let mut ret = String::new();

    while !m.is_empty() {
        let val = m.pop().unwrap() + n.pop().unwrap() - 48 * 2 + idx;

        idx = val / 2;
        ret.push_str(&(val % 2).to_string());
    }

    while !n.is_empty() {
        let val = n.pop().unwrap() - 48 + idx;

        idx = val / 2;
        ret.push_str(&(val % 2).to_string());
    }

    if idx != 0 {
        ret.push_str(&"1".to_string());
    }

    let ret = ret.chars().rev().collect::<String>();

    writeln!(out, "{ret}").unwrap();
}

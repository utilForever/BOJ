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
    let b = scan.token::<String>();
    let a = a.chars().collect::<Vec<_>>();
    let b = b.chars().collect::<Vec<_>>();
    let mut ret = String::new();

    if a.len() > b.len() {
        for i in 0..(a.len() - b.len()) {
            ret.push(a[i]);
        }

        for i in 0..b.len() {
            let val = (a[i + a.len() - b.len()].to_digit(10).unwrap() + b[i].to_digit(10).unwrap())
                .to_string();
            ret.push_str(val.as_str());
        }
    } else if a.len() < b.len() {
        for i in 0..(b.len() - a.len()) {
            ret.push(b[i]);
        }

        for i in 0..a.len() {
            let val = (a[i].to_digit(10).unwrap() + b[i + b.len() - a.len()].to_digit(10).unwrap())
                .to_string();
            ret.push_str(val.as_str());
        }
    } else {
        for i in 0..a.len() {
            let val = (a[i].to_digit(10).unwrap() + b[i].to_digit(10).unwrap()).to_string();
            ret.push_str(val.as_str());
        }
    }

    writeln!(out, "{ret}").unwrap();
}

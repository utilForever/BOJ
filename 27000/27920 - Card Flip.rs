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

    let n = scan.token::<usize>();
    let mut odds = Vec::new();
    let mut evens = Vec::new();

    for i in (1..=n).step_by(2) {
        odds.push(i);
    }

    odds.reverse();

    for i in (2..=n).step_by(2) {
        evens.push(i);
    }

    let mut b = odds.clone();
    b.append(&mut evens);

    let mut c = vec![0; n];
    let mut idx = 1;

    for i in (0..n).step_by(2).rev() {
        c[i] = idx;
        idx += 1;
    }

    for i in (1..n).step_by(2) {
        c[i] = idx;
        idx += 1;
    }

    writeln!(out, "YES").unwrap();

    for val in b {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();

    for val in c {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}

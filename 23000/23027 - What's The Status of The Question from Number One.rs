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
    let mut s = s.chars().collect::<Vec<_>>();

    if s.contains(&'A') {
        for i in 0..s.len() {
            if s[i] == 'B' || s[i] == 'C' || s[i] == 'D' || s[i] == 'F' {
                s[i] = 'A';
            }
        }
    } else if s.contains(&'B') {
        for i in 0..s.len() {
            if s[i] == 'C' || s[i] == 'D' || s[i] == 'F' {
                s[i] = 'B';
            }
        }
    } else if s.contains(&'C') {
        for i in 0..s.len() {
            if s[i] == 'D' || s[i] == 'F' {
                s[i] = 'C';
            }
        }
    } else {
        s.iter_mut().for_each(|x| *x = 'A');
    }

    writeln!(out, "{}", s.iter().collect::<String>()).unwrap();
}

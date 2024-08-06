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

    let _ = scan.token::<usize>();
    let s = scan.token::<String>();
    let mut s = s.chars().collect::<Vec<_>>();
    let mut idx = 0;

    while idx < s.len() {
        if idx + 1 < s.len() && s[idx] == s[idx + 1] {
            s[idx] = s[idx].to_ascii_uppercase();
            s[idx + 1] = s[idx + 1].to_ascii_uppercase();
            idx += 2;
        } else {
            idx += 1;
        }
    }

    writeln!(out, "{}", s.iter().collect::<String>()).unwrap();
}

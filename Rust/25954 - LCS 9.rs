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

// Reference: https://qwerasdfzxcl.tistory.com/28
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>();
    let t = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();

    let mut h = vec![vec![0; t.len() + 1]; s.len() + 1];
    let mut v = vec![vec![0; t.len() + 1]; s.len() + 1];

    for j in 1..=t.len() {
        h[0][j] = j;
    }

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i - 1] == t[j - 1] {
                h[i][j] = v[i][j - 1];
                v[i][j] = h[i - 1][j];
            } else {
                h[i][j] = v[i][j - 1].max(h[i - 1][j]);
                v[i][j] = v[i][j - 1].min(h[i - 1][j]);
            }
        }
    }

    let mut ret = 0;

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            ret += (j - h[i][j]) * (t.len() - j + 1);
        }
    }

    writeln!(out, "{ret}").unwrap();
}

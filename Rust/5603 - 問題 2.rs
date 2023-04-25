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

    let (n, mut s) = (scan.token::<i64>(), scan.token::<String>());

    for _ in 0..n {
        let chars = s.chars().collect::<Vec<_>>();
        let mut word = chars[0];
        let mut cnt = 1;
        let mut ret = String::new();

        for i in 1..chars.len() {
            if chars[i] == word {
                cnt += 1;
            } else {
                ret.push_str(&cnt.to_string());
                ret.push(word);
                word = chars[i];
                cnt = 1;
            }
        }

        ret.push_str(&cnt.to_string());
        ret.push(word);

        s = ret;
    }

    writeln!(out, "{s}").unwrap();
}

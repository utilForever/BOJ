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
    let s = s.chars().collect::<Vec<_>>();
    let mut ret = String::from("z".repeat(s.len()));

    for i in 1..s.len() - 1 {
        for j in i + 1..s.len() {
            let a = s[0..i].iter().collect::<String>();
            let b = s[i..j].iter().collect::<String>();
            let c = s[j..s.len()].iter().collect::<String>();

            let a = a.chars().rev().collect::<String>();
            let b = b.chars().rev().collect::<String>();
            let c = c.chars().rev().collect::<String>();

            let combined = a + &b + &c;
            ret = ret.min(combined);
        }
    }

    writeln!(out, "{ret}").unwrap();
}

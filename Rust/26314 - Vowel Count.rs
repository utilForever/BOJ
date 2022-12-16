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

    let n = scan.token::<i64>();

    for _ in 0..n {
        let name = scan.token::<String>();
        let str_name = name.chars().collect::<Vec<_>>();
        let mut cnt_vowel = 0;
        let mut cnt_consonant = 0;

        for c in str_name.iter() {
            if c == &'a' || c == &'e' || c == &'i' || c == &'o' || c == &'u' {
                cnt_vowel += 1;
            } else {
                cnt_consonant += 1;
            }
        }

        writeln!(out, "{name}").unwrap();
        writeln!(out, "{}", if cnt_vowel > cnt_consonant { 1 } else { 0 }).unwrap();
    }
}

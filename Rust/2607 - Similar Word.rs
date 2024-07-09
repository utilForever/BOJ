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
    let word_first = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut alphabet_first = [0; 26];

    for c in word_first.iter() {
        alphabet_first[(*c as u8 - b'A') as usize] += 1;
    }

    let mut ret = 0;

    for _ in 1..n {
        let word = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut alphabet = [0i64; 26];

        for c in word.iter() {
            alphabet[(*c as u8 - b'A') as usize] += 1;
        }

        let mut diff = 0;

        for i in 0..26 {
            diff += (alphabet_first[i] - alphabet[i]).abs();
        }

        if diff <= 1 || (diff == 2 && word.len() == word_first.len()) {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}

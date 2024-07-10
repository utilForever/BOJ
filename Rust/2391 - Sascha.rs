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
        let word_sascha = scan.token::<String>().chars().collect::<Vec<_>>();
        let w = scan.token::<i64>();
        let mut ret = (i64::MAX, String::new());

        for _ in 0..w {
            let word = scan.token::<String>().chars().collect::<Vec<_>>();
            let mut cnt = 0;

            for i in 0..word_sascha.len() {
                if word_sascha[i] != word[i] {
                    cnt += 1;
                }
            }

            if cnt < ret.0 {
                ret = (cnt, word.iter().collect());
            }
        }

        writeln!(out, "{}", ret.1).unwrap();
    }
}

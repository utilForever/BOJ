use io::Write;
use std::{collections::HashSet, io, str};

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
    let mut words = HashSet::new();

    for _ in 0..n {
        let word = scan.token::<String>();

        if words.is_empty() {
            words.insert(word.clone());
        } else {
            let mut word_new = word.chars().collect::<Vec<_>>();
            let mut found = false;

            for _ in 0..word.len() - 1 {
                let last = word_new.pop().unwrap();
                word_new.insert(0, last);

                let word_shifted = word_new.iter().collect::<String>();

                if words.contains(&word_shifted) {
                    found = true;
                    break;
                }
            }

            if !found {
                words.insert(word);
            }
        }
    }

    writeln!(out, "{}", words.len()).unwrap();
}

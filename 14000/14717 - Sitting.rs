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

    let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
    let mut cards = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10];

    let pos = cards.iter().position(|&x| x == a).unwrap();
    cards.remove(pos);
    let pos = cards.iter().position(|&x| x == b).unwrap();
    cards.remove(pos);

    let mut win = 0;
    let mut total = 0;

    for (idx1, &c) in cards.iter().enumerate() {
        for (idx2, &d) in cards.iter().enumerate() {
            if idx1 == idx2 {
                continue;
            }

            total += 1;

            if a == b {
                if c == d {
                    win += if a > c { 1 } else { 0 };
                } else {
                    win += 1;
                }
            } else {
                if c != d {
                    let sum_younghak = (a + b) % 10;
                    let sum_opponent = (c + d) % 10;
                    win += if sum_younghak > sum_opponent { 1 } else { 0 };
                }
            }
        }
    }

    writeln!(out, "{:.3}", win as f64 / total as f64).unwrap();
}

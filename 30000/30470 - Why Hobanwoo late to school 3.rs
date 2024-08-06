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
    let mut stairs = Vec::new();

    for _ in 0..n {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());

        if a == 1 {
            stairs.push(b);
        } else {
            if stairs.is_empty() {
                continue;
            }

            let len = stairs.len();
            stairs[len - 1] = (stairs[len - 1] - b).max(0);
        }
    }

    if stairs.is_empty() {
        writeln!(out, "0").unwrap();
        return;
    }

    for i in (0..stairs.len() - 1).rev() {
        stairs[i] = if stairs[i] > stairs[i + 1] {
            stairs[i + 1]
        } else {
            stairs[i]
        };
    }

    writeln!(out, "{}", stairs.iter().sum::<i64>()).unwrap();
}

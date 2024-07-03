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

    let n = scan.token::<usize>();
    let mut cows = vec![(0, 0); n];

    for i in 0..n {
        cows[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut ret = (0.0, 0, 0);

    for i in 0..n - 1 {
        for j in i + 1..n {
            let dist = ((cows[i].0 - cows[j].0).pow(2) + (cows[i].1 - cows[j].1).pow(2)) as f64;

            if ret.0 < dist {
                ret = (dist, i, j);
            }
        }
    }

    writeln!(out, "{} {}", ret.1 + 1, ret.2 + 1).unwrap();
}
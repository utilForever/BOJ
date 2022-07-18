use io::Write;
use std::{cmp, io, str};

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

    let (n, t) = (scan.token::<usize>(), scan.token::<usize>());
    let mut k = vec![0; n];
    let mut s = vec![0; n];

    for i in 0..n {
        (k[i], s[i]) = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut ret = vec![vec![0; t + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=t {
            if k[i - 1] > j as i64 {
                ret[i][j] = ret[i - 1][j];
            } else {
                ret[i][j] = cmp::max(ret[i - 1][j], s[i - 1] + ret[i - 1][j - k[i - 1] as usize]);
            }
        }
    }

    writeln!(out, "{}", ret[n][t]).unwrap();
}

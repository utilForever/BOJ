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

    let (n, m, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut lake = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            lake[i][j] = scan.token::<i64>();
        }
    }

    let mut prefix_sum = vec![vec![0; m]; n];

    for j in 0..m {
        prefix_sum[0][j] = lake[0][j];
    }

    for i in 1..n {
        for j in 0..m {
            prefix_sum[i][j] += prefix_sum[i - 1][j] + lake[i][j];
        }
    }

    let mut ret = vec![vec![0; m]; n];

    for j in 0..m {
        ret[0][j] = prefix_sum[0][j];
    }

    for i in 1..n {
        for j in 0..m {
            ret[i][j] += if j == 0 {
                prefix_sum[i][j]
            } else {
                prefix_sum[i][j] + ret[i - 1][j - 1]
            };
        }
    }

    for _ in 0..q {
        let (w, p) = (scan.token::<usize>(), scan.token::<usize>());
        writeln!(out, "{}", ret[w - 1][p - 1]).unwrap();
    }
}

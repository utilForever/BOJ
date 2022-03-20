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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut items = Vec::new();
    let mut max_values = vec![vec![0; k + 1]; n + 1];

    for _ in 0..n {
        let (w, v) = (scan.token::<usize>(), scan.token::<usize>());
        items.push((w, v));
    }

    for i in 0..n {
        for j in 0..=k {
            if i == 0 {
                if items[i].0 <= j {
                    max_values[i][j] = items[i].1;
                }
            } else {
                if items[i].0 <= j {
                    max_values[i][j] =
                        max_values[i - 1][j].max(max_values[i - 1][j - items[i].0] + items[i].1);
                } else {
                    max_values[i][j] = max_values[i - 1][j];
                }
            }
        }
    }

    writeln!(out, "{}", max_values[n - 1][k]).unwrap();
}

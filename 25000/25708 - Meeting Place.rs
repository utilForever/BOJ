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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut square = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            square[i][j] = scan.token::<i64>();
        }
    }

    let mut sum_row = vec![0; n];
    let mut sum_col = vec![0; m];

    for i in 0..n {
        for j in 0..m {
            sum_row[i] += square[i][j];
            sum_col[j] += square[i][j];
        }
    }

    let mut ret = i64::MIN;

    for i in 0..n - 1 {
        for j in i + 1..n {
            for k in 0..m - 1 {
                for l in k + 1..m {
                    let mut sum = sum_row[i] + sum_row[j] + sum_col[k] + sum_col[l];
                    sum -= square[i][k] + square[i][l] + square[j][k] + square[j][l];
                    sum += ((j - i - 1) * (l - k - 1)) as i64;

                    ret = ret.max(sum);
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

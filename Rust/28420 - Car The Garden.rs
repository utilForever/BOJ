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
    let (a, b, c) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut airs = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            airs[i][j] = scan.token::<i64>();
        }
    }

    let mut partial_sum = vec![vec![0; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            partial_sum[i][j] = airs[i - 1][j - 1] + partial_sum[i - 1][j] + partial_sum[i][j - 1]
                - partial_sum[i - 1][j - 1];
        }
    }

    let mut ret = i64::MAX;

    // First case
    for i in a..=n {
        for j in b + c..=m {
            ret = ret.min(
                partial_sum[i][j] - partial_sum[i - a][j] - partial_sum[i][j - (b + c)]
                    + partial_sum[i - a][j - (b + c)],
            );
        }
    }

    // Second case
    for i in a + b..=n {
        for j in a + c..=m {
            let rectangle_large =
                partial_sum[i][j] - partial_sum[i - (a + b)][j] - partial_sum[i][j - (a + c)]
                    + partial_sum[i - (a + b)][j - (a + c)];
            let rectangle_small =
                partial_sum[i][j - a] - partial_sum[i - b][j - a] - partial_sum[i][j - (a + c)]
                    + partial_sum[i - b][j - (a + c)];
            let rectangle_corner =
                partial_sum[i - b][j] - partial_sum[i - (a + b)][j] - partial_sum[i - b][j - a]
                    + partial_sum[i - (a + b)][j - a];

            ret = ret.min(rectangle_large - rectangle_small - rectangle_corner);
        }
    }

    // Third case
    for i in a + c..=n {
        for j in a + b..=m {
            let rectangle_large =
                partial_sum[i][j] - partial_sum[i - (a + c)][j] - partial_sum[i][j - (a + b)]
                    + partial_sum[i - (a + c)][j - (a + b)];
            let rectangle_small =
                partial_sum[i][j - a] - partial_sum[i - c][j - a] - partial_sum[i][j - (a + b)]
                    + partial_sum[i - c][j - (a + b)];
            let rectangle_corner =
                partial_sum[i - c][j] - partial_sum[i - (a + c)][j] - partial_sum[i - c][j - a]
                    + partial_sum[i - (a + c)][j - a];

            ret = ret.min(rectangle_large - rectangle_small - rectangle_corner);
        }
    }

    writeln!(out, "{ret}").unwrap();
}

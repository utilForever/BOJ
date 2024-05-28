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
    let mut prefix_sum = vec![vec![0; n + 1]; n + 1];
    let mut land = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            land[i][j] = -scan.token::<i64>();
        }
    }

    let range = m / 2;
    let mut ret = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            let y = i + range;
            let x = j + range;

            if y > n || x > n {
                continue;
            }

            prefix_sum[y][x] =
                prefix_sum[y - 1][x] + prefix_sum[y][x - 1] - prefix_sum[y - 1][x - 1] + ret[y][x];

            let y_min = if i > range { i - range } else { 1 };
            let x_min = if j > range { j - range } else { 1 };
            let y_max = if y < n { y } else { n };
            let x_max = if x < n { x } else { n };

            let sum = prefix_sum[y_max][x_max]
                - prefix_sum[y_max][x_min - 1]
                - prefix_sum[y_min - 1][x_max]
                + prefix_sum[y_min - 1][x_min - 1];
            ret[y][x] = land[i][j] - sum;

            prefix_sum[y][x] =
                prefix_sum[y - 1][x] + prefix_sum[y][x - 1] - prefix_sum[y - 1][x - 1] + ret[y][x];
        }
    }

    for i in 1..=n {
        for j in 1..=n {
            write!(out, "{} ", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}

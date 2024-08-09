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

    let (n, m) = (scan.token::<usize>(), scan.token::<i32>());
    let mut prefix_sum = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            prefix_sum[i][j] = prefix_sum[i][j - 1] + scan.token::<i32>();
        }
    }

    for _ in 0..m {
        let w = scan.token::<i32>();

        if w == 0 {
            let (x, y, c) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i32>(),
            );

            let diff = c - prefix_sum[x][y] + prefix_sum[x][y - 1];

            for i in y..=n {
                prefix_sum[x][i] += diff;
            }
        } else {
            let (x1, y1, x2, y2) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
            );

            let mut ret = 0;

            for i in x1..=x2 {
                ret += prefix_sum[i][y2] - prefix_sum[i][y1 - 1];
            }

            writeln!(out, "{ret}").unwrap();
        }
    }
}

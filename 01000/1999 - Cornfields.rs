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

    let (n, b, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut matrix = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = scan.token::<i64>();
        }
    }

    let mut max = vec![vec![0; n - b + 1]; n - b + 1];
    let mut min = vec![vec![0; n - b + 1]; n - b + 1];

    for i in 0..n - b + 1 {
        for j in 0..n - b + 1 {
            let mut max_val = 0;
            let mut min_val = i64::MAX;

            for x in i..i + b {
                for y in j..j + b {
                    max_val = max_val.max(matrix[x][y]);
                    min_val = min_val.min(matrix[x][y]);
                }
            }

            max[i][j] = max_val;
            min[i][j] = min_val;
        }
    }

    for _ in 0..k {
        let (i, j) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        writeln!(out, "{}", max[i][j] - min[i][j]).unwrap();
    }
}

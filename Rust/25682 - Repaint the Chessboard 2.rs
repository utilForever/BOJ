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

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut board = vec![vec![' '; m + 1]; n + 1];

    for i in 1..=n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            board[i][j + 1] = c;
        }
    }

    let mut prefix_sum_black = vec![vec![0i64; m + 1]; n + 1];
    let mut prefix_sum_white = vec![vec![0i64; m + 1]; n + 1];

    // Calculate prefix sum for black (start)
    for i in 1..=n {
        for j in 1..=m {
            let val = if (i + j) % 2 == 0 { 'B' } else { 'W' };
            prefix_sum_black[i][j] = prefix_sum_black[i - 1][j] + prefix_sum_black[i][j - 1]
                - prefix_sum_black[i - 1][j - 1]
                + (board[i][j] != val) as i64;
        }
    }

    // Calculate prefix sum for white (start)
    for i in 1..=n {
        for j in 1..=m {
            let val = if (i + j) % 2 == 0 { 'W' } else { 'B' };
            prefix_sum_white[i][j] = prefix_sum_white[i - 1][j] + prefix_sum_white[i][j - 1]
                - prefix_sum_white[i - 1][j - 1]
                + (board[i][j] != val) as i64;
        }
    }

    let mut ret = i64::MAX;

    // Calculate minimum value for each k x k square
    for i in k..=n {
        for j in k..=m {
            let val_black =
                prefix_sum_black[i][j] - prefix_sum_black[i - k][j] - prefix_sum_black[i][j - k]
                    + prefix_sum_black[i - k][j - k];
            let val_white =
                prefix_sum_white[i][j] - prefix_sum_white[i - k][j] - prefix_sum_white[i][j - k]
                    + prefix_sum_white[i - k][j - k];

            ret = ret.min(val_black).min(val_white);
        }
    }

    writeln!(out, "{ret}").unwrap();
}

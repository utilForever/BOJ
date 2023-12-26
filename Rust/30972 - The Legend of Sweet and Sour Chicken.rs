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
    let mut grid = vec![vec![0; n + 2]; n + 2];

    for i in 1..=n {
        for j in 1..=n {
            grid[i][j] = scan.token::<i64>();
        }
    }

    let mut prefix_sum = vec![vec![0; n + 2]; n + 2];

    for i in 1..=n + 1 {
        for j in 1..=n + 1 {
            prefix_sum[i][j] =
                prefix_sum[i - 1][j] + prefix_sum[i][j - 1] - prefix_sum[i - 1][j - 1] + grid[i][j];
        }
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (r1, c1, r2, c2) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        let outer = prefix_sum[r2][c2] - prefix_sum[r1 - 1][c2] - prefix_sum[r2][c1 - 1]
            + prefix_sum[r1 - 1][c1 - 1];
        let inner = prefix_sum[r2 - 1][c2 - 1] - prefix_sum[r1][c2 - 1] - prefix_sum[r2 - 1][c1]
            + prefix_sum[r1][c1];

        writeln!(out, "{}", inner * 2 - outer).unwrap();
    }
}

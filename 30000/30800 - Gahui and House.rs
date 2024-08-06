use io::Write;
use std::{cmp::Reverse, collections::BinaryHeap, io, str};

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

fn calculate_range(prefix_sum: &Vec<Vec<i64>>, i1: usize, i2: usize, j1: usize, j2: usize) -> i64 {
    prefix_sum[i2][j2] - prefix_sum[i1 - 1][j2] - prefix_sum[i2][j1 - 1]
        + prefix_sum[i1 - 1][j1 - 1]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
    let (h1, h2, w1, w2) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut grid = vec![vec![0; w + 1]; h + 1];

    for i in 1..=h {
        for j in 1..=w {
            grid[i][j] = scan.token::<i64>();
        }
    }

    if h <= h1 || w <= w1 {
        writeln!(out, "No").unwrap();
        return;
    }

    let mut prefix_sum = vec![vec![0; w + 1]; h + 1];

    for i in 1..=h {
        for j in 1..=w {
            prefix_sum[i][j] =
                prefix_sum[i - 1][j] + prefix_sum[i][j - 1] - prefix_sum[i - 1][j - 1] + grid[i][j];
        }
    }

    let h2 = h2.min(h - 1);
    let w2 = w2.min(w - 1);
    let mut ret = i64::MAX;

    for i in h1 + 1..=h2 + 1 {
        let mut priority_queue: BinaryHeap<Reverse<(i64, i64)>> = BinaryHeap::new();

        for j in w1 + 1..=w {
            while !priority_queue.is_empty()
                && priority_queue.peek().unwrap().0 .1 < (j - w2) as i64
            {
                priority_queue.pop();
            }

            priority_queue.push(Reverse((
                calculate_range(&prefix_sum, 1, i, j - w1, w)
                    - calculate_range(&prefix_sum, 2, i - 1, j - w1 + 1, w),
                (j - w1) as i64,
            )));

            let val = if j < w {
                calculate_range(&prefix_sum, 1, 1, j + 1, w)
                    + calculate_range(&prefix_sum, i, i, j + 1, w)
            } else {
                0
            };

            ret = ret.min(
                priority_queue.peek().unwrap().0 .0 - val
                    + calculate_range(&prefix_sum, 2, i - 1, j, j),
            );
        }
    }

    writeln!(out, "{ret}").unwrap();
}

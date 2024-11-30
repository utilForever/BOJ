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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (h, w, k, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut grid = vec![vec![0; w + 1]; h + 1];

    for i in 1..=h {
        for j in 1..=w {
            grid[i][j] = scan.token::<usize>();
        }
    }

    let mut info = vec![(h + 1, 0, w + 1, 0); h * w + 1];

    for i in 1..=h {
        for j in 1..=w {
            info[grid[i][j]].0 = info[grid[i][j]].0.min(i);
            info[grid[i][j]].1 = info[grid[i][j]].1.max(i);
            info[grid[i][j]].2 = info[grid[i][j]].2.min(j);
            info[grid[i][j]].3 = info[grid[i][j]].3.max(j);
        }
    }

    // 2D imos method
    let mut prefix_sum = vec![vec![0; w + 2]; h + 2];
    let mut cnt_num = 0;

    for i in 1..=h * w {
        let (min_y, max_y, min_x, max_x) = info[i];

        // The value doesn't exist
        if min_y == h + 1 {
            continue;
        }

        cnt_num += 1;

        // Check whether the rectangle (k x k) affects the number of different values
        let max_y = (max_y as i64 - k + 1).max(1);
        let max_x = (max_x as i64 - k + 1).max(1);

        // If the rectangle doesn't affect the number of different values, skip it
        if (min_y as i64) < max_y || (min_x as i64) < max_x {
            continue;
        }

        let max_y = max_y as usize;
        let max_x = max_x as usize;

        // Update the prefix sum
        prefix_sum[max_y][max_x] += 1;
        prefix_sum[max_y][min_x + 1] -= 1;
        prefix_sum[min_y + 1][max_x] -= 1;
        prefix_sum[min_y + 1][min_x + 1] += 1;
    }

    for i in 1..=h {
        for j in 1..=w {
            prefix_sum[i][j] +=
                prefix_sum[i][j - 1] + prefix_sum[i - 1][j] - prefix_sum[i - 1][j - 1];
        }
    }

    for _ in 0..q {
        let (y, x) = (scan.token::<usize>(), scan.token::<usize>());
        writeln!(out, "{}", cnt_num - prefix_sum[y][x]).unwrap();
    }
}

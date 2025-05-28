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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let (n, r, c, w) = (
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut grid = vec![vec![0; c]; r];

    for _ in 0..n {
        let (x, y) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        grid[x][y] = 1;
    }

    let mut prefix_sum = vec![vec![0; c + 1]; r + 1];

    for i in 1..=r {
        for j in 1..=c {
            prefix_sum[i][j] = prefix_sum[i - 1][j] + prefix_sum[i][j - 1]
                - prefix_sum[i - 1][j - 1]
                + grid[i - 1][j - 1];
        }
    }

    let mut ret_satisfaction = 0;
    let mut ret_position = (usize::MAX, usize::MAX);

    for x in 1..=r {
        for y in 1..=c {
            if grid[x - 1][y - 1] != 0 {
                continue;
            }

            let up = x.saturating_sub(w / 2).max(1);
            let down = (x + w / 2).min(r);
            let left = y.saturating_sub(w / 2).max(1);
            let right = (y + w / 2).min(c);

            let satisfaction =
                prefix_sum[down][right] - prefix_sum[up - 1][right] - prefix_sum[down][left - 1]
                    + prefix_sum[up - 1][left - 1];

            if satisfaction > ret_satisfaction
                || (satisfaction == ret_satisfaction
                    && (x < ret_position.0 || (x == ret_position.0 && y < ret_position.1)))
            {
                ret_satisfaction = satisfaction;
                ret_position = (x, y);
            }
        }
    }

    writeln!(out, "{ret_satisfaction}").unwrap();
    writeln!(out, "{} {}", ret_position.0, ret_position.1).unwrap();
}

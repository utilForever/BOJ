use io::Write;
use std::{collections::VecDeque, io, str};

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
    let mut grid = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            grid[i][j] = scan.token::<u8>();
        }
    }

    let x = scan.token::<i64>();

    if grid[0][0] != grid[n - 1][m - 1] {
        writeln!(out, "DEAD").unwrap();
        return;
    }

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; m]; n];

    queue.push_back((0, 0));
    visited[0][0] = true;

    while !queue.is_empty() {
        let (y_curr, x_curr) = queue.pop_front().unwrap();

        if y_curr == n - 1 && x_curr == m - 1 {
            writeln!(out, "ALIVE").unwrap();
            return;
        }

        // Can move the manhattan distance within x
        for i in -x..=x {
            for j in -x..=x {
                if i.abs() + j.abs() > x {
                    continue;
                }

                if i == 0 && j == 0 {
                    continue;
                }

                let y_next = y_curr as i64 + i;
                let x_next = x_curr as i64 + j;

                if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= m as i64 {
                    continue;
                }

                let y_next = y_next as usize;
                let x_next = x_next as usize;

                if visited[y_next][x_next] {
                    continue;
                }

                if grid[y_next][x_next] == grid[y_curr][x_curr] {
                    queue.push_back((y_next, x_next));
                    visited[y_next][x_next] = true;
                }
            }
        }
    }

    writeln!(out, "DEAD").unwrap();
}

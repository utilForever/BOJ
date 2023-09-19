use io::Write;
use std::{collections::BinaryHeap, io, str};

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
    let mut field = vec![vec![0; m]; n];
    let mut visited = vec![vec![false; m]; n];

    for i in 0..n {
        for j in 0..m {
            field[i][j] = scan.token::<i64>();
        }
    }

    let k = scan.token::<i64>();
    let mut priority_queue = BinaryHeap::new();

    for i in 0..n {
        for j in 0..m {
            if i == 0 || j == 0 || i == n - 1 || j == m - 1 {
                priority_queue.push((field[i][j], i, j));
                visited[i][j] = true;
            }
        }
    }

    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];

    for _ in 0..k {
        let (_, x, y) = priority_queue.pop().unwrap();

        writeln!(out, "{} {}", x + 1, y + 1).unwrap();

        for i in 0..4 {
            let next_x = x as i64 + dx[i];
            let next_y = y as i64 + dy[i];

            if next_x < 0 || next_x >= n as i64 || next_y < 0 || next_y >= m as i64 {
                continue;
            }

            let next_x = next_x as usize;
            let next_y = next_y as usize;

            if visited[next_x][next_y] {
                continue;
            }

            priority_queue.push((field[next_x][next_y], next_x, next_y));
            visited[next_x][next_y] = true;
        }
    }
}

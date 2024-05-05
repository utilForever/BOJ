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
    let mut banner = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            banner[i][j] = scan.token::<u8>();
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; m]; n];
    let mut ret = 0;

    let dy = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dx = [-1, 0, 1, -1, 1, -1, 0, 1];

    for i in 0..n {
        for j in 0..m {
            if visited[i][j] || banner[i][j] == 0 {
                continue;
            }

            queue.push_back((i, j));
            ret += 1;

            while !queue.is_empty() {
                let (y, x) = queue.pop_front().unwrap();
                visited[y][x] = true;

                for k in 0..8 {
                    let y_next = y as i32 + dy[k];
                    let x_next = x as i32 + dx[k];

                    if y_next < 0 || x_next < 0 || y_next >= n as i32 || x_next >= m as i32 {
                        continue;
                    }

                    let y_next = y_next as usize;
                    let x_next = x_next as usize;

                    if visited[y_next][x_next] || banner[y_next][x_next] == 0 {
                        continue;
                    }

                    visited[y_next][x_next] = true;
                    queue.push_back((y_next, x_next));
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

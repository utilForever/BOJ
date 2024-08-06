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

fn explore(maze: &Vec<Vec<char>>, visited: &mut Vec<Vec<i64>>, n: usize, m: usize) -> i64 {
    let dy: [i64; 4] = [0, 1, 0, -1];
    let dx: [i64; 4] = [1, 0, -1, 0];

    let mut queue = VecDeque::new();
    let mut ret = i64::MAX;

    queue.push_back((0, 0));
    visited[0][0] = 0;

    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();

        if y == n - 1 && x == m - 1 {
            ret = ret.min(visited[y][x]);
        }

        for i in 0..4 {
            let y_next: i64 = y as i64 + dy[i];
            let x_next = x as i64 + dx[i];

            if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= m as i64 {
                continue;
            }

            let x_next = x_next as usize;
            let y_next = y_next as usize;

            if visited[y_next][x_next] == 0 {
                if maze[y_next][x_next] == '0' {
                    visited[y_next][x_next] = visited[y][x] + 1;
                    queue.push_back((y_next, x_next));
                } else if maze[y_next][x_next] == '2' {
                    visited[y_next][x_next] = visited[y][x] + 1;
                    ret = ret.min(
                        visited[y_next][x_next] + (n - 1 - y_next) as i64 + (m - 1 - x_next) as i64,
                    );
                }
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, t) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut maze = vec![vec![' '; m]; n];
    let mut visited = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            maze[i][j] = scan.token::<char>();
        }
    }

    let ret = explore(&maze, &mut visited, n, m);

    if ret > t {
        writeln!(out, "Fail").unwrap();
    } else {
        writeln!(out, "{ret}").unwrap();
    }
}

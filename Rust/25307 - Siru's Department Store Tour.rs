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

fn preprocess_bfs(
    department_store: &mut Vec<Vec<i64>>,
    visited: &mut Vec<Vec<i64>>,
    queue: &mut VecDeque<(usize, usize)>,
    n: usize,
    m: usize,
    k: i64,
) {
    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        if visited[x][y] == k + 1 {
            continue;
        }

        for i in 0..4 {
            let next_x = x as i64 + dx[i];
            let next_y = y as i64 + dy[i];

            if next_x < 0 || next_x >= n as i64 || next_y < 0 || next_y >= m as i64 {
                continue;
            }

            let next_x = next_x as usize;
            let next_y = next_y as usize;

            if visited[next_x][next_y] > 0 {
                continue;
            }

            if department_store[next_x][next_y] == 4 {
                continue;
            }

            visited[next_x][next_y] = visited[x][y] + 1;
            department_store[next_x][next_y] = 1;
            queue.push_back((next_x, next_y));
        }
    }
}

fn process_bfs(
    department_store: &Vec<Vec<i64>>,
    visited: &mut Vec<Vec<i64>>,
    n: usize,
    m: usize,
    pos: (usize, usize),
) -> i64 {
    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];

    for i in 0..n {
        for j in 0..m {
            visited[i][j] = 0;
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(pos);
    visited[pos.0][pos.1] = 1;

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        for i in 0..4 {
            let next_x = x as i64 + dx[i];
            let next_y = y as i64 + dy[i];

            if next_x < 0 || next_x >= n as i64 || next_y < 0 || next_y >= m as i64 {
                continue;
            }

            let next_x = next_x as usize;
            let next_y = next_y as usize;

            if visited[next_x][next_y] > 0 {
                continue;
            }

            if department_store[next_x][next_y] == 1 {
                continue;
            }

            visited[next_x][next_y] = visited[x][y] + 1;

            if department_store[next_x][next_y] == 2 {
                return visited[x][y];
            }

            queue.push_back((next_x, next_y));
        }
    }

    -1
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut queue = VecDeque::new();
    let mut department_store = vec![vec![0; m]; n];
    let mut visited = vec![vec![0; m]; n];
    let mut pos_start = (0, 0);

    for i in 0..n {
        for j in 0..m {
            department_store[i][j] = scan.token::<i64>();

            if department_store[i][j] == 4 {
                pos_start = (i, j);
            } else if department_store[i][j] == 3 {
                visited[i][j] = 1;
                department_store[i][j] = 1;
                queue.push_back((i, j));
            }
        }
    }

    preprocess_bfs(&mut department_store, &mut visited, &mut queue, n, m, k);
    let ret = process_bfs(&department_store, &mut visited, n, m, pos_start);

    writeln!(out, "{ret}").unwrap();
}

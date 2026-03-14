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

const INF: i64 = i64::MAX / 4;
const DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn is_endpoint(y: usize, x: usize, n: usize, m: usize) -> bool {
    (y == 0 && x == 0) || (y + 1 == n && x + 1 == m)
}

fn process_bfs(board: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let (n, m) = (board.len(), board[0].len());
    let mut queue = VecDeque::new();
    let mut dist = vec![vec![INF; m]; n];

    for i in 0..n {
        if is_endpoint(i, m - 1, n, m) {
            continue;
        }

        let cost = board[i][m - 1] ^ 1;

        if dist[i][m - 1] > cost {
            dist[i][m - 1] = cost;

            if cost == 0 {
                queue.push_front((i, m - 1));
            } else {
                queue.push_back((i, m - 1));
            }
        }
    }

    for j in 0..m {
        if is_endpoint(0, j, n, m) {
            continue;
        }

        let cost = board[0][j] ^ 1;

        if dist[0][j] > cost {
            dist[0][j] = cost;

            if cost == 0 {
                queue.push_front((0, j));
            } else {
                queue.push_back((0, j));
            }
        }
    }

    while let Some((y, x)) = queue.pop_front() {
        for (dy, dx) in DIRECTIONS.iter() {
            let (y_next, x_next) = (y as i64 + dy, x as i64 + dx);

            if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= m as i64 {
                continue;
            }

            let (y_next, x_next) = (y_next as usize, x_next as usize);

            if is_endpoint(y_next, x_next, n, m) {
                continue;
            }

            let cost = board[y_next][x_next] ^ 1;
            let dist_next = dist[y][x] + cost;

            if dist[y_next][x_next] > dist_next {
                dist[y_next][x_next] = dist_next;

                if cost == 0 {
                    queue.push_front((y_next, x_next));
                } else {
                    queue.push_back((y_next, x_next));
                }
            }
        }
    }

    dist
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut board = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            board[i][j] = scan.token::<i64>();
        }
    }

    let dist = process_bfs(&board);
    let mut ret = 2;

    for i in 0..n {
        if !is_endpoint(i, 0, n, m) {
            ret = ret.min(dist[i][0]);
        }
    }

    for j in 0..m {
        if !is_endpoint(n - 1, j, n, m) {
            ret = ret.min(dist[n - 1][j]);
        }
    }

    writeln!(out, "{ret}").unwrap();
}

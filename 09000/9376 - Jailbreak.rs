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
const DIRECTIONS: [(i64, i64); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn process_bfs(board: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<Vec<i64>> {
    let (h, w) = (board.len(), board[0].len());
    let mut queue = VecDeque::new();
    let mut dist = vec![vec![INF; w]; h];

    queue.push_back(start);
    dist[start.0][start.1] = 0;

    while let Some((y, x)) = queue.pop_front() {
        for (dy, dx) in DIRECTIONS.iter() {
            let (y_next, x_next) = (y as i64 + dy, x as i64 + dx);

            if y_next < 0 || y_next >= h as i64 || x_next < 0 || x_next >= w as i64 {
                continue;
            }

            let (y_next, x_next) = (y_next as usize, x_next as usize);

            if board[y_next][x_next] == '*' {
                continue;
            }

            let cost = if board[y_next][x_next] == '#' { 1 } else { 0 };
            let dist_new = dist[y][x] + cost;

            if dist_new < dist[y_next][x_next] {
                dist[y_next][x_next] = dist_new;

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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
        let mut board = vec![vec![' '; w + 2]; h + 2];
        let mut prisoners = Vec::new();

        for i in 1..=h {
            let line = scan.line().trim().to_string();

            for (j, c) in line.chars().enumerate() {
                board[i][j + 1] = c;

                if c == '$' {
                    prisoners.push((i, j + 1));
                    board[i][j + 1] = '.';
                }
            }
        }

        let dist0 = process_bfs(&board, (0, 0));
        let dist1 = process_bfs(&board, prisoners[0]);
        let dist2 = process_bfs(&board, prisoners[1]);
        let mut ret = INF;

        for i in 0..=h + 1 {
            for j in 0..=w + 1 {
                if board[i][j] == '*' {
                    continue;
                }

                if dist0[i][j] == INF || dist1[i][j] == INF || dist2[i][j] == INF {
                    continue;
                }

                let mut total = dist0[i][j] + dist1[i][j] + dist2[i][j];

                if board[i][j] == '#' {
                    total -= 2;
                }

                ret = ret.min(total);
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}

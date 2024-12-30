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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn process_bfs(
    board: &Vec<Vec<bool>>,
    idxes: &mut Vec<Vec<i64>>,
    y_start: usize,
    x_start: usize,
    label: i64,
    direction: &[(i32, i32)],
) {
    let n = board.len();
    let mut queue = VecDeque::new();

    queue.push_back((y_start, x_start));
    idxes[y_start][x_start] = label;

    while let Some((y, x)) = queue.pop_front() {
        for &(dy, dx) in direction {
            let y_next = y as i32 + dy;
            let x_next = x as i32 + dx;

            if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= n as i32 {
                continue;
            }

            let y_next = y_next as usize;
            let x_next = x_next as usize;

            if board[y_next][x_next] && idxes[y_next][x_next] == -1 {
                queue.push_back((y_next, x_next));
                idxes[y_next][x_next] = label;
            }
        }
    }
}

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    check: &mut Vec<bool>,
    matched: &mut Vec<i64>,
    idx: usize,
) -> bool {
    for &next in graph[idx].iter() {
        if check[next] {
            continue;
        }

        check[next] = true;

        if matched[next] == -1 || process_dfs(graph, check, matched, matched[next] as usize) {
            matched[next] = idx as i64;
            return true;
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let m = scan.token::<i64>();
    let mut board = vec![vec![true; n]; n];

    for _ in 0..m {
        let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
        board[r - 1][c - 1] = false;
    }

    let mut idxes_left = vec![vec![-1; n]; n];
    let mut cnt_left = 0;

    for i in 0..n {
        for j in 0..n {
            if board[i][j] && idxes_left[i][j] == -1 {
                process_bfs(&board, &mut idxes_left, i, j, cnt_left, &[(1, 1), (-1, -1)]);

                cnt_left += 1;
            }
        }
    }

    let mut idxes_right = vec![vec![-1; n]; n];
    let mut cnt_right = 0;

    for i in 0..n {
        for j in 0..n {
            if board[i][j] && idxes_right[i][j] == -1 {
                process_bfs(
                    &board,
                    &mut idxes_right,
                    i,
                    j,
                    cnt_right,
                    &[(1, -1), (-1, 1)],
                );

                cnt_right += 1;
            }
        }
    }

    let mut graph = vec![Vec::new(); cnt_left as usize];

    for i in 0..n {
        for j in 0..n {
            if board[i][j] {
                let u = idxes_left[i][j] as usize;
                let v = idxes_right[i][j] as usize;

                graph[u].push(v);
            }
        }
    }

    let mut check = vec![false; cnt_right as usize];
    let mut matched = vec![-1; cnt_right as usize];
    let mut ret = 0;

    for u in 0..cnt_left {
        check.fill(false);

        if process_dfs(&graph, &mut check, &mut matched, u as usize) {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}

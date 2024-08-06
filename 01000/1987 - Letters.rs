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
}

fn process_dfs(
    board: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    chekced: &mut Vec<bool>,
    y_curr: i64,
    x_curr: i64,
    r: usize,
    c: usize,
    depth: i64,
) -> i64 {
    let dy: [i64; 4] = [0, 1, 0, -1];
    let dx: [i64; 4] = [1, 0, -1, 0];

    let mut ret = depth;

    for i in 0..4 {
        let (y_next, x_next) = (y_curr + dy[i], x_curr + dx[i]);

        if y_next < 0 || y_next >= r as i64 || x_next < 0 || x_next >= c as i64 {
            continue;
        }

        if visited[y_next as usize][x_next as usize] {
            continue;
        }

        if chekced[board[y_next as usize][x_next as usize] as usize - 'A' as usize] {
            continue;
        }

        visited[y_next as usize][x_next as usize] = true;
        chekced[board[y_next as usize][x_next as usize] as usize - 'A' as usize] = true;

        let val = process_dfs(board, visited, chekced, y_next, x_next, r, c, depth + 1);

        visited[y_next as usize][x_next as usize] = false;
        chekced[board[y_next as usize][x_next as usize] as usize - 'A' as usize] = false;

        ret = ret.max(val);
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut board = vec![vec![' '; c]; r];
    let mut checked = vec![false; 26];
    let mut visited = vec![vec![false; c]; r];

    for i in 0..r {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            board[i][j] = c;
        }
    }

    visited[0][0] = true;
    checked[board[0][0] as usize - 'A' as usize] = true;

    let ret = process_dfs(&board, &mut visited, &mut checked, 0, 0, r, c, 1);
    writeln!(out, "{ret}").unwrap();
}

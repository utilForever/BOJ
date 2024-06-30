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
    corridor: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    cnt: &mut i64,
    i: usize,
    j: usize,
) {
    let n = corridor.len();
    let m = corridor[0].len();
    let dy = [-1, 1, 0, 0];
    let dx = [0, 0, -1, 1];

    visited[i][j] = true;
    *cnt += 1;

    for k in 0..4 {
        let (y_next, x_next) = (i as i32 + dy[k], j as i32 + dx[k]);

        if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= m as i32 {
            continue;
        }

        let (y_next, x_next) = (y_next as usize, x_next as usize);

        if corridor[y_next][x_next] == '*' && !visited[y_next][x_next] {
            process_dfs(corridor, visited, cnt, y_next, x_next);
        }
    }
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
    let mut corridor = vec![vec!['.'; m]; n];

    for _ in 0..k {
        let (r, c) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        corridor[r][c] = '*';
    }

    let mut visited = vec![vec![false; m]; n];
    let mut ret = 0;

    for i in 0..n {
        for j in 0..m {
            if corridor[i][j] == '.' {
                continue;
            }

            if visited[i][j] {
                continue;
            }

            let mut cnt = 0;

            process_dfs(&corridor, &mut visited, &mut cnt, i, j);

            ret = ret.max(cnt);
        }
    }

    writeln!(out, "{ret}").unwrap();
}

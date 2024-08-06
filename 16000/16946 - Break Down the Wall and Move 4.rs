use io::Write;
use std::{collections::HashSet, io, str};

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
    maze: &Vec<Vec<i32>>,
    numbered_maze: &mut Vec<Vec<i32>>,
    visited: &mut Vec<Vec<bool>>,
    y_curr: usize,
    x_curr: usize,
    cnt_empty: i32,
    n: usize,
    m: usize,
) -> i32 {
    numbered_maze[y_curr][x_curr] = cnt_empty;
    visited[y_curr][x_curr] = true;

    let dy = [-1, 1, 0, 0];
    let dx = [0, 0, -1, 1];
    let mut ret = 1;

    for i in 0..4 {
        let y_next = y_curr as i32 + dy[i];
        let x_next = x_curr as i32 + dx[i];

        if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= m as i32 {
            continue;
        }

        let y_next = y_next as usize;
        let x_next = x_next as usize;

        if visited[y_next][x_next] || maze[y_next][x_next] == 1 {
            continue;
        }

        ret += process_dfs(
            maze,
            numbered_maze,
            visited,
            y_next,
            x_next,
            cnt_empty,
            n,
            m,
        )
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut maze = vec![vec![0; m]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            maze[i][j] = (c as u8 - b'0') as i32;
        }
    }

    let mut maze_numbered = vec![vec![0; m]; n];
    let mut visited = vec![vec![false; m]; n];
    let mut cnt = Vec::new();
    let mut cnt_empty = 1;

    cnt.push(0);

    for i in 0..n {
        for j in 0..m {
            if visited[i][j] || maze[i][j] == 1 {
                continue;
            }

            cnt.push(process_dfs(
                &mut maze,
                &mut maze_numbered,
                &mut visited,
                i,
                j,
                cnt_empty,
                n,
                m,
            ));

            cnt_empty += 1;
        }
    }

    let dy = [-1, 1, 0, 0];
    let dx = [0, 0, -1, 1];

    for i in 0..n {
        for j in 0..m {
            if maze[i][j] == 0 {
                continue;
            }

            let mut set = HashSet::new();

            for k in 0..4 {
                let y_next = i as i32 + dy[k];
                let x_next = j as i32 + dx[k];

                if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= m as i32 {
                    continue;
                }

                let y_next = y_next as usize;
                let x_next = x_next as usize;

                set.insert(maze_numbered[y_next][x_next]);
            }

            for &idx in set.iter() {
                maze[i][j] += cnt[idx as usize];
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            write!(out, "{}", maze[i][j] % 10).unwrap();
        }

        writeln!(out).unwrap();
    }
}

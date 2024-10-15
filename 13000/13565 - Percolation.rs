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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn process_dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, y: usize, x: usize) {
    let dy = [-1, 0, 1, 0];
    let dx = [0, 1, 0, -1];

    for i in 0..4 {
        let y_next = y as i32 + dy[i];
        let x_next = x as i32 + dx[i];

        if y_next < 0 || y_next >= grid.len() as i32 || x_next < 0 || x_next >= grid[0].len() as i32
        {
            continue;
        }

        let (y_next, x_next) = (y_next as usize, x_next as usize);

        if grid[y_next][x_next] == '1' || visited[y_next][x_next] {
            continue;
        }

        visited[y_next][x_next] = true;
        process_dfs(grid, visited, y_next, x_next);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (m, n) = (scan.token::<usize>(), scan.token::<usize>());
    let mut grid = vec![vec![' '; n]; m];

    for i in 0..m {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    let mut visited = vec![vec![false; n]; m];

    for i in 0..n {
        if visited[0][i] {
            continue;
        }

        process_dfs(&grid, &mut visited, 0, i);
    }

    let mut ret = false;

    for i in 0..n {
        if visited[m - 1][i] {
            ret = true;
            break;
        }
    }

    writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
}

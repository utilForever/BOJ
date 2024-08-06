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
    grid: &Vec<Vec<i64>>,
    visited: &mut Vec<Vec<bool>>,
    x: i64,
    y: i64,
    n: i64,
    cnt: &mut i64,
) {
    if x < 0 || y < 0 || x >= n || y >= n || grid[x as usize][y as usize] == 1 || visited[x as usize][y as usize] {
        return;
    }

    visited[x as usize][y as usize] = true;
    *cnt += 1;

    process_dfs(grid, visited, x - 1, y, n, cnt);
    process_dfs(grid, visited, x + 1, y, n, cnt);
    process_dfs(grid, visited, x, y - 1, n, cnt);
    process_dfs(grid, visited, x, y + 1, n, cnt);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut grid = vec![vec![0; n]; n];
    let mut visited = vec![vec![false; n]; n];
    let mut ret = 0;

    for i in 0..n {
        for j in 0..n {
            grid[i][j] = scan.token::<i64>();
        }
    }

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 0 && !visited[i][j] {
                let mut cnt = 0;
                process_dfs(&grid, &mut visited, i as i64, j as i64, n as i64, &mut cnt);
                ret += if cnt % k == 0 { cnt / k } else { cnt / k + 1 };
            }
        }
    }

    if ret == 0 || ret > m {
        writeln!(out, "IMPOSSIBLE").unwrap();
    } else {
        writeln!(out, "POSSIBLE").unwrap();
        writeln!(out, "{}", m - ret).unwrap();
    }
}

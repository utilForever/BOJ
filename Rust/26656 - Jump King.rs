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

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn process_dfs(
    grid: &Vec<Vec<Vec<(usize, usize)>>>,
    visited: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
    cnt_visit: &mut i64,
) {
    *cnt_visit += 1;

    for cell in grid[i][j].iter() {
        if visited[cell.0][cell.1] {
            continue;
        }

        visited[cell.0][cell.1] = true;
        process_dfs(grid, visited, cell.0, cell.1, cnt_visit);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut grid_direction = vec![vec![Direction::Up; m + 1]; n + 1];
    let mut grid_dist = vec![vec![0; m + 1]; n + 1];
    let mut visited = vec![vec![false; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            let c = scan.token::<char>();

            grid_direction[i][j] = match c {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!(),
            };
        }
    }

    for i in 1..=n {
        for j in 1..=m {
            grid_dist[i][j] = scan.token::<i64>();
        }
    }

    let mut grid_rev = vec![vec![Vec::new(); m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            match grid_direction[i][j] {
                Direction::Up => {
                    if i as i64 - grid_dist[i][j] < 1 {
                        grid_rev[0][0].push((i, j));
                    } else {
                        let next_i = (i as i64 - grid_dist[i][j]) as usize;

                        grid_rev[next_i][j].push((i, j));
                        grid_rev[i][j].push((next_i, j));
                    }
                }
                Direction::Down => {
                    if i as i64 + grid_dist[i][j] > n as i64 {
                        grid_rev[0][0].push((i, j));
                    } else {
                        let next_i = (i as i64 + grid_dist[i][j]) as usize;

                        grid_rev[next_i][j].push((i, j));
                        grid_rev[i][j].push((next_i, j));
                    }
                }
                Direction::Left => {
                    if j as i64 - grid_dist[i][j] < 1 {
                        grid_rev[0][0].push((i, j));
                    } else {
                        let next_j = (j as i64 - grid_dist[i][j]) as usize;

                        grid_rev[i][next_j].push((i, j));
                        grid_rev[i][j].push((i, next_j));
                    }
                }
                Direction::Right => {
                    if j as i64 + grid_dist[i][j] > m as i64 {
                        grid_rev[0][0].push((i, j));
                    } else {
                        let next_j = (j as i64 + grid_dist[i][j]) as usize;

                        grid_rev[i][next_j].push((i, j));
                        grid_rev[i][j].push((i, next_j));
                    }
                }
            }
        }
    }

    let mut ret_true = Vec::new();
    let mut ret_false = Vec::new();

    for cell in grid_rev[0][0].iter() {
        let mut cnt = 0;
        visited[cell.0][cell.1] = true;

        process_dfs(&grid_rev, &mut visited, cell.0, cell.1, &mut cnt);
        ret_true.push(cnt);
    }

    for i in 1..=n {
        for j in 1..=m {
            if visited[i][j] {
                continue;
            }

            let mut cnt = 0;
            visited[i][j] = true;

            process_dfs(&grid_rev, &mut visited, i, j, &mut cnt);
            ret_false.push(cnt);
        }
    }

    let ret_initial = ret_true.iter().sum::<i64>();
    ret_true.sort_by(|a, b| b.cmp(a));
    ret_false.sort_by(|a, b| b.cmp(a));

    let mut sum_true = 0;
    let mut sum_false = 0;

    for i in 0..k.min(ret_true.len()) {
        sum_true += ret_true[i];
    }

    for i in 0..k.min(ret_false.len()) {
        sum_false += ret_false[i];
    }

    writeln!(
        out,
        "{} {}",
        ret_initial - sum_true,
        ret_initial + sum_false
    )
    .unwrap();
}

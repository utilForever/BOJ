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

fn diffuse_dust(grid: &mut Vec<Vec<i32>>, r: usize, c: usize) {
    let dy = [0, 1, 0, -1];
    let dx = [-1, 0, 1, 0];

    let mut after_grid = grid.clone();

    for i in 0..r {
        for j in 0..c {
            if grid[i][j] >= 5 {
                let dust_to_diffuse = grid[i][j] / 5;

                for k in 0..4 {
                    let x_next = j as i32 + dx[k];
                    let y_next = i as i32 + dy[k];

                    if x_next >= 0
                        && x_next < c as i32
                        && y_next >= 0
                        && y_next < r as i32
                        && grid[y_next as usize][x_next as usize] != -1
                    {
                        after_grid[y_next as usize][x_next as usize] += dust_to_diffuse;
                        after_grid[i][j] -= dust_to_diffuse;
                    }
                }
            }
        }
    }

    *grid = after_grid;
}

fn work_air_purifier(
    grid: &mut Vec<Vec<i32>>,
    air_purifier_y_pos: &Vec<usize>,
    r: usize,
    c: usize,
) {
    // Top purifier
    // Down
    for i in (0..=air_purifier_y_pos[0] - 2).rev() {
        grid[i + 1][0] = grid[i][0];
    }
    // Left
    for i in 0..c - 1 {
        grid[0][i] = grid[0][i + 1];
    }
    // Up
    for i in 0..air_purifier_y_pos[0] {
        grid[i][c - 1] = grid[i + 1][c - 1];
    }
    // Right
    for i in (0..=c - 2).rev() {
        grid[air_purifier_y_pos[0]][i + 1] = grid[air_purifier_y_pos[0]][i];
    }

    grid[air_purifier_y_pos[0]][1] = 0;

    // Bottom purifier
    // Up
    for i in air_purifier_y_pos[1] + 1..r - 1 {
        grid[i][0] = grid[i + 1][0];
    }
    // Left
    for i in 0..c - 1 {
        grid[r - 1][i] = grid[r - 1][i + 1];
    }
    // Down
    for i in (air_purifier_y_pos[1]..=r - 2).rev() {
        grid[i + 1][c - 1] = grid[i][c - 1];
    }
    // Right
    for i in (0..=c - 2).rev() {
        grid[air_purifier_y_pos[1]][i + 1] = grid[air_purifier_y_pos[1]][i];
    }

    grid[air_purifier_y_pos[1]][1] = 0;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (r, c, t) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut grid = vec![vec![0; c]; r];
    let mut air_purifier_y_pos = vec![0, 0];

    for i in 0..r {
        for j in 0..c {
            grid[i][j] = scan.token::<i32>();

            if grid[i][j] == -1 {
                if air_purifier_y_pos[0] == 0 {
                    air_purifier_y_pos[0] = i;
                } else {
                    air_purifier_y_pos[1] = i;
                }
            }
        }
    }

    for _ in 0..t {
        diffuse_dust(&mut grid, r, c);
        work_air_purifier(&mut grid, &air_purifier_y_pos, r, c);
    }

    let total_dust = grid.iter().map(|row| row.iter().sum::<i32>()).sum::<i32>();
    writeln!(out, "{}", total_dust + 2).unwrap();
}

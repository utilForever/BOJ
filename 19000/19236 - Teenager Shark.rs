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

#[derive(Clone, Debug, Default)]
struct Fish {
    idx: i64,
    x: usize,
    y: usize,
    direction: Option<usize>,
}

fn explore(mut fishes: Vec<Fish>, mut grid: Vec<Vec<Option<usize>>>, x: usize, y: usize) -> i64 {
    let dx = [-1, -1, 0, 1, 1, 1, 0, -1];
    let dy = [0, -1, -1, -1, 0, 1, 1, 1];

    // Step 1: Adult shark eats the fish and change direction
    let idx_fish_to_eat = grid[x][y].unwrap();
    let shark_direction = fishes[idx_fish_to_eat].direction.unwrap();
    let mut ret = fishes[idx_fish_to_eat].idx as i64;

    fishes[idx_fish_to_eat].direction = None;
    grid[x][y] = None;

    // Step 2: Move all fishes according to direction
    for i in 0..16 {
        // Skip if fish is already eaten
        if fishes[i].direction.is_none() {
            continue;
        }

        let fish_direction = fishes[i].direction.unwrap();
        let mut can_move = false;
        let mut move_to_next = (0, 0);

        for j in 0..8 {
            let (x_next, y_next) = (
                fishes[i].x as i64 + dx[(fish_direction + j) % 8],
                fishes[i].y as i64 + dy[(fish_direction + j) % 8],
            );

            if x_next < 0 || x_next >= 4 || y_next < 0 || y_next >= 4 {
                continue;
            }

            if x_next != x as i64 || y_next != y as i64 {
                can_move = true;
                fishes[i].direction = Some((fish_direction + j) % 8);
                move_to_next = (x_next as usize, y_next as usize);
                break;
            }
        }

        if !can_move {
            continue;
        }

        let (move_x, move_y) = move_to_next;

        if grid[move_x][move_y].is_some() {
            fishes[grid[move_x][move_y].unwrap()].x = fishes[i].x;
            fishes[grid[move_x][move_y].unwrap()].y = fishes[i].y;
        }

        grid[fishes[i].x][fishes[i].y] = grid[move_x][move_y];
        grid[move_x][move_y] = Some(fishes[i].idx as usize - 1);
        fishes[i].x = move_x;
        fishes[i].y = move_y;
    }

    // Step 3: Backtracking
    let (mut x_next, mut y_next) = (x as i64, y as i64);

    loop {
        x_next += dx[shark_direction];
        y_next += dy[shark_direction];

        if x_next < 0 || x_next >= 4 || y_next < 0 || y_next >= 4 {
            break;
        }

        let x_next = x_next as usize;
        let y_next = y_next as usize;

        if grid[x_next][y_next].is_none() {
            continue;
        }

        ret = ret.max(
            fishes[idx_fish_to_eat].idx + explore(fishes.clone(), grid.clone(), x_next, y_next),
        );
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut fishes = vec![Fish::default(); 16];
    let mut grid = vec![vec![None; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

            fishes[a - 1] = Fish {
                idx: a as i64,
                x: i,
                y: j,
                direction: Some(b - 1),
            };
            grid[i][j] = Some(a - 1);
        }
    }

    writeln!(out, "{}", explore(fishes.clone(), grid.clone(), 0, 0)).unwrap();
}

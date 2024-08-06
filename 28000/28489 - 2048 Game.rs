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

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn alpha_beta(
    grid: &mut Vec<i64>,
    depth: i64,
    mut alpha: i64,
    mut beta: i64,
    max_p: bool,
    moved: bool,
    h: &dyn Fn(&Vec<i64>) -> i64,
) -> (Vec<i64>, i64) {
    if depth <= 0 || !moved {
        return (grid.clone(), h(grid));
    }

    let empty_tiles = grid.iter().filter(|&val| *val == 0).count();
    let new_depth = if empty_tiles > 8 { depth / 2 } else { depth };

    if max_p {
        let mut best: (Vec<i64>, i64) = (Vec::new(), std::i64::MIN);
        let possible_moves = get_possible_moves(grid);

        for direction in possible_moves.iter() {
            let ref mut grid_new = grid.clone();
            let moved = can_move(grid_new, *direction);
            do_move(grid_new, *direction);
            let (_, val) =
                alpha_beta(grid_new, new_depth - 1, alpha, beta, false, moved, h);

            if best.1 < val {
                best = (grid_new.clone(), val);
            }

            alpha = alpha.max(best.1);

            if beta <= alpha {
                break;
            }
        }

        return best;
    } else {
        let mut best: (Vec<i64>, i64) = (Vec::new(), std::i64::MAX);

        for (c, v) in grid.iter().enumerate() {
            if *v != 0 {
                continue;
            }

            let ref mut grid_candidate = grid.clone();
            grid_candidate[c] = 2;

            let (_, val) = alpha_beta(grid_candidate, new_depth - 1, alpha, beta, true, true, h);

            if best.1 > val {
                best = (Vec::new(), val);
            }

            beta = beta.min(best.1);

            if beta <= alpha {
                break;
            }
        }

        return best;
    }
}

struct Iter2048 {
    start: i64,
    step: i64,
    off: i64,
    count: i64,
}

impl Iter2048 {
    fn new(dir: Direction, off: i64, skip: i64) -> Iter2048 {
        match dir {
            Direction::Up => Iter2048 {
                start: 0,
                step: 4,
                off: off,
                count: skip,
            },
            Direction::Down => Iter2048 {
                start: 12,
                step: -4,
                off: off,
                count: skip,
            },
            Direction::Left => Iter2048 {
                start: 0,
                step: 1,
                off: off * 4,
                count: skip,
            },
            Direction::Right => Iter2048 {
                start: 3,
                step: -1,
                off: off * 4,
                count: skip,
            },
        }
    }
}

impl Iterator for Iter2048 {
    type Item = (i64, usize);

    fn next(&mut self) -> Option<(i64, usize)> {
        if self.count < 4 {
            let pair = (
                self.count,
                (self.start + self.count * self.step + self.off) as usize,
            );

            self.count += 1;

            Some(pair)
        } else {
            None
        }
    }
}

fn get_possible_moves(grid: &Vec<i64>) -> Vec<Direction> {
    let mut ret = Vec::new();

    if can_move(&mut grid.clone(), Direction::Up) {
        ret.push(Direction::Up);
    }

    if can_move(&mut grid.clone(), Direction::Down) {
        ret.push(Direction::Down);
    }

    if can_move(&mut grid.clone(), Direction::Left) {
        ret.push(Direction::Left);
    }

    if can_move(&mut grid.clone(), Direction::Right) {
        ret.push(Direction::Right);
    }

    ret
}

fn can_move(grid: &Vec<i64>, dir: Direction) -> bool {
    let mut grid_new = grid.clone();

    for line in 0..4 {
        squash_line(&mut grid_new, dir, line);
        merge_line(&mut grid_new, dir, line);
        squash_line(&mut grid_new, dir, line);
    }

    return grid.iter().zip(grid_new.iter()).any(|(a, b)| *a != *b);
}

fn get_move_direction(grid: &Vec<i64>, grid_new: &Vec<i64>) -> Direction {
    if can_move(grid, Direction::Up) {
        let mut grid_up = grid.clone();
        do_move(&mut grid_up, Direction::Up);

        // println!("{:?} {:?}", grid_up, grid_new);

        if grid_up == *grid_new {
            return Direction::Up;
        }
    }

    if can_move(grid, Direction::Down) {
        let mut grid_down = grid.clone();
        do_move(&mut grid_down, Direction::Down);

        // println!("{:?} {:?}", grid_down, grid_new);

        if grid_down == *grid_new {
            return Direction::Down;
        }
    }

    if can_move(grid, Direction::Left) {
        let mut grid_left = grid.clone();
        do_move(&mut grid_left, Direction::Left);

        // println!("{:?} {:?}", grid_left, grid_new);

        if grid_left == *grid_new {
            return Direction::Left;
        }
    }

    if can_move(grid, Direction::Right) {
        let mut grid_right = grid.clone();
        do_move(&mut grid_right, Direction::Right);

        // println!("{:?} {:?}", grid_right, grid_new);

        if grid_right == *grid_new {
            return Direction::Right;
        }
    }

    return Direction::Up;
}

fn do_move(grid: &mut Vec<i64>, dir: Direction) {
    for line in 0..4 {
        squash_line(grid, dir, line);
        merge_line(grid, dir, line);
        squash_line(grid, dir, line);
    }
}

fn squash_line(grid: &mut Vec<i64>, dir: Direction, line: i64) {
    for (n, i) in Iter2048::new(dir, line, 0) {
        if grid[i] != 0 {
            continue;
        }

        for (_, j) in Iter2048::new(dir, line, n) {
            if grid[j] != 0 {
                grid[i] = grid[j];
                grid[j] = 0;
                break;
            }
        }
    }
}

fn merge_line(grid: &mut Vec<i64>, dir: Direction, line: i64) {
    let mut prev: Option<usize> = None;

    for (_, i) in Iter2048::new(dir, line, 0) {
        match prev {
            Some(p) => {
                if grid[p] == grid[i] {
                    grid[p] = grid[p] * 2;
                    grid[i] = 0;
                }
            }
            None => (),
        };

        prev = Some(i);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let mut grid = vec![0_i64; 16];
    let call = |grid: &Vec<i64>| -> i64 {
        let mut cnt_empty = 0;
        let mut sum: i64 = 0;
        let snake: [i64; 16] = [
            150, 100, 70, 60, 10, 15, 25, 40, -5, -15, -17, -20, -40, -38, -35, -30,
        ];

        for i in 0..4 {
            for j in 0..4 {
                sum += grid[i * 4 + j] * snake[i * 4 + j];

                if grid[i * 4 + j] == 0 {
                    cnt_empty += 1;
                }
            }
        }

        sum + cnt_empty * cnt_empty * 10
    };

    loop {
        let pos = scan.token::<i64>();

        if pos == -1 {
            break;
        }

        let pos = pos as usize - 1;
        grid[pos] = 2;

        let (grid_new, _) = alpha_beta(
            &mut grid,
            9,
            std::i64::MIN,
            std::i64::MAX,
            true,
            true,
            &call,
        );

        let direction = get_move_direction(&grid, &grid_new);
        grid = grid_new;

        match direction {
            Direction::Up => println!("UP"),
            Direction::Down => println!("DOWN"),
            Direction::Left => println!("LEFT"),
            Direction::Right => println!("RIGHT"),
        }
    }
}

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

#[derive(Copy, Clone, Debug)]
struct Cell {
    shark: i64,
    smell: i64,
}

impl Cell {
    fn new(shark: i64, smell: i64) -> Self {
        Self { shark, smell }
    }
}

#[derive(Copy, Clone)]
struct Shark {
    alive: bool,
    y: usize,
    x: usize,
    direction: Direction,
}

impl Shark {
    fn new(y: usize, x: usize, direction: i64) -> Self {
        Self {
            alive: true,
            y,
            x,
            direction: Direction::from(direction),
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Unknown = 0,
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

impl From<i64> for Direction {
    fn from(item: i64) -> Self {
        match item {
            0 => Direction::Unknown,
            1 => Direction::Up,
            2 => Direction::Down,
            3 => Direction::Left,
            4 => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

impl Into<i64> for Direction {
    fn into(self) -> i64 {
        match self {
            Direction::Unknown => 0,
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 3,
            Direction::Right => 4,
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
    let mut grid = vec![vec![Cell::new(0, 0); n]; n];
    let mut sharks = vec![Shark::new(0, 0, 0); m];
    let mut directions = vec![vec![vec![Direction::from(0); 4]; 4]; m];

    for i in 0..n {
        for j in 0..n {
            grid[i][j] = Cell::new(scan.token::<i64>(), 0);

            if grid[i][j].shark != 0 {
                sharks[grid[i][j].shark as usize - 1] = Shark::new(i, j, 0);
                grid[i][j].smell = k;
            }
        }
    }

    for i in 0..m {
        sharks[i].direction = Direction::from(scan.token::<i64>());
    }

    for i in 0..m {
        for j in 0..4 {
            for k in 0..4 {
                directions[i][j][k] = Direction::from(scan.token::<i64>());
            }
        }
    }

    let mut time = 0;
    let mut num_shark = m;

    while time <= 1000 && num_shark > 1 {
        let mut sharks_next = Vec::new();

        for (idx, shark) in sharks.iter_mut().enumerate() {
            if !shark.alive {
                continue;
            }

            let (cur_y, cur_x) = (shark.y, shark.x);
            let mut candidates = Vec::new();

            for direction in directions[idx][shark.direction as usize - 1].iter() {
                let (next_y, next_x) = match direction {
                    Direction::Up => (shark.y as i64 - 1, shark.x as i64),
                    Direction::Down => (shark.y as i64 + 1, shark.x as i64),
                    Direction::Left => (shark.y as i64, shark.x as i64 - 1),
                    Direction::Right => (shark.y as i64, shark.x as i64 + 1),
                    _ => panic!("Invalid direction"),
                };

                if next_y < 0 || next_y >= n as i64 || next_x < 0 || next_x >= n as i64 {
                    continue;
                }

                if grid[next_y as usize][next_x as usize].shark == 0 {
                    candidates.push((*direction, next_y, next_x, 0));
                } else if grid[next_y as usize][next_x as usize].shark == grid[cur_y][cur_x].shark {
                    candidates.push((*direction, next_y, next_x, 1));
                }
            }

            let can_move_my_area_only = candidates
                .iter()
                .all(|(_, _, _, is_my_area)| *is_my_area == 1);

            if !can_move_my_area_only {
                candidates.retain(|(_, _, _, is_my_area)| *is_my_area == 0);
            }

            sharks_next.push((idx, candidates[0]));
        }

        for i in 0..n {
            for j in 0..n {
                if grid[i][j].shark != 0 {
                    grid[i][j].smell -= 1;

                    if grid[i][j].smell == 0 {
                        grid[i][j].shark = 0;
                    }
                }
            }
        }

        for shark in sharks_next.iter_mut() {
            let (y, x) = (shark.1 .1 as usize, shark.1 .2 as usize);

            if grid[y][x].shark == shark.0 as i64 + 1 {
                grid[y][x].shark = shark.0 as i64 + 1;
                grid[y][x].smell = k;

                sharks[shark.0].y = y;
                sharks[shark.0].x = x;
                sharks[shark.0].direction = shark.1 .0;
            } else if grid[y][x].shark != 0 {
                if grid[y][x].shark > shark.0 as i64 + 1 {
                    sharks[grid[y][x].shark as usize - 1].alive = false;

                    grid[y][x].shark = shark.0 as i64 + 1;
                    grid[y][x].smell = k;

                    sharks[shark.0].y = y;
                    sharks[shark.0].x = x;
                    sharks[shark.0].direction = shark.1 .0;
                } else {
                    sharks[shark.0].alive = false;
                }

                num_shark -= 1;
            } else {
                grid[y][x].shark = shark.0 as i64 + 1;
                grid[y][x].smell = k;

                sharks[shark.0].y = y;
                sharks[shark.0].x = x;
                sharks[shark.0].direction = shark.1 .0;
            }
        }

        time += 1;
    }

    writeln!(out, "{}", if time > 1000 { -1 } else { time }).unwrap();
}

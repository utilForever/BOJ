use io::Write;
use std::{collections::VecDeque, io, str};

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

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
enum State {
    Empty,
    Wall,
    Sheep,
    Wolf,
}

fn process_bfs(
    land: &Vec<Vec<State>>,
    visited: &mut Vec<Vec<bool>>,
    r: usize,
    c: usize,
) -> (i64, i64) {
    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];
    let mut delete_sheep = 0;
    let mut delete_wolf = 0;

    for i in 0..r {
        for j in 0..c {
            if visited[i][j] || land[i][j] == State::Wall {
                continue;
            }

            let mut queue = VecDeque::new();
            let mut cnt_sheep = 0;
            let mut cnt_wolf = 0;

            queue.push_back((i, j));
            visited[i][j] = true;

            if land[i][j] == State::Sheep {
                cnt_sheep += 1;
            } else if land[i][j] == State::Wolf {
                cnt_wolf += 1;
            }

            while !queue.is_empty() {
                let (x, y) = queue.pop_front().unwrap();

                for k in 0..4 {
                    let next_x = x as i64 + dx[k];
                    let next_y = y as i64 + dy[k];

                    if next_x < 0 || next_x >= r as i64 || next_y < 0 || next_y >= c as i64 {
                        continue;
                    }

                    let next_x = next_x as usize;
                    let next_y = next_y as usize;

                    if visited[next_x][next_y] || land[next_x][next_y] == State::Wall {
                        continue;
                    }

                    if land[next_x][next_y] == State::Sheep {
                        cnt_sheep += 1;
                    } else if land[next_x][next_y] == State::Wolf {
                        cnt_wolf += 1;
                    }

                    visited[next_x][next_y] = true;
                    queue.push_back((next_x, next_y));
                }
            }

            if cnt_sheep > cnt_wolf {
                delete_wolf += cnt_wolf;
            } else {
                delete_sheep += cnt_sheep;
            }
        }
    }

    (delete_sheep, delete_wolf)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut land = vec![vec![State::Empty; c]; r];
    let mut visited = vec![vec![false; c]; r];
    let mut cnt_sheep = 0;
    let mut cnt_wolf = 0;

    for i in 0..r {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            match c {
                '.' => land[i][j] = State::Empty,
                '#' => {
                    land[i][j] = State::Wall;
                    visited[i][j] = true;
                }
                'o' => {
                    land[i][j] = State::Sheep;
                    cnt_sheep += 1;
                }
                'v' => {
                    land[i][j] = State::Wolf;
                    cnt_wolf += 1;
                }
                _ => (),
            }
        }
    }

    let ret = process_bfs(&land, &mut visited, r, c);

    writeln!(out, "{} {}", cnt_sheep - ret.0, cnt_wolf - ret.1).unwrap();
}

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

const DX: [i64; 4] = [0, 0, -1, 1];
const DY: [i64; 4] = [-1, 1, 0, 0];

fn process_dfs(map: &mut Vec<Vec<char>>, keys: &mut [bool; 26], h: usize, w: usize) -> i64 {
    let mut visited = vec![vec![false; w + 2]; h + 2];
    let mut queue = VecDeque::new();
    let mut queue_door = vec![VecDeque::new(); 26];
    let mut ret = 0;

    queue.push_back((0, 0));
    visited[0][0] = true;

    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();

        for i in 0..4 {
            let y_next = y as i64 + DY[i];
            let x_next = x as i64 + DX[i];

            if y_next < 0 || y_next > h as i64 + 1 || x_next < 0 || x_next > w as i64 + 1 {
                continue;
            }

            let x_next = x_next as usize;
            let y_next = y_next as usize;

            // Check if the cell is a wall
            if map[y_next][x_next] == '*' {
                continue;
            }

            // Check if the cell is already visited
            if visited[y_next][x_next] {
                continue;
            }

            visited[y_next][x_next] = true;

            // Check if the cell is a door
            if map[y_next][x_next].is_ascii_uppercase() {
                let key = (map[y_next][x_next] as u8 - b'A') as usize;

                if keys[key] {
                    queue.push_back((y_next, x_next));
                } else {
                    queue_door[key].push_back((y_next, x_next));
                }

                continue;
            }

            // Check if the cell is a key
            if map[y_next][x_next].is_ascii_lowercase() {
                let key = (map[y_next][x_next] as u8 - b'a') as usize;

                if !keys[key] {
                    keys[key] = true;

                    while !queue_door[key].is_empty() {
                        let (y_door, x_door) = queue_door[key].pop_front().unwrap();
                        queue.push_back((y_door, x_door));
                    }
                }
            }

            if map[y_next][x_next] == '$' {
                ret += 1;
            }

            queue.push_back((y_next, x_next));
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
        let mut map = vec![vec!['.'; w + 2]; h + 2];
        let mut keys = [false; 26];

        for i in 1..=h {
            let line = scan.token::<String>();

            for (j, c) in line.chars().enumerate() {
                map[i][j + 1] = c;
            }
        }

        let line = scan.token::<String>();

        for key in line.chars() {
            if key == '0' {
                break;
            }

            keys[(key as u8 - b'a') as usize] = true;
        }

        writeln!(out, "{}", process_dfs(&mut map, &mut keys, h, w)).unwrap();
    }
}

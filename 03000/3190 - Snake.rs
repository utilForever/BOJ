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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut board = vec![vec![0; n]; n];

    for _ in 0..k {
        let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
        board[x - 1][y - 1] = 1;
    }

    let l = scan.token::<usize>();
    let mut direction_info = vec![(0, String::new()); l];

    for i in 0..l {
        direction_info[i] = (scan.token::<i64>(), scan.token::<String>());
    }

    // Down, Right, Up, Left
    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];

    let mut queue = VecDeque::new();
    let mut idx_direction = 1;
    let mut is_collide = false;
    let mut ret = 0;

    queue.push_back((0, 0));

    loop {
        ret += 1;

        let (x, y) = queue.front().unwrap();
        let (next_x, next_y) = (x + dx[idx_direction], y + dy[idx_direction]);

        for val in queue.iter() {
            if val.0 == next_x && val.1 == next_y {
                is_collide = true;
                break;
            }
        }

        if is_collide || next_x < 0 || next_x >= n as i64 || next_y < 0 || next_y >= n as i64 {
            break;
        }

        queue.push_front((next_x, next_y));

        if board[next_x as usize][next_y as usize] == 0 {
            queue.pop_back();
        } else {
            board[next_x as usize][next_y as usize] = 0;
        }

        for direction in direction_info.iter() {
            if direction.0 == ret {
                if direction.1 == "D" {
                    idx_direction = (idx_direction + 3) % 4;
                } else if direction.1 == "L" {
                    idx_direction = (idx_direction + 1) % 4;
                }
            }
        }
    }

    writeln!(out, "{}", ret).unwrap();
}

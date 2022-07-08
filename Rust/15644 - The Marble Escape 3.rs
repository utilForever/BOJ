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

#[derive(Clone, Debug, Eq, PartialEq)]
enum Type {
    Empty,
    Wall,
    Hole,
    BeadRed,
    BeadBlue,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut board = vec![vec![Type::Empty; m]; n];
    let mut visited_beads = vec![vec![vec![vec![false; m]; n]; m]; n];
    let mut direction_bead = vec![vec![vec![vec![((0, 0), (0, 0), 0); m]; n]; m]; n];
    let mut pos_bead_red = (0, 0);
    let mut pos_bead_blue = (0, 0);

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            let t = match c {
                '#' => Type::Wall,
                '.' => Type::Empty,
                'O' => Type::Hole,
                'R' => Type::BeadRed,
                'B' => Type::BeadBlue,
                _ => unreachable!(),
            };

            if t == Type::BeadRed {
                pos_bead_red = (i as i64, j as i64);
            } else if t == Type::BeadBlue {
                pos_bead_blue = (i as i64, j as i64);
            }

            board[i][j] = t;
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((pos_bead_red, pos_bead_blue, 0));

    visited_beads[pos_bead_red.0 as usize][pos_bead_red.1 as usize][pos_bead_blue.0 as usize]
        [pos_bead_blue.1 as usize] = true;

    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];
    let direction: [char; 4] = ['D', 'R', 'U', 'L'];

    while !queue.is_empty() {
        let (pos_bead_red, pos_bead_blue, turn) = queue.pop_front().unwrap();

        if turn >= 10 {
            break;
        }

        for i in 0..4 {
            let mut next_pos_bead_red = (pos_bead_red.0 as i64, pos_bead_red.1 as i64);
            let mut next_pos_bead_blue = (pos_bead_blue.0 as i64, pos_bead_blue.1 as i64);
            let mut num_move_bead_red = 0;
            let mut num_move_bead_blue = 0;

            let cond_pos = |pos_x: i64, pos_y: i64| {
                pos_x >= 0 && pos_x < n as i64 && pos_y >= 0 && pos_y < m as i64
            };

            if !cond_pos(next_pos_bead_red.0, next_pos_bead_red.1)
                || !cond_pos(next_pos_bead_blue.0, next_pos_bead_blue.1)
            {
                continue;
            }

            while (cond_pos(next_pos_bead_red.0 + dx[i], next_pos_bead_red.1 + dy[i])
                && board[(next_pos_bead_red.0 + dx[i]) as usize]
                    [(next_pos_bead_red.1 + dy[i]) as usize]
                    != Type::Wall)
                && (cond_pos(next_pos_bead_red.0, next_pos_bead_red.1)
                    && board[next_pos_bead_red.0 as usize][next_pos_bead_red.1 as usize]
                        != Type::Hole)
            {
                next_pos_bead_red = (next_pos_bead_red.0 + dx[i], next_pos_bead_red.1 + dy[i]);
                num_move_bead_red += 1;
            }

            while (cond_pos(next_pos_bead_blue.0 + dx[i], next_pos_bead_blue.1 + dy[i])
                && board[(next_pos_bead_blue.0 + dx[i]) as usize]
                    [(next_pos_bead_blue.1 + dy[i]) as usize]
                    != Type::Wall)
                && (cond_pos(next_pos_bead_blue.0, next_pos_bead_blue.1)
                    && board[next_pos_bead_blue.0 as usize][next_pos_bead_blue.1 as usize]
                        != Type::Hole)
            {
                next_pos_bead_blue = (next_pos_bead_blue.0 + dx[i], next_pos_bead_blue.1 + dy[i]);
                num_move_bead_blue += 1;
            }

            if board[next_pos_bead_blue.0 as usize][next_pos_bead_blue.1 as usize] == Type::Hole {
                continue;
            }

            if board[next_pos_bead_red.0 as usize][next_pos_bead_red.1 as usize] == Type::Hole {
                let mut path = Vec::new();
                path.push(direction[i]);

                let mut bead = direction_bead[pos_bead_red.0 as usize][pos_bead_red.1 as usize]
                    [pos_bead_blue.0 as usize][pos_bead_blue.1 as usize];

                while bead.0 .0 != 0 {
                    let prev_pos_bead_red = bead.0;
                    let prev_pos_bead_blue = bead.1;

                    path.push(direction[bead.2]);
                    bead = direction_bead[prev_pos_bead_red.0 as usize]
                        [prev_pos_bead_red.1 as usize][prev_pos_bead_blue.0 as usize]
                        [prev_pos_bead_blue.1 as usize];
                }

                writeln!(out, "{}", turn + 1).unwrap();
                for c in path.iter().rev() {
                    write!(out, "{}", c).unwrap();
                }
                writeln!(out).unwrap();
                return;
            }

            if next_pos_bead_red == next_pos_bead_blue {
                if num_move_bead_red > num_move_bead_blue {
                    next_pos_bead_red.0 -= dx[i];
                    next_pos_bead_red.1 -= dy[i];
                } else {
                    next_pos_bead_blue.0 -= dx[i];
                    next_pos_bead_blue.1 -= dy[i];
                }
            }

            if visited_beads[next_pos_bead_red.0 as usize][next_pos_bead_red.1 as usize]
                [next_pos_bead_blue.0 as usize][next_pos_bead_blue.1 as usize]
            {
                continue;
            }

            visited_beads[next_pos_bead_red.0 as usize][next_pos_bead_red.1 as usize]
                [next_pos_bead_blue.0 as usize][next_pos_bead_blue.1 as usize] = true;
            direction_bead[next_pos_bead_red.0 as usize][next_pos_bead_red.1 as usize]
                [next_pos_bead_blue.0 as usize][next_pos_bead_blue.1 as usize] =
                (pos_bead_red, pos_bead_blue, i);

            queue.push_back((next_pos_bead_red, next_pos_bead_blue, turn + 1));
        }
    }

    writeln!(out, "-1").unwrap();
}

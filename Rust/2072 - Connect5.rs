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
    Right,
    Down,
    RightDown,
    LeftDown,
}

fn is_middle(
    board: &Vec<Vec<u8>>,
    r: usize,
    c: usize,
    stone_color: u8,
    direction: Direction,
) -> bool {
    match direction {
        Direction::Right => {
            if c == board[r].len() - 1 {
                return false;
            }
            if board[r][c - 1] == stone_color {
                return false;
            }
        }
        Direction::Down => {
            if r == board.len() - 1 {
                return false;
            }
            if board[r - 1][c] == stone_color {
                return false;
            }
        }
        Direction::RightDown => {
            if r == board.len() - 1 || c == board[r].len() - 1 {
                return false;
            }
            if board[r - 1][c - 1] == stone_color {
                return false;
            }
        }
        Direction::LeftDown => {
            if r == board.len() - 1 || c == 0 {
                return false;
            }
            if board[r - 1][c + 1] == stone_color {
                return false;
            }
        }
    }

    true
}

fn is_connect5(
    board: &Vec<Vec<u8>>,
    r: usize,
    c: usize,
    stone_color: u8,
    direction: Direction,
) -> bool {
    if !is_middle(board, r, c, stone_color, direction.clone()) {
        return false;
    }

    let mut stones = Vec::new();
    stones.push((r, c));

    let mut d_row = r;
    let mut d_col = c;

    match direction {
        Direction::Right => {
            d_col += 1;

            while board[d_row][d_col] == stone_color {
                stones.push((d_row, d_col));
                d_col += 1;
            }
        }
        Direction::Down => {
            d_row += 1;

            while board[d_row][d_col] == stone_color {
                stones.push((d_row, d_col));
                d_row += 1;
            }
        }
        Direction::RightDown => {
            d_row += 1;
            d_col += 1;

            while board[d_row][d_col] == stone_color {
                stones.push((d_row, d_col));
                d_row += 1;
                d_col += 1;
            }
        }
        Direction::LeftDown => {
            d_row += 1;
            d_col -= 1;

            while board[d_row][d_col] == stone_color {
                stones.push((d_row, d_col));
                d_row += 1;
                d_col -= 1;
            }
        }
    }

    if stones.len() == 5 {
        true
    } else {
        false
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut board = vec![vec![0; 21]; 21];
    let mut turns = vec![(0, 0); n + 1];
    let mut stone_curr = 1;

    for i in 1..=n {
        turns[i] = (scan.token::<usize>(), scan.token::<usize>());
    }

    for i in 1..=n {
        let (r, c) = turns[i];
        board[r][c] = stone_curr;

        for r in 1..=19 {
            for c in 1..=19 {
                let (r, c) = (r as usize, c as usize);
                let stone_color = board[r][c];

                if stone_color != 0 {
                    // Right
                    if stone_color == board[r][c + 1] {
                        if is_connect5(&board, r, c, stone_color, Direction::Right) {
                            writeln!(out, "{i}").unwrap();
                            return;
                        }
                    }

                    // Down
                    if stone_color == board[r + 1][c] {
                        if is_connect5(&board, r, c, stone_color, Direction::Down) {
                            writeln!(out, "{i}").unwrap();
                            return;
                        }
                    }

                    // Right-Down
                    if stone_color == board[r + 1][c + 1] {
                        if is_connect5(&board, r, c, stone_color, Direction::RightDown) {
                            writeln!(out, "{i}").unwrap();
                            return;
                        }
                    }

                    // Left-Down
                    if stone_color == board[r + 1][c - 1] {
                        if is_connect5(&board, r, c, stone_color, Direction::LeftDown) {
                            writeln!(out, "{i}").unwrap();
                            return;
                        }
                    }
                }
            }
        }

        stone_curr = if stone_curr == 1 { 2 } else { 1 };
    }

    writeln!(out, "-1").unwrap();
}

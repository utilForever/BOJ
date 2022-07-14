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

fn move_blocks(board: &mut [[i64; 21]; 21], n: usize, direction: usize) -> i64 {
    let mut ret = 0;

    match direction {
        // Up
        0 => {
            for i in 0..n {
                let mut val = -1;
                let mut height = -1_i64;

                for j in 0..n {
                    if board[j][i] == 0 {
                        continue;
                    }

                    if board[j][i] == val {
                        board[height as usize][i] *= 2;
                        val = -1;
                    } else {
                        val = board[j][i];
                        height += 1;
                        board[height as usize][i] = board[j][i];
                    }

                    ret = ret.max(board[height as usize][i]);
                }

                for j in (height as usize + 1)..n {
                    board[j][i] = 0;
                }
            }
        }
        // Left
        1 => {
            for i in 0..n {
                let mut val = -1;
                let mut width = -1_i64;

                for j in 0..n {
                    if board[i][j] == 0 {
                        continue;
                    }

                    if board[i][j] == val {
                        board[i][width as usize] *= 2;
                        val = -1;
                    } else {
                        val = board[i][j];
                        width += 1;
                        board[i][width as usize] = board[i][j];
                    }

                    ret = ret.max(board[i][width as usize]);
                }

                for j in (width as usize + 1)..n {
                    board[i][j] = 0;
                }
            }
        }
        // Down
        2 => {
            for i in 0..n {
                let mut val = -1;
                let mut height = n as i64;

                for j in (0..n).rev() {
                    if board[j][i] == 0 {
                        continue;
                    }

                    if board[j][i] == val {
                        board[height as usize][i] *= 2;
                        val = -1;
                    } else {
                        val = board[j][i];
                        height -= 1;
                        board[height as usize][i] = board[j][i];
                    }

                    ret = ret.max(board[height as usize][i]);
                }

                for j in (0..(height as usize)).rev() {
                    board[j][i] = 0;
                }
            }
        }
        // Right
        3 => {
            for i in 0..n {
                let mut val = -1;
                let mut width = n as i64;

                for j in (0..n).rev() {
                    if board[i][j] == 0 {
                        continue;
                    }

                    if board[i][j] == val {
                        board[i][width as usize] *= 2;
                        val = -1;
                    } else {
                        val = board[i][j];
                        width -= 1;
                        board[i][width as usize] = board[i][j];
                    }

                    ret = ret.max(board[i][width as usize]);
                }

                for j in (0..(width as usize)).rev() {
                    board[i][j] = 0;
                }
            }
        }
        _ => unreachable!(),
    }

    ret
}

fn process_game(board: &mut [[i64; 21]; 21], val_max: &mut i64, n: usize, num_turn: u32) {
    if num_turn == 10 {
        let ret = *board.iter().map(|v| v.iter().max().unwrap()).max().unwrap();
        *val_max = std::cmp::max(*val_max, ret);

        return;
    }

    for i in 0..4 {
        let mut board_new = board.clone();
        let ret = move_blocks(&mut board_new, n, i);

        if board_new != *board && *val_max < ret * 2_i64.pow(9 - num_turn) {
            process_game(&mut board_new, val_max, n, num_turn + 1);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut board = [[0; 21]; 21];

    for i in 0..n {
        for j in 0..n {
            board[i][j] = scan.token::<i64>();
        }
    }

    let mut ret = *board.iter().map(|v| v.iter().max().unwrap()).max().unwrap();

    process_game(&mut board, &mut ret, n, 0);

    writeln!(out, "{ret}").unwrap();
}

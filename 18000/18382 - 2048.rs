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

fn move_blocks(board: &mut [[i64; 4]; 4], n: usize, direction: usize) -> i64 {
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
                        ret += board[height as usize][i];
                        val = -1;
                    } else {
                        val = board[j][i];
                        height += 1;
                        board[height as usize][i] = board[j][i];
                    }
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
                        ret += board[i][width as usize];
                        val = -1;
                    } else {
                        val = board[i][j];
                        width += 1;
                        board[i][width as usize] = board[i][j];
                    }
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
                        ret += board[height as usize][i];
                        val = -1;
                    } else {
                        val = board[j][i];
                        height -= 1;
                        board[height as usize][i] = board[j][i];
                    }
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
                        ret += board[i][width as usize];
                        val = -1;
                    } else {
                        val = board[i][j];
                        width -= 1;
                        board[i][width as usize] = board[i][j];
                    }
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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<i64>();
    let m = scan.token::<String>();
    let mut commands = Vec::new();
    let mut board = [[0; 4]; 4];
    let mut ret = 0;

    if m.chars().nth(0).unwrap().is_ascii_alphabetic() {
        commands = m.chars().collect::<Vec<_>>();

        for i in 0..4 {
            for j in 0..4 {
                board[i][j] = scan.token::<i64>();
            }
        }
    } else {
        for i in 0..4 {
            for j in 0..4 {
                if i == 0 && j == 0 {
                    board[i][j] = m.parse::<i64>().unwrap();
                    continue;
                }

                board[i][j] = scan.token::<i64>();
            }
        }
    }

    for i in 0..commands.len() / 4 {
        let commands_sub = commands[i * 4..i * 4 + 4].iter().collect::<String>();
        let commands_sub = commands_sub.chars().collect::<Vec<_>>();

        ret += move_blocks(
            &mut board,
            4,
            match commands_sub[0] {
                'U' => 0,
                'L' => 1,
                'D' => 2,
                'R' => 3,
                _ => unreachable!(),
            },
        );

        let (val, row, col) = (
            commands_sub[1] as i64 - '0' as i64,
            commands_sub[2] as usize - '0' as usize,
            commands_sub[3] as usize - '0' as usize,
        );
        board[row][col] = val;
    }

    writeln!(out, "{}", s + ret).unwrap();
}

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

fn move_blocks(board: &mut Vec<Vec<i64>>, n: usize, direction: usize) {
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
                }

                for j in (0..(width as usize)).rev() {
                    board[i][j] = 0;
                }
            }
        }
        _ => unreachable!(),
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let (n, dir) = (scan.token::<usize>(), scan.token::<String>());
        let mut board = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                board[i][j] = scan.token::<i64>();
            }
        }

        move_blocks(
            &mut board,
            n,
            match dir.as_str() {
                "up" => 0,
                "left" => 1,
                "down" => 2,
                "right" => 3,
                _ => unreachable!(),
            },
        );

        writeln!(out, "Case #{i}:").unwrap();

        for i in 0..n {
            for j in 0..n {
                write!(out, "{} ", board[i][j]).unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}

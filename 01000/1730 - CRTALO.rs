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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let commands = scan.line().trim().to_string();

    let mut board = vec![vec!['.'; n]; n];
    let mut pos = (0, 0);

    for command in commands.chars() {
        let next = match command {
            'U' => (pos.0 as i64 - 1, pos.1 as i64),
            'D' => (pos.0 as i64 + 1, pos.1 as i64),
            'L' => (pos.0 as i64, pos.1 as i64 - 1),
            'R' => (pos.0 as i64, pos.1 as i64 + 1),
            _ => unreachable!(),
        };

        if next.0 < 0 || next.0 >= n as i64 || next.1 < 0 || next.1 >= n as i64 {
            continue;
        }

        let next = (next.0 as usize, next.1 as usize);

        match command {
            'U' | 'D' => {
                board[pos.0][pos.1] = if board[pos.0][pos.1] == '.' {
                    '|'
                } else if board[pos.0][pos.1] == '-' {
                    '+'
                } else {
                    board[pos.0][pos.1]
                };

                board[next.0][next.1] = if board[next.0][next.1] == '.' {
                    '|'
                } else if board[next.0][next.1] == '-' {
                    '+'
                } else {
                    board[next.0][next.1]
                };
            }
            'L' | 'R' => {
                board[pos.0][pos.1] = if board[pos.0][pos.1] == '.' {
                    '-'
                } else if board[pos.0][pos.1] == '|' {
                    '+'
                } else {
                    board[pos.0][pos.1]
                };

                board[next.0][next.1] = if board[next.0][next.1] == '.' {
                    '-'
                } else if board[next.0][next.1] == '|' {
                    '+'
                } else {
                    board[next.0][next.1]
                };
            }
            _ => unreachable!(),
        }

        pos = next;
    }

    for i in 0..n {
        for j in 0..n {
            write!(out, "{}", board[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}

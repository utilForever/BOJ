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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let board = scan.token::<String>();
    let mut board = board.chars().collect::<Vec<_>>();
    let mut idx = 0;

    while idx < board.len() {
        if idx + 4 <= board.len() && board[idx..idx + 4].iter().all(|x| *x == 'X') {
            for i in idx..idx + 4 {
                board[i] = 'A';
            }

            idx += 4;
        } else if idx + 2 <= board.len() && board[idx..idx + 2].iter().all(|x| *x == 'X') {
            for i in idx..idx + 2 {
                board[i] = 'B';
            }

            idx += 2;
        } else if board[idx] == '.' {
            idx += 1;
        } else {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    for val in board {
        write!(out, "{val}").unwrap();
    }

    writeln!(out).unwrap();
}

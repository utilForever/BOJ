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

    let (r, c) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
    let mut board = [[' '; 10]; 10];

    for i in 0..10 {
        let line = scan.token::<String>();

        for (j, c) in line.trim().chars().enumerate() {
            board[i][j] = c;
        }
    }

    let mut board_bombed = board.clone();

    for i in 0..10 {
        for j in 0..10 {
            if board[i][j] == 'o' {
                for k in 0..10 {
                    board_bombed[k][j] = 'o';
                }

                for k in 0..10 {
                    board_bombed[i][k] = 'o';
                }
            }
        }
    }

    let mut ret = i64::MAX;

    for i in 0..10 {
        for j in 0..10 {
            if board_bombed[i][j] == 'o' {
                continue;
            }

            ret = ret.min((i as i64 - r as i64).abs() + (j as i64 - c as i64).abs());
        }
    }

    writeln!(out, "{ret}").unwrap();
}

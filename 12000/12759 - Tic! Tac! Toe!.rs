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

    let mut player = scan.token::<i64>();
    let mut tic_tac_toe = [[0; 3]; 3];
    let mut ret = 0;

    for _ in 0..9 {
        let (r, c) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);

        if ret != 0 {
            continue;
        }

        tic_tac_toe[r][c] = player;

        // Check row
        if tic_tac_toe[r][0] == tic_tac_toe[r][1] && tic_tac_toe[r][1] == tic_tac_toe[r][2] {
            ret = player;
        }

        // Check column
        if tic_tac_toe[0][c] == tic_tac_toe[1][c] && tic_tac_toe[1][c] == tic_tac_toe[2][c] {
            ret = player;
        }

        // Check diagonal
        if (r == 0 && c == 0) || (r == 1 && c == 1) || (r == 2 && c == 2) {
            if tic_tac_toe[0][0] == tic_tac_toe[1][1] && tic_tac_toe[1][1] == tic_tac_toe[2][2] {
                ret = player;
            }
        }

        if (r == 0 && c == 2) || (r == 1 && c == 1) || (r == 2 && c == 0) {
            if tic_tac_toe[0][2] == tic_tac_toe[1][1] && tic_tac_toe[1][1] == tic_tac_toe[2][0] {
                ret = player;
            }
        }

        player = if player == 1 { 2 } else { 1 };
    }

    writeln!(out, "{ret}").unwrap();
}

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

fn check(sudoku: &Vec<Vec<usize>>, y: usize, x: usize, idx: usize) -> bool {
    for i in 0..9 {
        if sudoku[y][i] == idx || sudoku[i][x] == idx {
            return false;
        }
    }

    for i in (y / 3) * 3..(y / 3) * 3 + 3 {
        for j in (x / 3) * 3..(x / 3) * 3 + 3 {
            if sudoku[i][j] == idx {
                return false;
            }
        }
    }

    true
}

fn process_dfs(sudoku: &mut Vec<Vec<usize>>, zeros: &mut Vec<(usize, usize)>, idx: usize) -> bool {
    if idx == zeros.len() {
        return true;
    }

    let (y, x) = zeros[idx];

    for i in 1..=9 {
        if check(sudoku, y, x, i) {
            sudoku[y][x] = i;

            if process_dfs(sudoku, zeros, idx + 1) {
                return true;
            }

            sudoku[y][x] = 0;
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut sudoku = vec![vec![0; 9]; 9];
    let mut zeros = Vec::new();

    for i in 0..9 {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            sudoku[i][j] = c.to_digit(10).unwrap() as usize;

            if sudoku[i][j] == 0 {
                zeros.push((i, j));
            }
        }
    }

    process_dfs(&mut sudoku, &mut zeros, 0);

    for i in 0..9 {
        for j in 0..9 {
            write!(out, "{}", sudoku[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}

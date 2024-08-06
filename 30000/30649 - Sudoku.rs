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

    let mut sudoku = [[0; 9]; 9];
    let mut idx_row = 0;

    for i in 0..13 {
        let s = scan.token::<String>();

        if i == 0 || i == 4 || i == 8 || i == 12 {
            continue;
        }

        let mut idx_column = 0;

        for (j, c) in s.chars().enumerate() {
            if j == 0 || j == 4 || j == 8 || j == 12 {
                continue;
            }

            sudoku[idx_row][idx_column] = if c == '.' { 0 } else { c.to_digit(10).unwrap() };

            idx_column += 1;
        }

        idx_row += 1;
    }

    let mut is_valid = true;

    // Check rows
    'outer: for i in 0..9 {
        let mut row = [0; 10];

        for j in 0..9 {
            row[sudoku[i][j] as usize] += 1;
        }

        for j in 1..10 {
            if row[j] > 1 {
                is_valid = false;
                break 'outer;
            }
        }
    }

    // Check columns
    'outer: for i in 0..9 {
        let mut column = [0; 10];

        for j in 0..9 {
            column[sudoku[j][i] as usize] += 1;
        }

        for j in 1..10 {
            if column[j] > 1 {
                is_valid = false;
                break 'outer;
            }
        }
    }

    // Check 3x3 squares
    'outer: for i in 0..3 {
        for j in 0..3 {
            let mut square = [0; 10];

            for k in 0..3 {
                for l in 0..3 {
                    square[sudoku[i * 3 + k][j * 3 + l] as usize] += 1;
                }
            }

            for k in 1..10 {
                if square[k] > 1 {
                    is_valid = false;
                    break 'outer;
                }
            }
        }
    }

    writeln!(out, "{}", if is_valid { "OK" } else { "GRESKA" }).unwrap();
}

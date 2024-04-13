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

    let t = scan.token::<i64>();

    for i in 1..=t {
        let mut sudoku = [[0; 6]; 6];

        for i in 0..6 {
            for j in 0..6 {
                sudoku[i][j] = scan.token::<u8>();
            }
        }

        let mut is_valid = true;

        // Check rows
        'outer: for i in 0..6 {
            let mut row = [0; 7];

            for j in 0..6 {
                row[sudoku[i][j] as usize] += 1;
            }

            for j in 1..=6 {
                if row[j] > 1 {
                    is_valid = false;
                    break 'outer;
                }
            }
        }

        if !is_valid {
            writeln!(out, "Case#{i}: 0").unwrap();
            continue;
        }

        // Check columns
        'outer: for i in 0..6 {
            let mut column = [0; 7];

            for j in 0..6 {
                column[sudoku[j][i] as usize] += 1;
            }

            for j in 1..=6 {
                if column[j] > 1 {
                    is_valid = false;
                    break 'outer;
                }
            }
        }

        if !is_valid {
            writeln!(out, "Case#{i}: 0").unwrap();
            continue;
        }

        // Check diagonal
        let mut diagonal = [0; 7];

        for i in 0..6 {
            diagonal[sudoku[i][i] as usize] += 1;
        }

        for i in 1..=6 {
            if diagonal[i] > 1 {
                is_valid = false;
                break;
            }
        }

        if !is_valid {
            writeln!(out, "Case#{i}: 0").unwrap();
            continue;
        }

        diagonal = [0; 7];

        for i in 0..6 {
            diagonal[sudoku[i][5 - i] as usize] += 1;
        }

        for i in 1..=6 {
            if diagonal[i] > 1 {
                is_valid = false;
                break;
            }
        }

        if !is_valid {
            writeln!(out, "Case#{i}: 0").unwrap();
            continue;
        }

        // Check 2x3 squares
        'outer: for i in 0..3 {
            for j in 0..2 {
                let mut square = [0; 7];

                for k in 0..2 {
                    for l in 0..3 {
                        square[sudoku[i * 2 + k][j * 3 + l] as usize] += 1;
                    }
                }

                for k in 1..=6 {
                    if square[k] > 1 {
                        is_valid = false;
                        break 'outer;
                    }
                }
            }
        }

        writeln!(out, "Case#{i}: {}", if is_valid { 1 } else { 0 }).unwrap();
    }
}

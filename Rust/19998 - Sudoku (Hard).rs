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

// Using Backtracking + Knuth's Algorithm X + Dancing Links (DLX)
fn calculate_sudoku(
    sudoku: &mut [[u8; 9]; 9],
    exact_cover_row: &mut [[bool; 10]; 9],
    exact_cover_col: &mut [[bool; 10]; 9],
    exact_cover_square: &mut [[bool; 10]; 9],
    cnt: &mut usize,
) {
    if *cnt == 81 {
        // Print the result
        for i in 0..9 {
            for j in 0..9 {
                print!("{} ", sudoku[i][j]);
            }

            println!();
        }

        std::process::exit(0);
    }

    // Consider Row
    for i in 0..9 {
        for num in 1..=9 {
            // Check if the number is already used in the row
            if exact_cover_row[i][num] {
                continue;
            }

            let mut target = -1;
            let mut total_cnt = 0;

            for j in 0..9 {
                if sudoku[i][j] == 0
                    && !exact_cover_col[j][num]
                    && !exact_cover_square[(i / 3) * 3 + j / 3][num]
                {
                    total_cnt += 1;
                    target = j as i64;
                }
            }

            // If the total count is 0, then the exact cover is not satisfied
            if total_cnt == 0 {
                return;
            } else if total_cnt == 1 {
                // Cover DLX
                sudoku[i][target as usize] = num as u8;
                exact_cover_row[i][num] = true;
                exact_cover_col[target as usize][num] = true;
                exact_cover_square[(i / 3) * 3 + target as usize / 3][num] = true;
                *cnt += 1;

                // Backtracking
                calculate_sudoku(
                    sudoku,
                    exact_cover_row,
                    exact_cover_col,
                    exact_cover_square,
                    cnt,
                );

                // Uncover DLX
                sudoku[i][target as usize] = 0;
                exact_cover_row[i][num] = false;
                exact_cover_col[target as usize][num] = false;
                exact_cover_square[(i / 3) * 3 + target as usize / 3][num] = false;
                *cnt -= 1;

                return;
            }
        }
    }

    // Consider Column
    for j in 0..9 {
        for num in 1..=9 {
            // Check if the number is already used in the column
            if exact_cover_col[j][num] {
                continue;
            }

            let mut target = -1;
            let mut total_cnt = 0;

            for i in 0..9 {
                if sudoku[i][j] == 0
                    && !exact_cover_row[i][num]
                    && !exact_cover_square[i / 3 * 3 + j / 3][num]
                {
                    total_cnt += 1;
                    target = i as i64;
                }
            }

            // If the total count is 0, then the exact cover is not satisfied
            if total_cnt == 0 {
                return;
            } else if total_cnt == 1 {
                // Cover DLX
                sudoku[target as usize][j] = num as u8;
                exact_cover_row[target as usize][num] = true;
                exact_cover_col[j][num] = true;
                exact_cover_square[(target as usize / 3) * 3 + j / 3][num] = true;
                *cnt += 1;

                // Backtacking
                calculate_sudoku(
                    sudoku,
                    exact_cover_row,
                    exact_cover_col,
                    exact_cover_square,
                    cnt,
                );

                // Uncover DLX
                sudoku[target as usize][j] = 0;
                exact_cover_row[target as usize][num] = false;
                exact_cover_col[j][num] = false;
                exact_cover_square[(target as usize / 3) * 3 + j / 3][num] = false;
                *cnt -= 1;

                return;
            }
        }
    }

    // Consider Square
    for k in 0..9 {
        for num in 1..=9 {
            // Check if the number is already used in the square
            if exact_cover_square[k][num] {
                continue;
            }

            let mut target_row = -1;
            let mut target_col = -1;
            let mut total_cnt = 0;

            for i in (k / 3) * 3..(k / 3) * 3 + 3 {
                for j in (k % 3) * 3..(k % 3) * 3 + 3 {
                    if sudoku[i][j] == 0 && !exact_cover_row[i][num] && !exact_cover_col[j][num] {
                        total_cnt += 1;
                        target_row = i as i64;
                        target_col = j as i64;
                    }
                }
            }

            // If the total count is 0, then the exact cover is not satisfied
            if total_cnt == 0 {
                return;
            } else if total_cnt == 1 {
                // Cover DLX
                sudoku[target_row as usize][target_col as usize] = num as u8;
                exact_cover_row[target_row as usize][num] = true;
                exact_cover_col[target_col as usize][num] = true;
                exact_cover_square[k][num] = true;
                *cnt += 1;

                // Backtracking
                calculate_sudoku(
                    sudoku,
                    exact_cover_row,
                    exact_cover_col,
                    exact_cover_square,
                    cnt,
                );

                // Uncover DLX
                sudoku[target_row as usize][target_col as usize] = 0;
                exact_cover_row[target_row as usize][num] = false;
                exact_cover_col[target_col as usize][num] = false;
                exact_cover_square[k][num] = false;
                *cnt -= 1;

                return;
            }
        }
    }

    // Consider Cell
    for i in 0..9 {
        for j in 0..9 {
            // Check if the cell is already filled
            if sudoku[i][j] > 0 {
                continue;
            }

            let mut target = -1;
            let mut total_cnt = 0;

            for num in 1..=9 {
                if !exact_cover_row[i][num]
                    && !exact_cover_col[j][num]
                    && !exact_cover_square[(i / 3) * 3 + j / 3][num]
                {
                    total_cnt += 1;
                    target = num as i64;
                }
            }

            // If the total count is 0, then the exact cover is not satisfied
            if total_cnt == 0 {
                return;
            } else if total_cnt == 1 {
                // Cover DLX
                sudoku[i][j] = target as u8;
                exact_cover_row[i][target as usize] = true;
                exact_cover_col[j][target as usize] = true;
                exact_cover_square[(i / 3) * 3 + j / 3][target as usize] = true;
                *cnt += 1;

                // Backtacking
                calculate_sudoku(
                    sudoku,
                    exact_cover_row,
                    exact_cover_col,
                    exact_cover_square,
                    cnt,
                );

                // Uncover DLX
                sudoku[i][j] = 0;
                exact_cover_row[i][target as usize] = false;
                exact_cover_col[j][target as usize] = false;
                exact_cover_square[(i / 3) * 3 + j / 3][target as usize] = false;
                *cnt -= 1;

                return;
            }
        }
    }

    // Choose Random
    for i in 0..9 {
        for j in 0..9 {
            // Check if the cell is already filled
            if sudoku[i][j] > 0 {
                continue;
            }

            for num in 1..=9 {
                if !exact_cover_row[i][num]
                    && !exact_cover_col[j][num]
                    && !exact_cover_square[(i / 3) * 3 + j / 3][num]
                {
                    // Cover DLX
                    sudoku[i][j] = num as u8;
                    exact_cover_row[i][num] = true;
                    exact_cover_col[j][num] = true;
                    exact_cover_square[(i / 3) * 3 + j / 3][num] = true;
                    *cnt += 1;

                    // Backtracking
                    calculate_sudoku(
                        sudoku,
                        exact_cover_row,
                        exact_cover_col,
                        exact_cover_square,
                        cnt,
                    );

                    // Uncover DLX
                    sudoku[i][j] = 0;
                    exact_cover_row[i][num] = false;
                    exact_cover_col[j][num] = false;
                    exact_cover_square[(i / 3) * 3 + j / 3][num] = false;
                    *cnt -= 1;
                }
            }

            return;
        }
    }
}

// Reference: http://www.secmem.org/blog/2019/12/15/knuths-algorithm-x/
fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let mut sudoku = [[0; 9]; 9];
    let mut exact_cover_row = [[false; 10]; 9];
    let mut exact_cover_col = [[false; 10]; 9];
    let mut exact_cover_square = [[false; 10]; 9];
    let mut cnt = 0;

    for i in 0..9 {
        for j in 0..9 {
            let num = scan.token::<u8>();

            if num > 0 {
                cnt += 1;
                sudoku[i][j] = num;
                exact_cover_row[i][num as usize] = true;
                exact_cover_col[j][num as usize] = true;
                exact_cover_square[(i / 3) * 3 + j / 3][num as usize] = true;
            }
        }
    }

    // Calculate the sudoku
    // If a function returns false, the result is fail
    calculate_sudoku(
        &mut sudoku,
        &mut exact_cover_row,
        &mut exact_cover_col,
        &mut exact_cover_square,
        &mut cnt,
    );
}

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
        let n = scan.token::<usize>();
        let mut sudoku = vec![vec![0; n * n]; n * n];

        for i in 0..n * n {
            for j in 0..n * n {
                sudoku[i][j] = scan.token::<i64>();
            }
        }

        let mut is_valid = true;

        // Check rows
        'outer: for i in 0..n * n {
            let mut row = vec![0; 1001];

            for j in 0..n * n {
                row[sudoku[i][j] as usize] += 1;
            }

            if row.iter().skip(n * n + 1).any(|&x| x > 0) {
                is_valid = false;
                break;
            }

            for j in 1..=n * n {
                if row[j] > 1 {
                    is_valid = false;
                    break 'outer;
                }
            }
        }

        // Check columns
        'outer: for i in 0..n * n {
            let mut column = vec![0; 1001];

            for j in 0..n * n {
                column[sudoku[j][i] as usize] += 1;
            }

            if column.iter().skip(n * n + 1).any(|&x| x > 0) {
                is_valid = false;
                break;
            }

            for j in 1..=n * n {
                if column[j] > 1 {
                    is_valid = false;
                    break 'outer;
                }
            }
        }

        // Check NxN squares
        'outer: for i in 0..n {
            for j in 0..n {
                let mut square = vec![0; 1001];

                for k in 0..n {
                    for l in 0..n {
                        square[sudoku[i * n + k][j * n + l] as usize] += 1;
                    }
                }

                if square.iter().skip(n * n + 1).any(|&x| x > 0) {
                    is_valid = false;
                    break 'outer;
                }

                for k in 1..=n * n {
                    if square[k] > 1 {
                        is_valid = false;
                        break 'outer;
                    }
                }
            }
        }

        writeln!(out, "Case #{i}: {}", if is_valid { "Yes" } else { "No" }).unwrap();
    }
}

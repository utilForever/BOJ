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
        let mut minefield = vec![vec![' '; n]; n];

        for j in 0..n {
            let line = scan.token::<String>();

            for (k, c) in line.chars().enumerate() {
                minefield[j][k] = c;
            }
        }

        let count_mines = |minefield: &Vec<Vec<char>>, row_idx: usize, col_idx: usize| -> i64 {
            let mut count = 0;

            for i in row_idx.saturating_sub(1)..=row_idx + 1 {
                for j in col_idx.saturating_sub(1)..=col_idx + 1 {
                    if i < minefield.len() && j < minefield[0].len() && minefield[i][j] == '*' {
                        count += 1;
                    }
                }
            }

            count
        };

        let mut visited = vec![vec![false; n]; n];
        let mut ret = 0;

        for j in 0..n {
            for k in 0..n {
                if minefield[j][k] == '*' {
                    visited[j][k] = true;
                }
            }
        }

        for j in 0..n {
            for k in 0..n {
                if visited[j][k] {
                    continue;
                }

                let mines = count_mines(&minefield, j, k);

                if mines == 0 {
                    visited[j][k] = true;
                    ret += 1;

                    let mut queue = Vec::new();
                    let (row_idx, col_idx) = (j, k);

                    for j in row_idx.saturating_sub(1)..=row_idx + 1 {
                        for k in col_idx.saturating_sub(1)..=col_idx + 1 {
                            if j < minefield.len() && k < minefield[0].len() {
                                queue.push((j, k));
                            }
                        }
                    }

                    while !queue.is_empty() {
                        let (row_idx, col_idx) = queue.pop().unwrap();
            
                        if visited[row_idx][col_idx] {
                            continue;
                        }
            
                        visited[row_idx][col_idx] = true;
            
                        if minefield[row_idx][col_idx] == '.' {
                            let mines = count_mines(&minefield, row_idx, col_idx);
            
                            if mines == 0 {
                                for j in row_idx.saturating_sub(1)..=row_idx + 1 {
                                    for k in col_idx.saturating_sub(1)..=col_idx + 1 {
                                        if j < minefield.len() && k < minefield[0].len() {
                                            queue.push((j, k));
                                        }
                                    }
                                }
                            }
                        }
                    }
            
                }
            }
        }

        // Process rest cell
        for j in 0..n {
            for k in 0..n {
                if !visited[j][k] {
                    ret += 1;
                }
            }
        }

        writeln!(out, "Case #{i}: {ret}").unwrap();
    }
}

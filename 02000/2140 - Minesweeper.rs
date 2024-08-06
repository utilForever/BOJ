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

    let n = scan.token::<usize>();
    let mut minefield = vec![vec![' '; n]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            minefield[i][j] = c;
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
    let fill_cells =
        |minefield: &mut Vec<Vec<char>>, row_idx: usize, col_idx: usize, symbol: char| {
            for i in row_idx.saturating_sub(1)..=row_idx + 1 {
                for j in col_idx.saturating_sub(1)..=col_idx + 1 {
                    if i < minefield.len() && j < minefield[0].len() && minefield[i][j] == '#' {
                        minefield[i][j] = symbol;
                    }
                }
            }
        };

    let process_minefield = |minefield: &mut Vec<Vec<char>>, row_idx: usize, col_idx: usize| {
        let cnt = count_mines(&minefield, row_idx, col_idx);

        if cnt == minefield[row_idx][col_idx] as i64 - '0' as i64 {
            fill_cells(minefield, row_idx, col_idx, '.');
        } else {
            fill_cells(minefield, row_idx, col_idx, '*');
        }
    };

    for i in 0..n {
        process_minefield(&mut minefield, 0, i);
    }

    for i in 0..n {
        process_minefield(&mut minefield, n - 1, i);
    }

    for i in 0..n {
        process_minefield(&mut minefield, i, 0);
    }

    for i in 0..n {
        process_minefield(&mut minefield, i, n - 1);
    }

    let mut ret = 0;

    for i in 0..n {
        for j in 0..n {
            if minefield[i][j] == '*' || minefield[i][j] == '#' {
                ret += 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

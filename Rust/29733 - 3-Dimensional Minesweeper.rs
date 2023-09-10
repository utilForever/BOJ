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

    let (r, c, h) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut minefield = vec![vec![vec![' '; c]; r]; h];

    for i in 0..h {
        for j in 0..r {
            let line = scan.token::<String>();

            for (k, c) in line.chars().enumerate() {
                minefield[i][j][k] = c;
            }
        }
    }

    let ret = minefield
        .iter()
        .enumerate()
        .map(|(height_idx, height_content)| {
            height_content
                .iter()
                .enumerate()
                .map(|(row_idx, row_content)| {
                    row_content
                        .iter()
                        .enumerate()
                        .map(|(col_idx, col_content)| {
                            if *col_content == '*' {
                                '*'
                            } else {
                                let mut count = 0;

                                for i in height_idx.saturating_sub(1)..=height_idx + 1 {
                                    for j in row_idx.saturating_sub(1)..=row_idx + 1 {
                                        for k in col_idx.saturating_sub(1)..=col_idx + 1 {
                                            if i < minefield.len()
                                                && j < height_content.len()
                                                && k < row_content.len()
                                                && minefield[i][j][k] == '*'
                                            {
                                                count += 1;
                                            }
                                        }
                                    }
                                }

                                (count % 10).to_string().chars().next().unwrap()
                            }
                        })
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>();

    for i in 0..h {
        for j in 0..r {
            for k in 0..c {
                write!(out, "{}", ret[i][j][k]).unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}

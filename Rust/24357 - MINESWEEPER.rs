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

    let mut minefield = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            minefield[i][j] = scan.token::<i64>();
        }
    }

    let ret = minefield
        .iter()
        .enumerate()
        .map(|(row_idx, row_content)| {
            row_content
                .iter()
                .enumerate()
                .map(|(col_idx, col_content)| {
                    if *col_content == 9 {
                        9
                    } else {
                        let mut count = 0;

                        for i in row_idx.saturating_sub(1)..=row_idx + 1 {
                            for j in col_idx.saturating_sub(1)..=col_idx + 1 {
                                if i < minefield.len()
                                    && j < row_content.len()
                                    && minefield[i][j] == 9
                                {
                                    count += 1;
                                }
                            }
                        }

                        count
                    }
                })
                .collect()
        })
        .collect::<Vec<Vec<i64>>>();

    for i in 0..3 {
        for j in 0..3 {
            write!(out, "{} ", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}

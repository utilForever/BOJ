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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut paper = vec![vec![' '; 8 * m]; 3 * n];

    for i in 0..3 * n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            paper[i][j] = c;
        }
    }

    for i in 0..n {
        for j in 0..m {
            let a = paper[3 * i + 1][8 * j + 1] as i64 - 48;
            let b = paper[3 * i + 1][8 * j + 3] as i64 - 48;
            let c = if paper[3 * i + 1][8 * j + 6].is_numeric() {
                (paper[3 * i + 1][8 * j + 5] as i64 - 48) * 10
                    + (paper[3 * i + 1][8 * j + 6] as i64 - 48)
            } else {
                paper[3 * i + 1][8 * j + 5] as i64 - 48
            };

            if a + b == c {
                for k in 1..6 {
                    paper[3 * i][8 * j + k] = '*';
                    paper[3 * i + 2][8 * j + k] = '*';
                }

                paper[3 * i + 1][8 * j] = '*';

                if paper[3 * i + 1][8 * j + 6].is_numeric() {
                    paper[3 * i][8 * j + 6] = '*';
                    paper[3 * i + 1][8 * j + 7] = '*';
                    paper[3 * i + 2][8 * j + 6] = '*';
                } else {
                    paper[3 * i + 1][8 * j + 6] = '*';
                }
            } else {
                paper[3 * i][8 * j + 3] = '/';
                paper[3 * i + 1][8 * j + 2] = '/';
                paper[3 * i + 2][8 * j + 1] = '/';
            }
        }
    }

    for i in 0..3 * n {
        for j in 0..8 * m {
            write!(out, "{}", paper[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}

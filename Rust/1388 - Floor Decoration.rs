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
    let mut tiles = vec![vec![' '; m]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            tiles[i][j] = c;
        }
    }

    let mut ret = 0;

    for i in 0..n {
        for j in 0..m {
            if tiles[i][j] == '*' {
                continue;
            }

            ret += 1;

            if tiles[i][j] == '-' {
                let mut idx = j;

                while idx < m && tiles[i][idx] == '-' {
                    tiles[i][idx] = '*';
                    idx += 1;
                }
            } else {
                let mut idx = i;

                while idx < n && tiles[idx][j] == '|' {
                    tiles[idx][j] = '*';
                    idx += 1;
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

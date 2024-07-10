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

    let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut pattern = vec![vec![' '; c]; r];

    for i in 0..r {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            pattern[i][j] = c;
        }
    }

    let mut card = vec![vec![' '; 2 * c]; 2 * r];

    // Upper left
    for i in 0..r {
        for j in 0..c {
            card[i][j] = pattern[i][j];
        }
    }

    // Upper right
    for i in 0..r {
        for j in 0..c {
            card[i][j + c] = pattern[i][c - j - 1];
        }
    }

    // Lower left
    for i in 0..r {
        for j in 0..c {
            card[i + r][j] = pattern[r - i - 1][j];
        }
    }

    // Lower right
    for i in 0..r {
        for j in 0..c {
            card[i + r][j + c] = pattern[r - i - 1][c - j - 1];
        }
    }

    let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
    card[a - 1][b - 1] = if card[a - 1][b - 1] == '.' { '#' } else { '.' };

    for i in 0..2 * r {
        for j in 0..2 * c {
            write!(out, "{}", card[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}

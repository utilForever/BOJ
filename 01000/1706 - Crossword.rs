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
    let mut puzzle = vec![vec![' '; c]; r];
    let mut ret = Vec::new();

    for i in 0..r {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            puzzle[i][j] = c;
        }
    }

    // Horizontal
    for i in 0..r {
        let mut word = String::new();

        for j in 0..c {
            if puzzle[i][j] == '#' {
                if word.len() >= 2 {
                    ret.push(word.clone());
                }

                word.clear();
            } else {
                word.push(puzzle[i][j]);
            }
        }

        if word.len() >= 2 {
            ret.push(word);
        }
    }

    // Vertical
    for j in 0..c {
        let mut word = String::new();

        for i in 0..r {
            if puzzle[i][j] == '#' {
                if word.len() >= 2 {
                    ret.push(word.clone());
                }

                word.clear();
            } else {
                word.push(puzzle[i][j]);
            }
        }

        if word.len() >= 2 {
            ret.push(word);
        }
    }

    ret.sort();

    writeln!(out, "{}", ret[0]).unwrap();
}

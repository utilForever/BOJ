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
    let mut farm = vec![vec![' '; c]; r];

    for i in 0..r {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            farm[i][j] = c;
        }
    }

    let dy = [-1, 0, 1, 0];
    let dx = [0, 1, 0, -1];

    for i in 0..r {
        for j in 0..c {
            if farm[i][j] == 'W' {
                for k in 0..4 {
                    let y_next = i as i32 + dy[k];
                    let x_next = j as i32 + dx[k];

                    if y_next < 0 || y_next >= r as i32 || x_next < 0 || x_next >= c as i32 {
                        continue;
                    }

                    let y_next = y_next as usize;
                    let x_next = x_next as usize;

                    if farm[y_next][x_next] == 'S' {
                        writeln!(out, "0").unwrap();
                        return;
                    }
                }
            }
        }
    }

    writeln!(out, "1").unwrap();

    for i in 0..r {
        for j in 0..c {
            write!(
                out,
                "{}",
                match farm[i][j] {
                    '.' => 'D',
                    c => c,
                }
            )
            .unwrap();
        }

        writeln!(out).unwrap();
    }
}

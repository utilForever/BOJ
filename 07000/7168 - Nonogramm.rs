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
    let mut grid = vec![vec![' '; m]; n];

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    for i in 0..n {
        let mut cnt_black = 0;
        let mut has_black = false;

        for j in 0..m {
            if grid[i][j] == '#' {
                cnt_black += 1;
                has_black = true;
            } else {
                if cnt_black > 0 {
                    write!(out, "{cnt_black} ").unwrap();
                    cnt_black = 0;
                }
            }
        }

        if cnt_black > 0 {
            write!(out, "{cnt_black}").unwrap();
        } else if !has_black {
            write!(out, "0").unwrap();
        }

        writeln!(out).unwrap();
    }

    for j in 0..m {
        let mut cnt_black = 0;
        let mut has_black = false;

        for i in 0..n {
            if grid[i][j] == '#' {
                cnt_black += 1;
                has_black = true;
            } else {
                if cnt_black > 0 {
                    write!(out, "{cnt_black} ").unwrap();
                    cnt_black = 0;
                }
            }
        }

        if cnt_black > 0 {
            write!(out, "{cnt_black}").unwrap();
        } else if !has_black {
            write!(out, "0").unwrap();
        }

        writeln!(out).unwrap();
    }
}

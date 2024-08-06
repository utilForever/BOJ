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

    let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
    let mut section = vec![vec![' '; w]; h];
    let mut ret = vec![vec![0; w]; h];

    for i in 0..h {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            section[i][j] = c;
        }
    }

    for i in 0..h {
        let mut found_cloud = false;

        for j in 0..w {
            if section[i][j] == 'c' {
                found_cloud = true;
            }

            ret[i][j] = if found_cloud {
                if section[i][j] == 'c' {
                    0
                } else {
                    ret[i][j - 1] + 1
                }
            } else {
                -1
            };
        }
    }

    for i in 0..h {
        for j in 0..w {
            write!(out, "{} ", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}

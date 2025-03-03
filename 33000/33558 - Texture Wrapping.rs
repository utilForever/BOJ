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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
    let mut texture = vec![vec![' '; v]; u];

    for i in 0..u {
        let line = scan.line().trim().to_string();

        for (j, c) in line.chars().enumerate() {
            texture[i][j] = c;
        }
    }

    let method = scan.token::<String>();
    let mut ret = vec![vec![' '; m]; n];

    if method == "clamp-to-edge" {
        for i in 0..n {
            for j in 0..m {
                let idx_row = if i < u { i } else { u - 1 };
                let idx_col = if j < v { j } else { v - 1 };

                ret[i][j] = texture[idx_row][idx_col];
            }
        }
    } else if method == "repeat" {
        for i in 0..n {
            for j in 0..m {
                let idx_row = i % u;
                let idx_col = j % v;

                ret[i][j] = texture[idx_row][idx_col];
            }
        }
    } else {
        for i in 0..n {
            for j in 0..m {
                let tile_row = i / u;
                let tile_col = j / v;

                let pos_row = i % u;
                let pos_col = j % v;

                let real_row = if tile_row % 2 == 0 {
                    pos_row
                } else {
                    u - 1 - pos_row
                };
                let real_col = if tile_col % 2 == 0 {
                    pos_col
                } else {
                    v - 1 - pos_col
                };

                ret[i][j] = texture[real_row][real_col];
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            write!(out, "{}", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}

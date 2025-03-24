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

    let t = scan.token::<i64>();
    let dy = [-2, -2, -1, -1, 1, 1, 2, 2];
    let dx = [-1, 1, -2, 2, -2, 2, -1, 1];

    for _ in 0..t {
        let mut board = [[' '; 5]; 5];

        for i in 0..5 {
            let line = scan.token::<String>();

            for (j, c) in line.trim().chars().enumerate() {
                board[i][j] = c;
            }
        }

        let mut ret = true;

        'outer: for i in 0..5 {
            for j in 0..5 {
                if board[i][j] == '.' {
                    continue;
                }

                for k in 0..8 {
                    let y_next = i as i64 + dy[k];
                    let x_next = j as i64 + dx[k];

                    if y_next < 0 || y_next >= 5 || x_next < 0 || x_next >= 5 {
                        continue;
                    }

                    if board[y_next as usize][x_next as usize] == 'k' {
                        ret = false;
                        break 'outer;
                    }
                }
            }
        }

        writeln!(out, "{}", if ret { "valid" } else { "invalid" }).unwrap();
    }
}

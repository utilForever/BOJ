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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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
    let mut board = vec![vec![' '; m]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            board[i][j] = c;
        }
    }

    let dy = [-1, 0, 1, 0];
    let dx = [0, 1, 0, -1];
    let mut ret = 0;

    for i in 0..n {
        for j in 0..m {
            for k in 0..4 {
                let (y_next, x_next) = (i as i64 + dy[k], j as i64 + dx[k]);

                if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= m as i64 {
                    continue;
                }

                let (y_next, x_next) = (y_next as usize, x_next as usize);

                if board[i][j] == 'X' && board[y_next][x_next] == 'Y' {
                    ret += 1;
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

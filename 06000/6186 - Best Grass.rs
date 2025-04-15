use io::Write;
use std::{collections::VecDeque, io, str};

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

    let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut field = vec![vec![' '; c]; r];

    for i in 0..r {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            field[i][j] = c;
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; c]; r];
    let dy = [-1, 0, 1, 0];
    let dx = [0, -1, 0, 1];
    let mut ret = 0;

    for i in 0..r {
        for j in 0..c {
            if visited[i][j] || field[i][j] == '.' {
                continue;
            }

            queue.push_back((i as i64, j as i64));
            visited[i][j] = true;
            ret += 1;

            while !queue.is_empty() {
                let (y_curr, x_curr) = queue.pop_front().unwrap();

                for k in 0..4 {
                    let y_next = y_curr + dy[k];
                    let x_next = x_curr + dx[k];

                    if y_next < 0 || y_next >= r as i64 || x_next < 0 || x_next >= c as i64 {
                        continue;
                    }

                    let y_next = y_next as usize;
                    let x_next = x_next as usize;

                    if visited[y_next][x_next] || field[y_next][x_next] == '.' {
                        continue;
                    }

                    queue.push_back((y_next as i64, x_next as i64));
                    visited[y_next][x_next] = true;
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

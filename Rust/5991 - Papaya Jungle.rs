use io::Write;
use std::{io, str, vec};

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

    let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut jungle = vec![vec![0; c]; r];
    let mut visited = vec![vec![false; c]; r];

    for i in 0..r {
        for j in 0..c {
            jungle[i][j] = scan.token::<i64>();
        }
    }

    let dy = [0, 0, 1, -1];
    let dx = [1, -1, 0, 0];
    let mut pos = (0, 0);
    let mut ret = jungle[0][0];

    visited[0][0] = true;

    while pos != (r - 1, c - 1) {
        let mut next = (0, 0);
        let mut val_max = 0;

        for i in 0..4 {
            let y_next = pos.0 as i64 + dy[i];
            let x_next = pos.1 as i64 + dx[i];

            if y_next < 0 || y_next >= r as i64 || x_next < 0 || x_next >= c as i64 {
                continue;
            }

            let y_next = y_next as usize;
            let x_next = x_next as usize;

            if visited[y_next][x_next] {
                continue;
            }

            if jungle[y_next][x_next] > val_max {
                val_max = jungle[y_next][x_next];
                next = (y_next, x_next);
            }
        }

        visited[next.0][next.1] = true;
        pos = next;
        ret += val_max;
    }

    writeln!(out, "{ret}").unwrap();
}

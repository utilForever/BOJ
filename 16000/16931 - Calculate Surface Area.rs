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
    let mut figure = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            figure[i][j] = scan.token::<i64>();
        }
    }

    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];
    let mut ret = (2 * n * m) as i64;

    for i in 0..n {
        for j in 0..m {
            for k in 0..4 {
                let next_x = i as i64 + dx[k];
                let next_y = j as i64 + dy[k];

                if next_x < 0 || next_x >= n as i64 || next_y < 0 || next_y >= m as i64 {
                    ret += figure[i][j];
                    continue;
                }

                if figure[next_x as usize][next_y as usize] < figure[i][j] {
                    ret += figure[i][j] - figure[next_x as usize][next_y as usize];
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

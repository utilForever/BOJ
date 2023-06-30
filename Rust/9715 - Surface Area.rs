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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
        let mut figure = vec![vec![0; c]; r];
        let mut cnt_zero = 0;

        for i in 0..r {
            let s = scan.token::<String>();

            for (j, ch) in s.chars().enumerate() {
                figure[i][j] = ch.to_digit(10).unwrap() as i64;

                if figure[i][j] == 0 {
                    cnt_zero += 1;
                }
            }
        }

        let dx: [i64; 4] = [1, 0, -1, 0];
        let dy: [i64; 4] = [0, 1, 0, -1];
        let mut ret = (2 * (r * c - cnt_zero)) as i64;

        for i in 0..r {
            for j in 0..c {
                for k in 0..4 {
                    let next_x = i as i64 + dx[k];
                    let next_y = j as i64 + dy[k];

                    if next_x < 0 || next_x >= r as i64 || next_y < 0 || next_y >= c as i64 {
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
}

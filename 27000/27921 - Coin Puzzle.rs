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

    let (h1, w1) = (scan.token::<usize>(), scan.token::<usize>());
    let mut grid1 = vec![vec![false; w1]; h1];

    for i in 0..h1 {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            grid1[i][j] = c == 'O';
        }
    }

    let (h2, w2) = (scan.token::<usize>(), scan.token::<usize>());
    let mut grid2 = vec![vec![false; w2]; h2];

    for i in 0..h2 {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            grid2[i][j] = c == 'O';
        }
    }

    let mut ret = i64::MAX;

    for r in -9..=9 {
        for c in -9..=9 {
            let mut cnt = 0;

            for i in 0..h1 {
                for j in 0..w1 {
                    if !grid1[i][j] {
                        continue;
                    }

                    cnt += 1;

                    let r_next = i as i64 + r;
                    let c_next = j as i64 + c;

                    if r_next < 0 || r_next >= h2 as i64 || c_next < 0 || c_next >= w2 as i64 {
                        continue;
                    }

                    if !grid2[r_next as usize][c_next as usize] {
                        continue;
                    }

                    cnt -= 1;
                }
            }

            ret = ret.min(cnt);
        }
    }

    writeln!(out, "{ret}").unwrap();
}

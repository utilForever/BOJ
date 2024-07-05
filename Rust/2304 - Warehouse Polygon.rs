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

    let n = scan.token::<usize>();
    let mut pillars = vec![(0, 0); n];
    let mut w_min = usize::MAX;
    let mut w_max = 0;
    let mut h = 0;
    let mut ret = 0;

    for i in 0..n {
        pillars[i] = (scan.token::<usize>(), scan.token::<usize>());

        w_min = w_min.min(pillars[i].0);
        w_max = w_max.max(pillars[i].0);
        h = h.max(pillars[i].1);
        ret += pillars[i].1;
    }

    let w = w_max - w_min + 1;
    let mut grid = vec![vec![0; w]; h];

    for i in 0..n {
        for j in 0..pillars[i].1 {
            grid[j][pillars[i].0 - w_min] = 1;
        }
    }

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 1 {
                continue;
            }

            let exist_left = (0..j).rev().find(|&x| grid[i][x] == 1);
            let exist_right = (j + 1..w).find(|&x| grid[i][x] == 1);

            if exist_left.is_some() && exist_right.is_some() {
                ret += 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

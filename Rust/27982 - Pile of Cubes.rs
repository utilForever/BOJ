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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut cubes = vec![vec![vec![0; n + 2]; n + 2]; n + 2];

    for _ in 0..m {
        let (i, j, k) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        cubes[i][j][k] = 1;
    }

    let mut ret = 0;

    for i in 1..=n {
        for j in 1..=n {
            for k in 1..=n {
                if cubes[i][j][k] == 1
                    && cubes[i + 1][j][k] == 1
                    && cubes[i - 1][j][k] == 1
                    && cubes[i][j + 1][k] == 1
                    && cubes[i][j - 1][k] == 1
                    && cubes[i][j][k + 1] == 1
                    && cubes[i][j][k - 1] == 1
                {
                    ret += 1;
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

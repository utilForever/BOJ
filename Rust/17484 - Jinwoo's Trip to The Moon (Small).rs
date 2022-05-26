use io::Write;
use std::{cmp, io, str};

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
    let mut fuels = vec![vec![100_000_000; m + 2]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            fuels[i][j] = scan.token::<usize>();
        }
    }

    let mut min_fuels = vec![vec![vec![100_000_000; 3]; m + 2]; n + 1];
    for i in 1..=m {
        for j in 0..3 {
            min_fuels[1][i][j] = fuels[1][i];
        }
    }

    for i in 2..=n {
        for j in 1..=m {
            min_fuels[i][j][0] =
                cmp::min(min_fuels[i - 1][j][1], min_fuels[i - 1][j - 1][2]) + fuels[i][j];
            min_fuels[i][j][1] =
                cmp::min(min_fuels[i - 1][j + 1][0], min_fuels[i - 1][j - 1][2]) + fuels[i][j];
            min_fuels[i][j][2] =
                cmp::min(min_fuels[i - 1][j + 1][0], min_fuels[i - 1][j][1]) + fuels[i][j];
        }
    }

    let mut ret = 100_000_000;

    for i in 1..=m {
        for j in 0..3 {
            ret = cmp::min(ret, min_fuels[n][i][j]);
        }
    }

    writeln!(out, "{}", ret).unwrap();
}

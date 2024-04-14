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
    let mut scores = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            scores[i][j] = scan.token::<i64>();
        }
    }

    let mut scores_same = vec![0; n];

    for i in 0..m {
        if scores[0][i] == 100 {
            for j in 0..n {
                if scores[j][i] == 100 {
                    scores_same[j] += 1;
                }
            }
        }
    }

    let max = *scores_same.iter().max().unwrap();
    let ret = scores_same.iter().filter(|&&x| x == max).count();

    writeln!(out, "1 {ret}").unwrap();
}

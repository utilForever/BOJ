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
    let mut matrix = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            matrix[i][j] = scan.token::<i64>();
        }
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (command, k, val) = (
            scan.token::<String>(),
            scan.token::<usize>() - 1,
            scan.token::<i64>(),
        );

        if command == "row" {
            matrix[k] = matrix[k].iter().map(|&x| x + val).collect();
        } else {
            for i in 0..n {
                matrix[i][k] += val;
            }
        }
    }

    let sum = matrix
        .iter()
        .map(|row| row.iter().sum::<i64>())
        .sum::<i64>();
    let min = matrix
        .iter()
        .map(|row| row.iter().min().unwrap())
        .min()
        .unwrap();
    let max = matrix
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap();

    writeln!(out, "{sum} {min} {max}").unwrap();
}

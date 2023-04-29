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
    let mut matrix = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = scan.token::<i64>();
        }
    }

    // Check rule 1
    for i in 0..n {
        if matrix[i][i] != 0 {
            writeln!(out, "1").unwrap();
            return;
        }
    }

    // Check rule 2
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if matrix[i][j] <= 0 {
                writeln!(out, "2").unwrap();
                return;
            }
        }
    }

    // Check rule 3
    for i in 0..n {
        for j in 0..n {
            if matrix[i][j] != matrix[j][i] {
                writeln!(out, "3").unwrap();
                return;
            }
        }
    }

    // Check rule 4
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if matrix[i][j] + matrix[j][k] < matrix[i][k] {
                    writeln!(out, "4").unwrap();
                    return;
                }
            }
        }
    }

    writeln!(out, "0").unwrap();
}

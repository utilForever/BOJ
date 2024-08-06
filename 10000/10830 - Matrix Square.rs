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

fn multiply(a: Vec<Vec<i64>>, b: &mut Vec<Vec<i64>>, n: usize) {
    let mut temp = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                temp[i][j] += a[i][k] * b[k][j];
                temp[i][j] %= 1000;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            b[i][j] = temp[i][j];
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, mut b) = (scan.token::<usize>(), scan.token::<i64>());
    let mut matrix = vec![vec![0; n]; n];
    let mut ret = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = scan.token::<i64>();
            ret[i][j] = matrix[i][j] % 1000;
        }
    }

    b -= 1;

    while b > 0 {
        if b % 2 == 1 {
            multiply(matrix.clone(), &mut ret, n);
        }

        multiply(matrix.clone(), &mut matrix, n);

        b /= 2;
    }

    for i in 0..n {
        for j in 0..n {
            write!(out, "{} ", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}

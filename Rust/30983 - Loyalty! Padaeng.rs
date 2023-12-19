use io::Write;
use std::{io, str, vec};

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

static MOD: i64 = 1_000_000_009;

#[derive(Clone)]
struct Matrix {
    size: usize,
    elems: Vec<Vec<i64>>,
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut ret = Matrix::new(self.size);

        for i in 0..self.size {
            for j in 0..self.size {
                for k in 0..self.size {
                    ret.elems[i][k] = (ret.elems[i][k] + self.elems[i][j] * rhs.elems[j][k]) % MOD;
                }
            }
        }

        ret
    }
}

impl Matrix {
    fn new(n: usize) -> Matrix {
        let elems = vec![vec![0; n]; n];
        Matrix { size: n, elems }
    }
}

fn calculate_matrix(mut matrix: Matrix, mut end_time: usize) -> Matrix {
    let mut ret = Matrix::new(matrix.size);

    for i in 0..matrix.size {
        ret.elems[i][i] = 1;
    }

    while end_time > 0 {
        if end_time % 2 == 1 {
            ret = ret * matrix.clone();
        }

        end_time /= 2;
        matrix = matrix.clone() * matrix.clone();
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, t) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut matrix = Matrix::new(n * 2);

    for i in 0..n {
        matrix.elems[i + n][i] = 1;
    }

    for _ in 0..m {
        let command = scan.token::<i64>();

        if command == 1 {
            let (a, b) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
            matrix.elems[a][b] += 1;
            matrix.elems[b][a] += 1;
        } else {
            let (a, b, c) = (
                scan.token::<usize>() - 1,
                scan.token::<usize>() - 1,
                scan.token::<usize>() - 1,
            );
            matrix.elems[a][b + n] += 1;
            matrix.elems[b][a + n] += 1;
            matrix.elems[a][c + n] += 1;
            matrix.elems[c][a + n] += 1;
            matrix.elems[b][c + n] += 1;
            matrix.elems[c][b + n] += 1;
        }
    }

    let ret = calculate_matrix(matrix, t);

    for i in 0..n {
        writeln!(out, "{}", ret.elems[i][0]).unwrap();
    }
}

use io::Write;
use std::{io, ops, str};

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

#[derive(Clone)]
struct Matrix {
    pub len: usize,
    pub elements: Vec<Vec<i64>>,
}

impl Matrix {
    pub fn new(n: usize) -> Self {
        Self {
            len: n,
            elements: vec![vec![0; n]; n],
        }
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut ret = Matrix::new(rhs.len);

        for i in 0..rhs.len {
            for j in 0..rhs.len {
                for k in 0..rhs.len {
                    ret.elements[i][k] = (ret.elements[i][k]
                        + self.elements[i][j] * rhs.elements[j][k])
                        % 1_000_000_007;
                }
            }
        }

        ret
    }
}

fn calculate_matrix(mut matrix: Matrix, mut end_time: usize) -> Matrix {
    let mut ret = Matrix::new(matrix.len);

    for i in 0..matrix.len {
        ret.elements[i][i] = 1;
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

    let (n, m) = (scan.token(), scan.token());

    let mut matrix = Matrix::new(n);

    for _ in 0..m {
        let (a, b): (usize, usize) = (scan.token(), scan.token());
        matrix.elements[a - 1][b - 1] = 1;
        matrix.elements[b - 1][a - 1] = 1;
    }

    let d = scan.token::<usize>();

    let ret = calculate_matrix(matrix, d);
    writeln!(out, "{}", ret.elements[0][0]).unwrap();
}

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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const MOD: i64 = 1000;

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

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut matrix = Matrix {
            size: 2,
            elems: vec![vec![6, -4], vec![1, 0]],
        };
        let temp = Matrix {
            size: 2,
            elems: vec![vec![6, 0], vec![2, 0]],
        };

        matrix = calculate_matrix(matrix, n - 1);
        matrix = matrix * temp;

        let mut ret = matrix.elems[0][0] - 1;

        if ret < 0 {
            ret += MOD;
        }

        writeln!(out, "Case #{i}: {:03}", ret).unwrap();
    }
}

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

const MOD: i64 = 1_000_000_007;

type Matrix = Vec<Vec<i64>>;

fn identity(n: usize) -> Matrix {
    let mut ret = vec![vec![0; n]; n];

    for i in 0..n {
        ret[i][i] = 1;
    }

    ret
}

fn matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let mut ret = vec![vec![0; n]; n];

    for i in 0..n {
        for k in 0..n {
            if a[i][k] == 0 {
                continue;
            }

            for j in 0..n {
                ret[i][j] = (ret[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }

    ret
}

fn matrix_pow(mut mat: Matrix, mut n: usize) -> Matrix {
    let mut ret = identity(mat.len());

    while n > 0 {
        if n % 2 == 1 {
            ret = matrix_multiply(&ret, &mat);
        }

        mat = matrix_multiply(&mat, &mat);
        n /= 2;
    }

    ret
}

fn matrix_vector_multiply(mat: &Matrix, vec: &Vec<i64>) -> Vec<i64> {
    let n = mat.len();
    let mut ret = vec![0; n];

    for i in 0..n {
        let mut acc = 0;

        for j in 0..n {
            acc = (acc + mat[i][j] * vec[j]) % MOD;
        }

        ret[i] = acc;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<i64>(), scan.token::<usize>());

    if n < m as i64 {
        writeln!(out, "1").unwrap();
        return;
    }

    if n == m as i64 {
        writeln!(out, "2").unwrap();
        return;
    }

    let vec = vec![1; m];
    let mut transpose = vec![vec![0; m]; m];

    transpose[0][0] = 1;
    transpose[0][m - 1] = 1;

    for i in 1..m {
        transpose[i][i - 1] = 1;
    }

    let exp = n - (m as i64 - 1);
    let pow = matrix_pow(transpose, exp as usize);
    let ret = matrix_vector_multiply(&pow, &vec);

    writeln!(out, "{}", ret[0]).unwrap();
}

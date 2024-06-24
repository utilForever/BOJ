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

static MOD: i64 = 1000;

fn matrix_multiply(first: &Vec<Vec<i64>>, second: &Vec<Vec<i64>>, result: &mut Vec<Vec<i64>>) {
    let n = first.len();
    let mut temp = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                temp[i][j] += first[i][k] * second[k][j];
                temp[i][j] %= MOD;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            result[i][j] = temp[i][j];
        }
    }
}

fn matrix_pow(mut matrix: Vec<Vec<i64>>, mut exp: usize) -> Vec<Vec<i64>> {
    let n = matrix.len();
    let mut ret = vec![vec![0; n]; n];
    let mut temp = vec![vec![0; n]; n];

    for i in 0..n {
        ret[i][i] = 1;
    }

    if exp == 0 {
        return ret;
    }

    while exp > 0 {
        if exp % 2 == 1 {
            matrix_multiply(&ret, &matrix, &mut temp);
            ret = temp.clone();
        }

        matrix_multiply(&matrix, &matrix, &mut temp);
        matrix = temp.clone();

        exp /= 2;
    }

    ret
}

fn matrix_sum(matrix: &Vec<Vec<i64>>, exp: usize) -> Vec<Vec<i64>> {
    if exp == 1 {
        return matrix.clone();
    }

    let n = matrix.len();
    let mut ret = vec![vec![0; n]; n];

    let a = matrix_sum(matrix, exp / 2);
    let mut b = matrix_pow(matrix.clone(), exp / 2);

    for i in 0..n {
        b[i][i] = (b[i][i] + 1) % MOD;
    }

    matrix_multiply(&a, &b, &mut ret);

    if exp % 2 == 1 {
        let c = matrix_pow(matrix.clone(), exp);

        for i in 0..n {
            for j in 0..n {
                ret[i][j] = (ret[i][j] + c[i][j]) % MOD;
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, b) = (scan.token::<usize>(), scan.token::<usize>());
    let mut matrix = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = scan.token::<i64>() % MOD;
        }
    }

    let result = matrix_sum(&matrix, b);

    for i in 0..n {
        for j in 0..n {
            write!(out, "{} ", result[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}

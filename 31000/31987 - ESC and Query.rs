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

static MOD: i64 = 1_000_000_007;

fn matrix_multiply(first: &[[i64; 4]; 4], second: &[[i64; 4]; 4], result: &mut [[i64; 4]; 4]) {
    let mut temp = [[0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            temp[i][j] = (first[i][0] * second[0][j]
                        + first[i][1] * second[1][j]
                        + first[i][2] * second[2][j]
                        + first[i][3] * second[3][j]) % MOD;
        }
    }

    for i in 0..4 {
        for j in 0..4 {
            result[i][j] = temp[i][j];
        }
    }
}

fn matrix_pow(mut matrix: [[i64; 4]; 4], mut n: usize) -> [[i64; 4]; 4] {
    let mut ret = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]];
    let mut temp = [[0; 4]; 4];

    if n == 0 {
        return ret;
    }

    while n > 0 {
        if n % 2 == 1 {
            matrix_multiply(&ret, &matrix, &mut temp);
            ret.copy_from_slice(&temp);
        }

        matrix_multiply(&matrix, &matrix, &mut temp);
        matrix.copy_from_slice(&temp);
        
        n /= 2;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let q = scan.token::<i64>();
    let coefficient = [[1, 0, -1, 0], [0, 1, 1, 0], [2, -2, 1, 0], [0, 0, 0, 0]];

    for _ in 0..q {
        let (i, j, k) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        let mut base = matrix_pow(coefficient, k);
        base[3][0] = (base[0][0] + base[1][0] + base[2][0]) % MOD;
        base[3][1] = (base[0][1] + base[1][1] + base[2][1]) % MOD;
        base[3][2] = (base[0][2] + base[1][2] + base[2][2]) % MOD;
        base[3][3] = 1;

        let mat_left = matrix_pow(base, i - 1);
        let mat_right = matrix_pow(base, j);
        let mut ret = (mat_right[3][2] - mat_left[3][2]) % MOD;

        if ret < 0 {
            ret += MOD;
        }

        writeln!(out, "{ret}").unwrap();
    }
}

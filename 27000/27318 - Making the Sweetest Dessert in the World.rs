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

#[derive(Clone)]
struct Matrix {
    size: usize,
    elems: Vec<Vec<i64>>,
}

impl Matrix {
    fn new(n: usize) -> Self {
        Self {
            size: n,
            elems: vec![vec![0; n]; n],
        }
    }

    fn pow(&self, mut pow: usize) -> Matrix {
        let mut base = self.clone();
        let mut ret = Matrix::new(base.size);

        for i in 0..base.size {
            ret.elems[i][i] = 1;
        }

        while pow > 0 {
            if pow % 2 == 1 {
                ret = ret * base.clone();
            }

            pow /= 2;
            base = base.clone() * base.clone();
        }

        ret
    }
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

fn pow(x: i64, mut y: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % MOD;

    while y != 0 {
        if y & 1 != 0 {
            ret = ret * piv % MOD
        }

        piv = piv * piv % MOD;
        y >>= 1;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<i64>(), scan.token::<i64>());
    let volume = pow((12 * n - 16) % MOD, m);

    let mut mat = Matrix::new(2);
    mat.elems[0][0] = (12 * n - 16) % MOD;
    mat.elems[0][1] = 0;
    mat.elems[1][0] = (4 * (n - 1)) % MOD;
    mat.elems[1][1] = (4 * (n - 1)) % MOD;

    let val = mat.pow(m as usize).elems[1][0] % MOD;
    let diff = (volume + MOD - val) % MOD;
    let area = (6 * diff) % MOD;

    writeln!(out, "{volume} {area}").unwrap();
}

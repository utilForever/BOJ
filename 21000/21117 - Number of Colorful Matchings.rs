#![allow(dead_code)]

use io::Write;
use std::{
    io,
    ops::{Index, IndexMut},
    str,
};

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

fn pow(x: i64, mut y: i64, modular: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % modular;

    while y != 0 {
        if y & 1 != 0 {
            ret = ret * piv % modular
        }

        piv = piv * piv % modular;
        y >>= 1;
    }

    ret
}

#[derive(Clone)]
struct Polynomial {
    degree: usize,
    coefficients: Vec<i64>,
}

impl Default for Polynomial {
    fn default() -> Self {
        Self {
            degree: 0,
            coefficients: vec![0],
        }
    }
}

impl Index<usize> for Polynomial {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index <= self.degree);
        &self.coefficients[index]
    }
}

impl IndexMut<usize> for Polynomial {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index <= self.degree);
        &mut self.coefficients[index]
    }
}

impl Polynomial {
    fn new(degree: usize) -> Self {
        Self {
            degree,
            coefficients: vec![0; degree + 1],
        }
    }

    fn new_with_coefficients(coefficients: Vec<i64>) -> Self {
        Self {
            degree: coefficients.len() - 1,
            coefficients,
        }
    }

    fn add(&mut self, p: Polynomial, modulus: i64) -> Polynomial {
        let n = self.degree.max(p.degree);
        let mut poly = Polynomial::new(n);

        for i in 0..=n {
            if i <= self.degree {
                poly[i] += self.coefficients[i];
            }

            if i <= p.degree {
                poly[i] += p.coefficients[i];
            }

            if modulus != 0 {
                poly[i] %= modulus;
            }
        }

        poly
    }

    fn multiply(&mut self, p: Polynomial, modulus: i64) -> Polynomial {
        let n = self.degree + p.degree;
        let mut poly = Polynomial::new(n + 1);

        for i in 0..=self.degree {
            for j in 0..=p.degree {
                poly[i + j] += self.coefficients[i] * p.coefficients[j];

                if modulus != 0 {
                    poly[i + j] %= modulus;
                }
            }
        }

        poly
    }
}

struct Matrix {
    n: usize,
    m: usize,
    column_idx: Vec<usize>,
    elements: Vec<Vec<i64>>,
}

impl Index<(usize, usize)> for Matrix {
    type Output = i64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        assert!(i < self.n && j < self.m);
        &self.elements[i][self.column_idx[j]]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        assert!(i < self.n && j < self.m);
        &mut self.elements[i][self.column_idx[j]]
    }
}

impl Matrix {
    fn new(n: usize, m: usize) -> Self {
        Self {
            n,
            m,
            column_idx: (0..m).collect(),
            elements: vec![vec![0; m]; n],
        }
    }

    fn new_with_elements(elements: Vec<Vec<i64>>) -> Self {
        Self {
            n: elements.len(),
            m: elements[0].len(),
            column_idx: (0..elements[0].len()).collect(),
            elements,
        }
    }

    fn add(&self, other: &Matrix, modular: i64) -> Matrix {
        assert!(self.n == other.n && self.m == other.m);

        let mut ret = Matrix::new(self.n, self.m);

        for i in 0..self.n {
            for j in 0..self.m {
                ret[(i, j)] = self.elements[i][self.column_idx[j]] + other[(i, j)];

                if modular != 0 {
                    ret[(i, j)] %= modular;
                }
            }
        }

        ret
    }

    fn multiply(&self, other: &Matrix, modular: i64) -> Matrix {
        assert!(self.m == other.n);

        let mut ret = Matrix::new(self.n, other.m);

        for i in 0..ret.n {
            for k in 0..self.m {
                for j in 0..ret.m {
                    ret[(i, j)] += self.elements[i][self.column_idx[k]] * other[(k, j)];

                    if modular != 0 {
                        ret[(i, j)] %= modular;
                    }
                }
            }
        }

        ret
    }

    fn row_swap(&mut self, i: usize, j: usize) {
        self.elements.swap(i, j);
    }

    fn row_add(&mut self, i1: usize, i2: usize, val: i64, modular: i64) {
        for j in 0..self.m {
            self.elements[i1][self.column_idx[j]] += self.elements[i2][self.column_idx[j]] * val;

            if modular != 0 {
                self.elements[i1][self.column_idx[j]] %= modular;
            }
        }
    }

    fn row_mul(&mut self, i: usize, val: i64, modular: i64) {
        for j in 0..self.m {
            self.elements[i][self.column_idx[j]] *= val;

            if modular != 0 {
                self.elements[i][self.column_idx[j]] %= modular;
            }
        }
    }

    fn col_swap(&mut self, j1: usize, j2: usize) {
        self.column_idx.swap(j1, j2);
    }

    fn column_add(&mut self, j1: usize, j2: usize, value: i64, modular: i64) {
        for i in 0..self.n {
            self.elements[i][self.column_idx[j1]] += self.elements[i][self.column_idx[j2]] * value;

            if modular != 0 {
                self.elements[i][self.column_idx[j1]] %= modular;
            }
        }
    }

    fn column_mul(&mut self, j: usize, value: i64, modular: i64) {
        for i in 0..self.n {
            self.elements[i][self.column_idx[j]] *= value;

            if modular != 0 {
                self.elements[i][self.column_idx[j]] %= modular;
            }
        }
    }

    fn char_poly(&mut self, modular: i64) -> Polynomial {
        let mut a = Matrix::new(self.n, self.n);

        for i in 0..self.n {
            for j in 0..self.n {
                a[(i, j)] = self.elements[i][self.column_idx[j]];
            }
        }

        for j in 0..self.n.saturating_sub(2) {
            for i in j + 1..self.n {
                if a[(i, j)] != 0 {
                    a.row_swap(i, j + 1);
                    a.col_swap(i, j + 1);
                    break;
                }
            }

            if a[(j + 1, j)] != 0 {
                for i in j + 2..self.n {
                    let val =
                        (modular - a[(i, j)]) * pow(a[(j + 1, j)], modular - 2, modular) % modular;
                    a.row_add(i, j + 1, val, modular);
                    a.column_add(j + 1, i, modular - val, modular);
                }
            }
        }

        let mut dp = vec![Polynomial::default(); self.n + 1];
        dp[0] = Polynomial::new_with_coefficients(vec![1]);

        for k in 1..=self.n {
            for i in 0..k {
                let mut tmp = 1;

                for j in i + 2..=k {
                    tmp = (tmp * (modular - a[(j - 1, j - 2)])) % modular;
                }

                let mut c = dp[i].multiply(Polynomial::new_with_coefficients(vec![tmp]), modular);

                if (k - 1 - i) % 2 != 0 {
                    c = c.multiply(
                        Polynomial::new_with_coefficients(vec![modular - 1]),
                        modular,
                    );
                }

                c = if i == k - 1 {
                    c.multiply(
                        Polynomial::new_with_coefficients(vec![modular - a[(i, k - 1)], 1]),
                        modular,
                    )
                } else {
                    c.multiply(
                        Polynomial::new_with_coefficients(vec![modular - a[(i, k - 1)]]),
                        modular,
                    )
                };

                dp[k] = dp[k].add(c, modular);
            }
        }

        dp[self.n].clone()
    }
}

fn linear_det(mut a: Matrix, mut b: Matrix, modular: i64) -> Polynomial {
    let n = a.n;

    assert!(n == a.m && n == b.n);

    let mut idx_a = 0;
    let mut idx_b = 0;
    let mut det = 1;

    while idx_a + idx_b < n {
        for i in 0..idx_a {
            b.column_add(idx_a, i, modular - a[(i, idx_a)], modular);
            a.column_add(idx_a, i, modular - a[(i, idx_a)], modular);
        }

        for i in idx_a..n - idx_b {
            if a[(i, idx_a)] != 0 {
                a.row_swap(idx_a, i);
                b.row_swap(idx_a, i);

                if i != idx_a {
                    det = modular - det;
                }

                break;
            }
        }

        if a[(idx_a, idx_a)] != 0 {
            det = (det * a[(idx_a, idx_a)]) % modular;

            b.row_mul(idx_a, pow(a[(idx_a, idx_a)], modular - 2, modular), modular);
            a.row_mul(idx_a, pow(a[(idx_a, idx_a)], modular - 2, modular), modular);

            for i in idx_a + 1..n {
                b.row_add(i, idx_a, modular - a[(i, idx_a)], modular);
                a.row_add(i, idx_a, modular - a[(i, idx_a)], modular);
            }

            idx_a += 1;
        } else {
            let r = n - idx_b - 1;

            a.col_swap(idx_a, r);
            b.col_swap(idx_a, r);

            if idx_a != r {
                det = modular - det;
            }

            let mut pos = None;

            for i in (0..=r).rev() {
                if b[(i, r)] != 0 {
                    pos = Some(i);
                    break;
                }
            }

            if pos.is_none() {
                return Polynomial::default();
            }

            let pos = pos.unwrap();

            if pos < idx_a {
                a.row_swap(pos, idx_a - 1);
                b.row_swap(pos, idx_a - 1);
                a.col_swap(pos, idx_a - 1);
                b.col_swap(pos, idx_a - 1);
                a.row_swap(idx_a - 1, r);
                b.row_swap(idx_a - 1, r);

                if idx_a - 1 != r {
                    det = modular - det;
                }

                idx_a -= 1;
            } else {
                a.row_swap(pos, r);
                b.row_swap(pos, r);

                if pos != r {
                    det = modular - det;
                }
            }

            det = (det * b[(r, r)]) % modular;

            a.row_mul(r, pow(b[(r, r)], modular - 2, modular), modular);
            b.row_mul(r, pow(b[(r, r)], modular - 2, modular), modular);

            for i in 0..r {
                a.row_add(i, r, modular - b[(i, r)], modular);
                b.row_add(i, r, modular - b[(i, r)], modular);
            }

            idx_b += 1;
        }
    }

    let mut c = Matrix::new(idx_a, idx_a);

    for i in 0..idx_a {
        for j in 0..idx_a {
            c[(i, j)] = (modular - b[(i, j)]) % modular;
        }
    }

    let mut ret = c.char_poly(modular);
    ret = ret.multiply(Polynomial::new_with_coefficients(vec![det]), modular);

    ret
}

// Reference: https://tistory.joonhyung.xyz/18
// Reference: https://tistory.joonhyung.xyz/19
// Reference: https://tistory.joonhyung.xyz/21
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut red = vec![vec![0; n]; n];
    let mut blue = vec![vec![0; n]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            red[i][j] = (c as u8 - b'0') as i64;
        }
    }

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            blue[i][j] = (c as u8 - b'0') as i64;
        }
    }

    const MOD: i64 = 2;

    let ret = linear_det(
        Matrix::new_with_elements(red),
        Matrix::new_with_elements(blue),
        MOD,
    );

    for k in 0..=n {
        writeln!(
            out,
            "{}",
            if k < ret.coefficients.len() {
                ret[k]
            } else {
                0
            }
        )
        .unwrap();
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ModInt {
    value: i64,
    modulo: i64,
}

impl ModInt {
    #[inline(always)]
    fn new(value: i64, modulo: i64) -> Self {
        let mut val = value % modulo;

        if val < 0 {
            val += modulo;
        }

        ModInt { value: val, modulo }
    }

    #[inline(always)]
    fn pow(self, mut exp: i64) -> Self {
        let m = self.modulo;
        let mut base = self.value;
        let mut ret = 1;

        while exp > 0 {
            if exp & 1 == 1 {
                ret = (ret * base) % m;
            }

            base = (base * base) % m;
            exp >>= 1;
        }

        ModInt::new(ret, m)
    }

    #[inline(always)]
    fn inv(self) -> Self {
        self.pow(self.modulo - 2)
    }
}

#[inline(always)]
fn add_mod(a: i64, b: i64, m: i64) -> i64 {
    let mut ret = a + b;

    if ret >= m {
        ret -= m;
    }

    ret
}

#[inline(always)]
fn sub_mod(a: i64, b: i64, m: i64) -> i64 {
    let mut ret = a - b;

    if ret < 0 {
        ret += m;
    }

    ret
}

#[inline(always)]
fn mul_mod(a: i64, b: i64, m: i64) -> i64 {
    ((a % m) * (b % m)) % m
}

#[inline(always)]
fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut ret = 1;

    while exp > 0 {
        if exp & 1 == 1 {
            ret = (ret * base) % m;
        }

        base = (base * base) % m;
        exp >>= 1;
    }

    ret
}

impl std::ops::Add for ModInt {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        ModInt {
            value: add_mod(self.value, other.value, self.modulo),
            modulo: self.modulo,
        }
    }
}

impl std::ops::Sub for ModInt {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        ModInt {
            value: sub_mod(self.value, other.value, self.modulo),
            modulo: self.modulo,
        }
    }
}

impl std::ops::Mul for ModInt {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        ModInt {
            value: (self.value * other.value) % self.modulo,
            modulo: self.modulo,
        }
    }
}

#[derive(Clone)]
struct Polynomial {
    coeffs: Vec<ModInt>,
}

impl Polynomial {
    #[inline(always)]
    fn new_with_coeffs(mut coeffs: Vec<ModInt>) -> Self {
        while coeffs.len() > 1 && coeffs.last().unwrap().value == 0 {
            coeffs.pop();
        }

        Self { coeffs }
    }

    #[inline(always)]
    fn constant(c: i64, modulo: i64) -> Self {
        Self::new_with_coeffs(vec![ModInt::new(c.rem_euclid(modulo), modulo)])
    }

    #[inline(always)]
    fn monomial_x(modulo: i64) -> Self {
        Self::new_with_coeffs(vec![ModInt::new(0, modulo), ModInt::new(1, modulo)])
    }

    #[inline(always)]
    fn zero(modulo: i64) -> Self {
        Self::new_with_coeffs(vec![ModInt::new(0, modulo)])
    }

    #[inline(always)]
    fn one(modulo: i64) -> Self {
        Self::new_with_coeffs(vec![ModInt::new(1, modulo)])
    }

    #[inline(always)]
    fn deg(&self) -> i64 {
        if self.coeffs.len() == 1 && self.coeffs[0].value == 0 {
            -1
        } else {
            self.coeffs.len() as i64 - 1
        }
    }

    #[inline(always)]
    fn modulo(&self) -> i64 {
        self.coeffs[0].modulo
    }

    #[inline(always)]
    fn coeff_leading(&self) -> ModInt {
        *self.coeffs.last().unwrap()
    }
}

impl<'a, 'b> std::ops::Add<&'b Polynomial> for &'a Polynomial {
    type Output = Polynomial;

    #[inline(always)]
    fn add(self, other: &'b Polynomial) -> Polynomial {
        let n = self.coeffs.len().max(other.coeffs.len());
        let modulo = self.modulo();
        let mut ret = vec![ModInt::new(0, modulo); n];

        for i in 0..n {
            let a = if i < self.coeffs.len() {
                self.coeffs[i]
            } else {
                ModInt::new(0, modulo)
            };
            let b = if i < other.coeffs.len() {
                other.coeffs[i]
            } else {
                ModInt::new(0, modulo)
            };

            ret[i] = a + b;
        }

        Polynomial::new_with_coeffs(ret)
    }
}

impl std::ops::Add for Polynomial {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        (&self) + (&other)
    }
}

impl<'a, 'b> std::ops::Sub<&'b Polynomial> for &'a Polynomial {
    type Output = Polynomial;

    #[inline(always)]
    fn sub(self, other: &'b Polynomial) -> Polynomial {
        let n = self.coeffs.len().max(other.coeffs.len());
        let modulo = self.modulo();
        let mut ret = vec![ModInt::new(0, modulo); n];

        for i in 0..n {
            let a = if i < self.coeffs.len() {
                self.coeffs[i]
            } else {
                ModInt::new(0, modulo)
            };
            let b = if i < other.coeffs.len() {
                other.coeffs[i]
            } else {
                ModInt::new(0, modulo)
            };

            ret[i] = a - b;
        }

        Polynomial::new_with_coeffs(ret)
    }
}

impl std::ops::Sub for Polynomial {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        (&self) - (&other)
    }
}

impl<'a, 'b> std::ops::Mul<&'b Polynomial> for &'a Polynomial {
    type Output = Polynomial;

    #[inline(always)]
    fn mul(self, other: &'b Polynomial) -> Polynomial {
        let modulo = self.modulo();

        if self.deg() < 0 || other.deg() < 0 {
            return Polynomial::zero(modulo);
        }

        let mut ret = vec![ModInt::new(0, modulo); self.coeffs.len() + other.coeffs.len() - 1];

        for i in 0..self.coeffs.len() {
            let ai = self.coeffs[i];

            if ai.value == 0 {
                continue;
            }

            for j in 0..other.coeffs.len() {
                let bj = other.coeffs[j];

                if bj.value == 0 {
                    continue;
                }

                let idx = i + j;
                let val = ret[idx] + (ai * bj);

                ret[idx] = val;
            }
        }

        Polynomial::new_with_coeffs(ret)
    }
}

impl std::ops::Mul for Polynomial {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        (&self) * (&other)
    }
}

impl std::ops::Rem<&Polynomial> for Polynomial {
    type Output = Polynomial;

    #[inline(always)]
    fn rem(self, divisor: &Polynomial) -> Polynomial {
        let modulo = self.modulo();
        let deg = self.deg();
        let deg_divisor = divisor.deg();

        if deg < deg_divisor || deg_divisor < 0 {
            return Polynomial::new_with_coeffs(self.coeffs);
        }

        let mut ret = self.coeffs;
        let degree = deg_divisor as usize;
        let coeff_leading_inv = divisor.coeff_leading().inv();

        while (ret.len() as i64 - 1) >= deg_divisor {
            let shift = ret.len() - 1 - degree;
            let last = ret.pop().unwrap();
            let coeff = last * coeff_leading_inv;

            for i in 0..degree {
                let idx = shift + i;
                let sub = (divisor.coeffs[i] * coeff).value;
                let value_new = sub_mod(ret[idx].value, sub, modulo);

                ret[idx].value = value_new;
            }

            while ret.len() > 1 && ret.last().unwrap().value == 0 {
                ret.pop();
            }
        }

        Polynomial::new_with_coeffs(ret)
    }
}

impl std::ops::Rem for Polynomial {
    type Output = Self;

    #[inline(always)]
    fn rem(self, divisor: Self) -> Self {
        self % &divisor
    }
}

fn resultant(mut f: Polynomial, mut g: Polynomial) -> ModInt {
    let modulo = f.coeffs[0].modulo;
    let mut ret = ModInt::new(1, modulo);

    loop {
        if g.deg() < 0 {
            return ModInt::new(0, modulo);
        }

        if g.deg() == 0 {
            let f_deg = f.deg().max(0) as i64;
            let g_coeff = g.coeffs[0];
            return ret * g_coeff.pow(f_deg);
        }

        let deg_f = f.deg() as i64;
        let r = f % &g;

        if r.deg() < 0 {
            return ModInt::new(0, modulo);
        }

        let deg_g = g.deg() as i64;
        let deg_r = r.deg() as i64;

        if (deg_f * deg_g) % 2 == 1 {
            ret = ret * ModInt::new(modulo - 1, modulo);
        }

        let g_coeff_leading_pow = g.coeff_leading().pow(deg_f - deg_r);
        ret = ret * g_coeff_leading_pow;

        f = g;
        g = r;
    }
}

fn multiply_poly2x2(
    a: &[[Polynomial; 2]; 2],
    b: &[[Polynomial; 2]; 2],
    poly: &Polynomial,
) -> [[Polynomial; 2]; 2] {
    let c00 = ((&a[0][0] * &b[0][0]) + (&a[0][1] * &b[1][0])) % poly;
    let c01 = ((&a[0][0] * &b[0][1]) + (&a[0][1] * &b[1][1])) % poly;
    let c10 = ((&a[1][0] * &b[0][0]) + (&a[1][1] * &b[1][0])) % poly;
    let c11 = ((&a[1][0] * &b[0][1]) + (&a[1][1] * &b[1][1])) % poly;

    [[c00, c01], [c10, c11]]
}

fn path_factor_poly(poly: &Polynomial, k: i64) -> Polynomial {
    let m = poly.coeffs[0].modulo;

    if k == 1 {
        return Polynomial::one(m);
    }

    let one = Polynomial::one(m);
    let zero = Polynomial::zero(m);
    let x = Polynomial::monomial_x(m);
    let two = Polynomial::constant(2, m);
    let x_plus_two = x + two;
    let minus_one = Polynomial::constant(m - 1, m);

    let mut mat = [[x_plus_two, minus_one], [one.clone(), zero.clone()]];
    let mut ret = [[one.clone(), zero.clone()], [zero, one]];
    let mut exp = k - 1;

    while exp > 0 {
        if exp & 1 == 1 {
            ret = multiply_poly2x2(&ret, &mat, poly);
        }

        mat = multiply_poly2x2(&mat, &mat, poly);
        exp >>= 1;
    }

    ret[0][0].clone()
}

#[derive(Debug, Clone)]
struct Matrix {
    n: usize,
    m: usize,
    column_idx: Vec<usize>,
    elements: Vec<Vec<i64>>,
}

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = i64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        assert!(i < self.n && j < self.m);
        &self.elements[i][self.column_idx[j]]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix {
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

    fn normalize(&mut self) {
        for i in 0..self.n {
            for j in 0..self.m {
                self[(i, j)] = self[(i, j)].rem_euclid(MOD);
            }
        }
    }

    fn row_swap(&mut self, i: usize, j: usize) {
        self.elements.swap(i, j);
    }

    fn row_add_mul(&mut self, i1: usize, i2: usize, val: i64) {
        for j in 0..self.m {
            let curr = self[(i1, j)];
            let add = mul_mod(self[(i2, j)], val, MOD);
            self[(i1, j)] = add_mod(curr, add, MOD);
        }
    }

    fn col_swap(&mut self, j1: usize, j2: usize) {
        self.column_idx.swap(j1, j2);
    }

    fn col_add_mul(&mut self, j1: usize, j2: usize, val: i64) {
        for i in 0..self.n {
            let curr = self[(i, j1)];
            let add = mul_mod(self[(i, j2)], val, MOD);
            self[(i, j1)] = add_mod(curr, add, MOD);
        }
    }

    fn update_laplacian(&mut self, u: usize, v: usize) {
        self[(u, u)] = add_mod(self[(u, u)], 1, MOD);
        self[(v, v)] = add_mod(self[(v, v)], 1, MOD);
        self[(u, v)] = sub_mod(self[(u, v)], 1, MOD);
        self[(v, u)] = sub_mod(self[(v, u)], 1, MOD);
    }

    fn cofactor(&self, skip_row: usize, skip_col: usize) -> Matrix {
        let mut mat = Matrix::new(self.n - 1, self.m - 1);
        let mut idx_i = 0;

        for i in 0..self.n {
            if i == skip_row {
                continue;
            }

            let mut idx_j = 0;

            for j in 0..self.m {
                if j == skip_col {
                    continue;
                }

                mat[(idx_i, idx_j)] = self[(i, j)];
                idx_j += 1;
            }

            idx_i += 1;
        }

        mat
    }

    fn det(&self) -> i64 {
        assert!(self.n == self.m);
        let n = self.n;
        let mut mat = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                mat[i][j] = self[(i, j)].rem_euclid(MOD);
            }
        }

        let mut det = 1;

        for i in 0..n {
            let mut piv = i;

            while piv < n && mat[piv][i] == 0 {
                piv += 1;
            }

            if piv == n {
                return 0;
            }

            if piv != i {
                mat.swap(piv, i);
                det = sub_mod(0, det, MOD);
            }

            let pivot = mat[i][i];
            let pivot_inv = pow_mod(pivot, MOD - 2, MOD);

            det = mul_mod(det, pivot, MOD);

            for r in i + 1..n {
                if mat[r][i] == 0 {
                    continue;
                }

                let factor = mul_mod(mat[r][i], pivot_inv, MOD);

                for c in i..n {
                    mat[r][c] = sub_mod(mat[r][c], mul_mod(factor, mat[i][c], MOD), MOD);
                }
            }
        }

        det
    }

    fn reduction_to_hessenberg(&mut self) {
        assert!(self.n == self.m);
        let n = self.n;

        for j in 0..n - 2 {
            let mut piv = None;

            for i in j + 1..n {
                if self[(i, j)] != 0 {
                    piv = Some(i);
                    break;
                }
            }

            let Some(piv) = piv else { continue };

            if piv != j + 1 {
                self.row_swap(piv, j + 1);
                self.col_swap(piv, j + 1);
            }

            let pivot = self[(j + 1, j)];

            if pivot == 0 {
                continue;
            }

            let pivot_inv = pow_mod(pivot, MOD - 2, MOD);

            for i in j + 2..n {
                let element = self[(i, j)];

                if element == 0 {
                    continue;
                }

                let val = mul_mod(sub_mod(0, element, MOD), pivot_inv, MOD);

                self.row_add_mul(i, j + 1, val);
                self.col_add_mul(j + 1, i, sub_mod(0, val, MOD));
            }
        }
    }

    fn char_poly(&self) -> Vec<i64> {
        assert!(self.n == self.m);
        let n = self.n;

        let mut h = self.clone();
        h.normalize();
        h.reduction_to_hessenberg();

        let mut dp = vec![Vec::new(); n + 1];
        dp[0] = vec![1];

        for k in 1..=n {
            let mut val = vec![0; k + 1];
            let mut prod_subdiagonal = vec![1; k];

            if k >= 2 {
                for i in (0..=k - 2).rev() {
                    prod_subdiagonal[i] = mul_mod(prod_subdiagonal[i + 1], h[(i + 1, i)], MOD);
                }
            }

            for i in 0..k {
                let coeff = sub_mod(0, h[(i, k - 1)], MOD);

                if i == k - 1 {
                    for d in 0..=i {
                        let v = mul_mod(dp[i][d], prod_subdiagonal[i], MOD);
                        val[d] = add_mod(val[d], mul_mod(v, coeff, MOD), MOD);
                        val[d + 1] = add_mod(val[d + 1], v, MOD);
                    }
                } else {
                    for d in 0..=i {
                        let v = mul_mod(dp[i][d], prod_subdiagonal[i], MOD);
                        val[d] = add_mod(val[d], mul_mod(v, coeff, MOD), MOD);
                    }
                }
            }

            dp[k] = val;
        }

        dp[n].clone()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut laplacian = Matrix::new(n, n);

    for _ in 0..m {
        let (u, v) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        laplacian.update_laplacian(u, v);
    }

    if n == 1 {
        writeln!(out, "1").unwrap();
        return;
    }

    let det = laplacian.cofactor(n - 1, n - 1).det();

    if det == 0 {
        writeln!(out, "0").unwrap();
        return;
    }

    let coeffs = laplacian.char_poly();
    let poly =
        Polynomial::new_with_coeffs(coeffs.into_iter().map(|c| ModInt::new(c, MOD)).collect());
    let poly1 = Polynomial::new_with_coeffs(poly.coeffs[1..].to_vec());
    let r = path_factor_poly(&poly1, k);
    let ret = resultant(poly1, r);

    writeln!(out, "{}", mul_mod(ret.value, det, MOD)).unwrap();
}

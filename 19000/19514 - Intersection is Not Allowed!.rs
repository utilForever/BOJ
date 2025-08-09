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

#[derive(Debug, Clone, Copy)]
struct ModInt {
    value: i64,
    modulo: i64,
}

impl ModInt {
    fn new(value: i64, modulo: i64) -> Self {
        ModInt {
            value: value % modulo,
            modulo,
        }
    }

    fn pow(self, mut exp: i64) -> Self {
        let mut base = self.value;
        let mut ret = 1;

        while exp > 0 {
            if exp % 2 == 1 {
                ret = (ret * base) % self.modulo;
            }

            base = (base * base) % self.modulo;
            exp /= 2;
        }

        ModInt::new(ret, self.modulo)
    }

    fn inv(self) -> Self {
        self.pow(self.modulo - 2)
    }
}

impl std::ops::Add for ModInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ModInt {
            value: (self.value + other.value) % self.modulo,
            modulo: self.modulo,
        }
    }
}

impl std::ops::Sub for ModInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        ModInt {
            value: (self.value - other.value + self.modulo) % self.modulo,
            modulo: self.modulo,
        }
    }
}

impl std::ops::Mul for ModInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        ModInt {
            value: (self.value * other.value) % self.modulo,
            modulo: self.modulo,
        }
    }
}

fn comb(fact: &Vec<ModInt>, fact_inv: &Vec<ModInt>, a: usize, b: usize) -> ModInt {
    fact[a] * fact_inv[a - b] * fact_inv[b]
}

fn determinant(mat: &mut Vec<Vec<ModInt>>) -> ModInt {
    let n = mat.len();

    if n == 0 {
        return ModInt::new(1, MOD);
    }

    let mut sign = ModInt::new(1, MOD);

    for i in 0..n {
        let mut pivot = i;

        while pivot < n && mat[pivot][i].value == 0 {
            pivot += 1;
        }

        if pivot == n {
            return ModInt::new(0, MOD);
        }

        if pivot != i {
            mat.swap(i, pivot);
            sign = sign * ModInt::new(MOD - 1, MOD);
        }

        let pivot_inv: ModInt = mat[i][i].inv();

        for r in (i + 1)..n {
            if mat[r][i].value == 0 {
                continue;
            }

            let factor = mat[r][i] * pivot_inv;

            for c in i..n {
                mat[r][c] = mat[r][c] - factor * mat[i][c];
            }
        }
    }

    let mut det = sign;

    for i in 0..n {
        det = det * mat[i][i];
    }

    det
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut fact = vec![ModInt::new(1, MOD); 200_001];

    for i in 1..=200_000 {
        fact[i] = fact[i - 1] * ModInt::new(i as i64, MOD);
    }

    let mut fact_inv = vec![ModInt::new(1, MOD); 200_001];
    fact_inv[200_000] = fact[200_000].inv();

    for i in (0..200_000).rev() {
        fact_inv[i] = fact_inv[i + 1] * ModInt::new((i + 1) as i64, MOD);
    }

    let t = scan.token::<usize>();

    for _ in 0..t {
        let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
        let mut pos_initial = vec![ModInt::new(0, MOD); k];
        let mut pos_final = vec![ModInt::new(0, MOD); k];

        for i in 0..k {
            pos_initial[i] = ModInt::new(scan.token::<i64>(), MOD);
        }

        for i in 0..k {
            pos_final[i] = ModInt::new(scan.token::<i64>(), MOD);
        }

        let mut mat = vec![vec![ModInt::new(0, MOD); k]; k];

        for i in 0..k {
            for j in 0..k {
                let diff = pos_final[j].value - pos_initial[i].value;

                if diff >= 0 {
                    mat[i][j] = comb(&fact, &fact_inv, diff as usize + n - 1, n - 1);
                }
            }
        }

        let ret = determinant(&mut mat);
        writeln!(out, "{}", ret.value).unwrap();
    }
}

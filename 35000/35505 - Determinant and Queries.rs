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

    fn is_zero(&self) -> bool {
        self.value == 0
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

impl std::cmp::PartialEq for ModInt {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl std::cmp::PartialOrd for ModInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

fn gauss_elimination(matrix: Vec<Vec<ModInt>>) -> (ModInt, Vec<Vec<ModInt>>) {
    let n = matrix.len();
    let zero = ModInt::new(0, MOD);
    let one = ModInt::new(1, MOD);
    let minus_one = ModInt::new(MOD - 1, MOD);

    let mut augment = vec![vec![zero; 2 * n]; n];

    for i in 0..n {
        for j in 0..n {
            augment[i][j] = matrix[i][j];
        }

        augment[i][n + i] = one;
    }

    let mut det = one;

    for col in 0..n {
        let mut pivot = col;

        while pivot < n && augment[pivot][col].is_zero() {
            pivot += 1;
        }

        if pivot != col {
            augment.swap(pivot, col);
            det = det * minus_one;
        }

        let pivot_val = augment[col][col];
        det = det * pivot_val;
        let pivot_inv = pivot_val.inv();

        for i in 0..2 * n {
            augment[col][i] = augment[col][i] * pivot_inv;
        }

        let pivot_row = augment[col].clone();

        for row in 0..n {
            if row == col {
                continue;
            }

            let factor = augment[row][col];

            if factor.is_zero() {
                continue;
            }

            for i in 0..2 * n {
                augment[row][i] = augment[row][i] - factor * pivot_row[i];
            }
        }
    }

    let mut inv = vec![vec![zero; n]; n];

    for i in 0..n {
        for j in 0..n {
            inv[i][j] = augment[i][n + j];
        }
    }

    (det, inv)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut matrix = vec![vec![ModInt::new(0, MOD); 2 * n]; n];

    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = ModInt::new(scan.token::<i64>(), MOD);
        }
    }

    let zero = ModInt::new(0, MOD);
    let one = ModInt::new(1, MOD);
    let (det, inv) = gauss_elimination(matrix);

    for _ in 0..q {
        let (cmd, idx) = (scan.token::<String>(), scan.token::<usize>() - 1);
        let mut dot = zero;

        if cmd == "row" {
            for i in 0..n {
                let x = ModInt::new(scan.token::<i64>(), MOD);
                dot = dot + x * inv[i][idx];
            }
        } else {
            for i in 0..n {
                let x = ModInt::new(scan.token::<i64>(), MOD);
                dot = dot + x * inv[idx][i];
            }
        }

        writeln!(out, "{}", (det * (one + dot)).value).unwrap();
    }
}

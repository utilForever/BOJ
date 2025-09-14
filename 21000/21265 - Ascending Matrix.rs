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

const MOD: i64 = 998_244_353;

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

    let (n, m, k, r, c, v) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>() - 1,
        scan.token::<usize>() - 1,
        scan.token::<usize>(),
    );

    if k == 1 {
        writeln!(out, "1").unwrap();
        return;
    }

    let mut visited = vec![vec![ModInt::new(0, MOD); k - 1]; k - 1];
    let mut not_visited = vec![vec![ModInt::new(0, MOD); k - 1]; k - 1];

    let upper_left = (r as i64 + v as i64 - 2, c as i64 + v as i64 - 2);
    let forbidden = (r + v - 1, c + v - 1);

    for idx in 0..k - 1 {
        let mut dp = vec![vec![[ModInt::new(0, MOD); 2]; m + k + 2]; n + k + 1];
        let start = if idx as i64 <= upper_left.0 && (m + idx) as i64 <= upper_left.1 {
            1
        } else {
            0
        };

        dp[idx][m + idx][start] = ModInt::new(1, MOD);

        for y in idx..=n + k - 1 {
            for x in (0..=m + idx).rev() {
                if y == forbidden.0 && x == forbidden.1 {
                    continue;
                }

                let op = if y as i64 <= upper_left.0 && x as i64 <= upper_left.1 {
                    1
                } else {
                    0
                };

                if y > 0 {
                    let from0 = dp[y - 1][x][0];
                    let from1 = dp[y - 1][x][1];

                    if from0 != ModInt::new(0, MOD) {
                        dp[y][x][op | 0] = dp[y][x][op | 0] + from0;
                    }

                    if from1 != ModInt::new(0, MOD) {
                        dp[y][x][op | 1] = dp[y][x][op | 1] + from1;
                    }
                }

                let from0 = dp[y][x + 1][0];
                let from1 = dp[y][x + 1][1];

                if from0 != ModInt::new(0, MOD) {
                    dp[y][x][op | 0] = dp[y][x][op | 0] + from0;
                }

                if from1 != ModInt::new(0, MOD) {
                    dp[y][x][op | 1] = dp[y][x][op | 1] + from1;
                }
            }
        }

        for y in 0..k - 1 {
            visited[idx][y] = dp[n + y][y][1];
            not_visited[idx][y] = dp[n + y][y][0];
        }
    }

    let mut det = vec![ModInt::new(0, MOD); k];

    for idx in 1..=k {
        let mut mat = vec![vec![ModInt::new(0, MOD); k - 1]; k - 1];

        for i in 0..k - 1 {
            for j in 0..k - 1 {
                mat[i][j] = not_visited[i][j] + (ModInt::new(idx as i64, MOD) * visited[i][j]);
            }
        }

        det[idx - 1] = determinant(&mut mat);
    }

    let deg = v - 1;
    let mut ret = ModInt::new(0, MOD);

    for i in 1..=k {
        let mut coeff = det[i - 1];

        for j in 1..=k {
            if i == j {
                continue;
            }

            let diff = ModInt::new(i as i64 - j as i64, MOD);
            coeff = coeff * ModInt::new(diff.value, MOD).inv();
        }

        let mut poly = vec![ModInt::new(0, MOD); k];
        poly[0] = ModInt::new(1, MOD);

        for j in 1..=k {
            if i == j {
                continue;
            }

            let mut next = vec![ModInt::new(0, MOD); k];

            for d in 0..k - 1 {
                next[d] = next[d] - poly[d] * ModInt::new(j as i64, MOD);
                next[d + 1] = next[d + 1] + poly[d];
            }

            poly = next;
        }

        ret = ret + coeff * poly[deg];
    }

    writeln!(out, "{}", ret.value).unwrap();
}

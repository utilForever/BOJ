use io::Write;
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hasher},
    iter::repeat_with,
};
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

const MOD: u64 = (1u64 << 31) - 1;

#[derive(Debug, Clone, Copy)]
struct ModInt {
    value: u64,
    modulo: u64,
}

impl ModInt {
    fn new(value: u64, modulo: u64) -> Self {
        ModInt {
            value: value % modulo,
            modulo,
        }
    }

    fn pow(self, mut exp: u64) -> Self {
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

#[derive(Clone, Debug)]
pub struct Xorshift {
    y: u64,
}

impl Xorshift {
    pub fn new_with_seed(seed: u64) -> Self {
        Xorshift { y: seed }
    }

    pub fn new() -> Self {
        Xorshift::new_with_seed(RandomState::new().build_hasher().finish())
    }

    pub fn rand64(&mut self) -> u64 {
        self.y ^= self.y << 5;
        self.y ^= self.y >> 17;
        self.y ^= self.y << 11;
        self.y
    }

    pub fn rand(&mut self, k: u64) -> u64 {
        self.rand64() % k
    }

    pub fn rands(&mut self, k: u64, n: usize) -> Vec<u64> {
        repeat_with(|| self.rand(k)).take(n).collect()
    }

    pub fn randf(&mut self) -> f64 {
        const UPPER_MASK: u64 = 0x3FF0_0000_0000_0000;
        const LOWER_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;
        let x = self.rand64();
        let tmp = UPPER_MASK | (x & LOWER_MASK);
        let result: f64 = f64::from_bits(tmp);
        f64::from_bits(f64::to_bits(result - 1.0) ^ (x >> 63))
    }

    pub fn gen_bool(&mut self, p: f64) -> bool {
        self.randf() < p
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let mut n = slice.len();
        while n > 1 {
            let i = self.rand(n as _) as usize;
            n -= 1;
            slice.swap(i, n);
        }
    }
}

// Reference: https://blog.naver.com/4u_olion/221973496078
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, l) = (scan.token::<usize>(), scan.token::<usize>());
    let mut matrix = vec![0; n * l];

    for i in 0..n {
        for j in 0..l {
            matrix[i * l + j] = scan.token::<u64>();
        }
    }

    if 3 * n > l {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut rng = Xorshift::new();
    let mut check = vec![true; l - 3 * n + 1];

    let r = ModInt::new(rng.rand(MOD - 1) + 1, MOD);
    let r_inv = r.inv();
    let mut pow = vec![ModInt::new(0, MOD); n];

    pow[0] = ModInt::new(1, MOD);

    for k in 1..n {
        pow[k] = pow[k - 1] * r;
    }

    let pow_last = pow[n - 1];
    let vec_random = rng.rands(MOD, n);
    let mut sum_col = vec![ModInt::new(0, MOD); l];

    for i in 0..n {
        for j in 0..l {
            sum_col[j] =
                sum_col[j] + ModInt::new(vec_random[i], MOD) * ModInt::new(matrix[i * l + j], MOD);
        }
    }

    let mut vec_b = vec![ModInt::new(0, MOD); n];
    let mut vec_c = vec![ModInt::new(0, MOD); n];

    for i in 0..n {
        let mut sum_b = ModInt::new(0, MOD);
        let mut sum_c = ModInt::new(0, MOD);

        for j in 0..n {
            sum_b = sum_b + ModInt::new(matrix[i * l + n + j], MOD) * pow[j];
            sum_c = sum_c + ModInt::new(matrix[i * l + 2 * n + j], MOD) * pow[j];
        }

        vec_b[i] = sum_b;
        vec_c[i] = sum_c;
    }

    for i in 0..l - 3 * n + 1 {
        let mut left = ModInt::new(0, MOD);
        let mut right = ModInt::new(0, MOD);

        for j in 0..n {
            left = left + sum_col[i + j] * vec_b[j];
        }

        for j in 0..n {
            right = right + ModInt::new(vec_random[j], MOD) * vec_c[j];
        }

        if left != right {
            check[i] = false;
        }

        if i + 1 == l - 3 * n + 1 {
            break;
        }

        for j in 0..n {
            let mut x = vec_b[j] - ModInt::new(matrix[j * l + i + n], MOD);
            x = x * r_inv;
            x = x + ModInt::new(matrix[j * l + i + 2 * n], MOD) * pow_last;
            vec_b[j] = x;

            let mut y = vec_c[j] - ModInt::new(matrix[j * l + i + 2 * n], MOD);
            y = y * r_inv;
            y = y + ModInt::new(matrix[j * l + i + 3 * n], MOD) * pow_last;
            vec_c[j] = y;
        }
    }

    let mut cnt = 0;
    let mut idx = 0;

    while idx < l - 3 * n + 1 {
        if check[idx] {
            cnt += 1;
            idx += 3 * n;
        } else {
            idx += 1;
        }
    }

    writeln!(out, "{}", cnt * 3 * n * n).unwrap();
}

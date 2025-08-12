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

#[inline(always)]
fn comb(fact: &Vec<ModInt>, fact_inv: &Vec<ModInt>, a: usize, b: usize) -> ModInt {
    fact[a] * fact_inv[a - b] * fact_inv[b]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut obstacles = vec![(0, 0, 0); k];

    let start1 = (1, 2);
    let start2 = (2, 1);
    let end1 = (n - 1, m);
    let end2 = (n, m - 1);
    let mut check = true;

    for i in 0..k {
        let (x, y) = (scan.token::<usize>(), scan.token::<usize>());

        if (x, y) == start1 || (x, y) == start2 || (x, y) == end1 || (x, y) == end2 {
            check = false;
        }

        obstacles[i] = (x, y, 0);
    }

    if !check {
        writeln!(out, "0").unwrap();
        return;
    }

    obstacles.push((end1.0, end1.1, 1));
    obstacles.push((end2.0, end2.1, 2));

    obstacles.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut fact = vec![ModInt::new(1, MOD); n + m + 1];

    for i in 1..=n + m {
        fact[i] = fact[i - 1] * ModInt::new(i as i64, MOD);
    }

    let mut fact_inv = vec![ModInt::new(1, MOD); n + m + 1];
    fact_inv[n + m] = fact[n + m].inv();

    for i in (0..n + m).rev() {
        fact_inv[i] = fact_inv[i + 1] * ModInt::new((i + 1) as i64, MOD);
    }

    let mut idx_end1 = 0;
    let mut idx_end2 = 0;

    for i in 0..k + 2 {
        if obstacles[i].2 == 1 {
            idx_end1 = i;
        } else if obstacles[i].2 == 2 {
            idx_end2 = i;
        }
    }

    let mut dp1 = vec![ModInt::new(0, MOD); k + 2];
    let mut dp2 = vec![ModInt::new(0, MOD); k + 2];

    for i in 0..k + 2 {
        let (x, y, _) = obstacles[i];
        let mut total1 = if x >= start1.0 && y >= start1.1 {
            comb(
                &fact,
                &fact_inv,
                (x - start1.0) + (y - start1.1),
                x - start1.0,
            )
        } else {
            ModInt::new(0, MOD)
        };
        let mut total2 = if x >= start2.0 && y >= start2.1 {
            comb(
                &fact,
                &fact_inv,
                (x - start2.0) + (y - start2.1),
                x - start2.0,
            )
        } else {
            ModInt::new(0, MOD)
        };

        for j in 0..i {
            let dx = x as i64 - obstacles[j].0 as i64;
            let dy = y as i64 - obstacles[j].1 as i64;

            if dx < 0 || dy < 0 {
                continue;
            }

            let dx = dx as usize;
            let dy = dy as usize;
            let weight = comb(&fact, &fact_inv, dx + dy, dx);

            if dp1[j] != ModInt::new(0, MOD) {
                total1 = total1 - dp1[j] * weight;
            }

            if dp2[j] != ModInt::new(0, MOD) {
                total2 = total2 - dp2[j] * weight;
            }
        }

        dp1[i] = total1;
        dp2[i] = total2;
    }

    let ret = dp1[idx_end1] * dp2[idx_end2] - dp1[idx_end2] * dp2[idx_end1];

    writeln!(out, "{}", ret.value).unwrap();
}

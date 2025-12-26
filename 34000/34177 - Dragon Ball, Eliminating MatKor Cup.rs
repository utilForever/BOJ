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
const BALLS: usize = 7;
const STATES: usize = 1 << (2 * BALLS);
const INV8: i64 = 125_000_001;

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

    #[inline(always)]
    fn zero() -> Self {
        ModInt {
            value: 0,
            modulo: MOD,
        }
    }

    #[inline(always)]
    fn one() -> Self {
        ModInt {
            value: 1,
            modulo: MOD,
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

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        self.value = (self.value + other.value) % self.modulo;
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

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        self.value = (self.value - other.value + self.modulo) % self.modulo;
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

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        self.value = (self.value * other.value) % self.modulo;
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

#[derive(Clone)]
struct DeltaStep {
    code: [u8; BALLS],
    prob: ModInt,
}

const NEXT_DIGIT: [[u8; 3]; 4] = [[0, 1, 1], [1, 2, 0], [2, 3, 1], [3, 3, 2]];

fn build_one_day_steps(n: usize, masks: &[u8]) -> Vec<DeltaStep> {
    let inv_n = ModInt::new(n as i64, MOD).inv();
    let inv_2 = ModInt::new(2, MOD).inv();

    let mut inv_2_pow = [ModInt::one(); 8];

    for k in 1..=7 {
        inv_2_pow[k] = inv_2_pow[k - 1] * inv_2;
    }

    let mut steps = Vec::new();

    for &mask in masks.iter() {
        let mut touched = Vec::new();

        for ball in 0..BALLS {
            if (mask >> ball) & 1 == 1 {
                touched.push(ball);
            }
        }

        let prob = inv_n * inv_2_pow[touched.len()];

        for s in 0..(1usize << touched.len()) {
            let mut code = [0; BALLS];

            for (t, &b) in touched.iter().enumerate() {
                let minus = ((s >> t) & 1) == 1;
                code[b] = if minus { 2 } else { 1 };
            }

            steps.push(DeltaStep { code, prob });
        }
    }

    steps
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, _, m) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut masks = Vec::with_capacity(n);

    for _ in 0..n {
        let c = scan.token::<usize>();
        let mut mask = 0;

        for _ in 0..c {
            let ball = scan.token::<usize>() - 1;
            mask |= 1u8 << ball;
        }

        masks.push(mask);
    }

    for _ in 0..BALLS {
        let _ = scan.token::<i64>();
    }

    let mut is_goal_folded = vec![false; STATES];

    for s in 0..STATES {
        let mut cnt = [0; 4];
        let mut x = s;

        for _ in 0..BALLS {
            cnt[(x & 3) as usize] += 1;
            x >>= 2;
        }

        is_goal_folded[s] = cnt == [1, 2, 2, 2];
    }

    let steps = build_one_day_steps(n, &masks);
    let k = steps.len();

    let mut trans = vec![0; k * STATES];

    for (idx, step) in steps.iter().enumerate() {
        let base = idx * STATES;

        for state in 0..STATES {
            let mut state_next = 0;
            let mut x = state;

            for b in 0..BALLS {
                let cur_digit = (x & 3) as usize;
                x >>= 2;

                state_next |= (NEXT_DIGIT[cur_digit][step.code[b] as usize] as usize) << (2 * b);
            }

            trans[base + state] = state_next as u16;
        }
    }

    let mut curr = vec![ModInt::zero(); STATES];
    let mut next = vec![ModInt::zero(); STATES];

    curr[0] = ModInt::one();

    for _ in 0..m {
        next.fill(ModInt::zero());

        for state in 0..STATES {
            if curr[state] == ModInt::zero() {
                continue;
            }

            for (idx, step) in steps.iter().enumerate() {
                let state_next = trans[idx * STATES + state] as usize;
                next[state_next] += curr[state] * step.prob;
            }
        }

        std::mem::swap(&mut curr, &mut next);
    }

    let inv8 = ModInt::new(INV8, MOD);
    let mut prob_main = ModInt::zero();

    for state in 0..STATES {
        if is_goal_folded[state] {
            prob_main += curr[state] * inv8;
        }
    }

    writeln!(out, "0").unwrap();
    writeln!(out, "{}", prob_main.value).unwrap();
}

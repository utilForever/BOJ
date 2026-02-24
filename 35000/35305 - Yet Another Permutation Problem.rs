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

#[derive(Debug, Clone, Copy)]
struct ModInt {
    value: i64,
}

impl ModInt {
    fn new(value: i64) -> Self {
        ModInt { value: value % MOD }
    }

    #[inline(always)]
    fn zero() -> Self {
        ModInt { value: 0 }
    }

    #[inline(always)]
    fn one() -> Self {
        ModInt { value: 1 }
    }
}

impl std::ops::Add for ModInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ModInt {
            value: (self.value + other.value) % MOD,
        }
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        self.value = (self.value + other.value) % MOD;
    }
}

impl std::ops::Sub for ModInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        ModInt {
            value: (self.value - other.value + MOD) % MOD,
        }
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        self.value = (self.value - other.value + MOD) % MOD;
    }
}

impl std::ops::Mul for ModInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        ModInt {
            value: (self.value * other.value) % MOD,
        }
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        self.value = (self.value * other.value) % MOD;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Constraint {
    Free,
    Greater,
    Less,
}

struct Precomputation {
    bounce_one_choices: Vec<ModInt>,
    bounce_one_inv_sum: Vec<ModInt>,

    fixed_inv_sum: Vec<ModInt>,

    down_choices: Vec<ModInt>,
    down_inv_sum: Vec<ModInt>,
    down_cycle_sum: Vec<ModInt>,
    down_cycle_inv_sum: Vec<ModInt>,
}

impl Precomputation {
    fn new(n: usize) -> Self {
        let mut bounce_one_choices = vec![ModInt::zero(); n + 1];
        let mut bounce_one_inv_sum = vec![ModInt::zero(); n + 1];

        let mut fixed_inv_sum = vec![ModInt::zero(); n + 1];

        let mut down_choices = vec![ModInt::zero(); n + 1];
        let mut down_inv_sum = vec![ModInt::zero(); n + 1];
        let mut down_cycle_sum = vec![ModInt::zero(); n + 1];
        let mut down_cycle_inv_sum = vec![ModInt::zero(); n + 1];

        for i in 1..=n {
            let h = i as i64;

            bounce_one_choices[i] = ModInt::new(h);
            bounce_one_inv_sum[i] = ModInt::new(h * (3 * h - 1) / 2);

            fixed_inv_sum[i] = ModInt::new(2 * h);

            down_choices[i] = ModInt::new(h * h);
            down_inv_sum[i] = ModInt::new(h * h * (3 * h - 2));
            down_cycle_sum[i] = ModInt::new(h);
            down_cycle_inv_sum[i] = ModInt::new(h * (3 * h - 2));
        }

        Self {
            bounce_one_choices,
            bounce_one_inv_sum,

            fixed_inv_sum,

            down_choices,
            down_inv_sum,
            down_cycle_sum,
            down_cycle_inv_sum,
        }
    }
}

fn apply_step(
    next_ways: &mut Vec<ModInt>,
    next_inv_sum: &mut Vec<ModInt>,
    next_cycle_sum: &mut Vec<ModInt>,
    next_cycle_inv_sum: &mut Vec<ModInt>,
    idx: usize,
    cnt_choices: ModInt,
    sum_delta_inv: ModInt,
    sum_delta_cycle: ModInt,
    sum_delta_cycle_inv: ModInt,
    ways: ModInt,
    inv_sum: ModInt,
    cycle_sum: ModInt,
    cycle_inv_sum: ModInt,
) {
    next_ways[idx] += ways * cnt_choices;

    next_inv_sum[idx] += inv_sum * cnt_choices;
    next_inv_sum[idx] += ways * sum_delta_inv;

    next_cycle_sum[idx] += cycle_sum * cnt_choices;
    next_cycle_sum[idx] += ways * sum_delta_cycle;

    next_cycle_inv_sum[idx] += cycle_inv_sum * cnt_choices;
    next_cycle_inv_sum[idx] += inv_sum * sum_delta_cycle;
    next_cycle_inv_sum[idx] += cycle_sum * sum_delta_inv;
    next_cycle_inv_sum[idx] += ways * sum_delta_cycle_inv;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut conditions = vec![Constraint::Free; n + 1];

    for _ in 0..m {
        let (t, i) = (scan.token::<usize>(), scan.token::<usize>());
        conditions[i] = if t == 1 {
            Constraint::Greater
        } else {
            Constraint::Less
        };
    }

    let precomputation = Precomputation::new(n);

    let mut dp_ways = vec![ModInt::zero(); n + 2];
    let mut dp_inv_sum = vec![ModInt::zero(); n + 2];
    let mut dp_cycle_sum = vec![ModInt::zero(); n + 2];
    let mut dp_cycle_inv_sum = vec![ModInt::zero(); n + 2];

    let mut next_ways = vec![ModInt::zero(); n + 2];
    let mut next_inv_sum = vec![ModInt::zero(); n + 2];
    let mut next_cycle_sum = vec![ModInt::zero(); n + 2];
    let mut next_cycle_inv_sum = vec![ModInt::zero(); n + 2];

    dp_ways[0] = ModInt::one();

    for i in 1..=n {
        next_ways.fill(ModInt::zero());
        next_inv_sum.fill(ModInt::zero());
        next_cycle_sum.fill(ModInt::zero());
        next_cycle_inv_sum.fill(ModInt::zero());

        for j in 0..=(i - 1).min(n - i + 1) {
            let ways = dp_ways[j];
            let inv_sum = dp_inv_sum[j];
            let cycle_sum = dp_cycle_sum[j];
            let cycle_inv_sum = dp_cycle_inv_sum[j];

            if (ways.value | inv_sum.value | cycle_sum.value | cycle_inv_sum.value) == 0 {
                continue;
            }

            if conditions[i] != Constraint::Less && j + 1 <= n - i {
                apply_step(
                    &mut next_ways,
                    &mut next_inv_sum,
                    &mut next_cycle_sum,
                    &mut next_cycle_inv_sum,
                    j + 1,
                    ModInt::one(),
                    ModInt::zero(),
                    ModInt::zero(),
                    ModInt::zero(),
                    ways,
                    inv_sum,
                    cycle_sum,
                    cycle_inv_sum,
                );
            }

            if j <= n - i {
                if j > 0 {
                    match conditions[i] {
                        Constraint::Free => {
                            for _ in 0..2 {
                                apply_step(
                                    &mut next_ways,
                                    &mut next_inv_sum,
                                    &mut next_cycle_sum,
                                    &mut next_cycle_inv_sum,
                                    j,
                                    precomputation.bounce_one_choices[j],
                                    precomputation.bounce_one_inv_sum[j],
                                    ModInt::zero(),
                                    ModInt::zero(),
                                    ways,
                                    inv_sum,
                                    cycle_sum,
                                    cycle_inv_sum,
                                );
                            }
                        }
                        Constraint::Greater | Constraint::Less => {
                            apply_step(
                                &mut next_ways,
                                &mut next_inv_sum,
                                &mut next_cycle_sum,
                                &mut next_cycle_inv_sum,
                                j,
                                precomputation.bounce_one_choices[j],
                                precomputation.bounce_one_inv_sum[j],
                                ModInt::zero(),
                                ModInt::zero(),
                                ways,
                                inv_sum,
                                cycle_sum,
                                cycle_inv_sum,
                            );
                        }
                    }
                }
            }

            if conditions[i] == Constraint::Free {
                apply_step(
                    &mut next_ways,
                    &mut next_inv_sum,
                    &mut next_cycle_sum,
                    &mut next_cycle_inv_sum,
                    j,
                    ModInt::one(),
                    precomputation.fixed_inv_sum[j],
                    ModInt::one(),
                    precomputation.fixed_inv_sum[j],
                    ways,
                    inv_sum,
                    cycle_sum,
                    cycle_inv_sum,
                );
            }

            if conditions[i] != Constraint::Greater && j > 0 {
                apply_step(
                    &mut next_ways,
                    &mut next_inv_sum,
                    &mut next_cycle_sum,
                    &mut next_cycle_inv_sum,
                    j - 1,
                    precomputation.down_choices[j],
                    precomputation.down_inv_sum[j],
                    precomputation.down_cycle_sum[j],
                    precomputation.down_cycle_inv_sum[j],
                    ways,
                    inv_sum,
                    cycle_sum,
                    cycle_inv_sum,
                );
            }
        }

        std::mem::swap(&mut dp_ways, &mut next_ways);
        std::mem::swap(&mut dp_inv_sum, &mut next_inv_sum);
        std::mem::swap(&mut dp_cycle_sum, &mut next_cycle_sum);
        std::mem::swap(&mut dp_cycle_inv_sum, &mut next_cycle_inv_sum);
    }

    writeln!(
        out,
        "{}",
        (ModInt::new(n as i64) * dp_inv_sum[0] - dp_cycle_inv_sum[0]).value
    )
    .unwrap();
}

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

fn pow(mut base: i64, mut exp: i64) -> i64 {
    let mut ret = 1;

    base %= MOD;

    while exp > 0 {
        if exp & 1 == 1 {
            ret = ret * base % MOD;
        }

        base = base * base % MOD;
        exp >>= 1;
    }

    ret
}

const MOD: i64 = 1_000_000_009;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    nums.sort_unstable();

    let mut groups = Vec::new();
    let mut left = 0;

    while left < n {
        let mut right = left + 1;

        while right < n && nums[left] == nums[right] {
            right += 1;
        }

        groups.push(right - left);
        left = right;
    }

    let mut factorial = vec![0; n + 1];
    factorial[0] = 1;

    for i in 1..=n {
        factorial[i] = factorial[i - 1] * i as i64 % MOD;
    }

    let mut factorial_inv = vec![0; n + 1];
    factorial_inv[n] = pow(factorial[n], MOD - 2);

    for i in (0..n).rev() {
        factorial_inv[i] = factorial_inv[i + 1] * (i + 1) as i64 % MOD;
    }

    let mut group_of = vec![0; n + 1];
    let mut idx_in_group = vec![0; n + 1];
    let mut pos = 1;

    for (idx, &cnt) in groups.iter().enumerate() {
        for i in 1..=cnt {
            group_of[pos] = idx + 1;
            idx_in_group[pos] = i;
            pos += 1;
        }
    }

    let mut prefix_group_fact = vec![1; groups.len() + 1];
    let mut prefix_group_fact_inv = vec![1; groups.len() + 1];

    for i in 1..=groups.len() {
        prefix_group_fact_inv[i] =
            prefix_group_fact_inv[i - 1] * factorial_inv[groups[i - 1]] % MOD;
        prefix_group_fact[i] = prefix_group_fact[i - 1] * factorial[groups[i - 1]] % MOD;
    }

    let interval = |start: usize, end: usize| -> i64 {
        if start > end {
            return 1;
        }

        if group_of[start] == group_of[end] {
            return 1;
        }

        let len = end - start + 1;
        let left = groups[group_of[start] - 1] - idx_in_group[start] + 1;
        let right = idx_in_group[end];
        let mid =
            prefix_group_fact_inv[group_of[end] - 1] * prefix_group_fact[group_of[start]] % MOD;

        factorial[len] * factorial_inv[left] % MOD * factorial_inv[right] % MOD * mid % MOD
    };

    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        let mut sum = 0;

        for j in 0..i {
            sum = (sum + dp[j] * interval(j + 1, i - 1)) % MOD;
        }

        dp[i] = (MOD - sum) % MOD;
    }

    let mut ret = 0;

    for i in 0..=n {
        ret = (ret + dp[i] * interval(i + 1, n)) % MOD;
    }

    writeln!(out, "{ret}").unwrap();
}

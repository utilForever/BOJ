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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut problems = vec![0; n];
    let mut sum = 0;

    for i in 0..n {
        let a = scan.token::<i64>();

        problems[i] = a * a;
        sum += problems[i];
    }

    let k = n - 1;
    let len = 1usize << k;

    let mut inv = vec![0; sum as usize + 1];
    inv[1] = 1;

    for i in 2..=sum as usize {
        inv[i] = MOD - (MOD / i as i64) * inv[(MOD % i as i64) as usize] % MOD;
    }

    let mut sum_problems = vec![0; len];

    for mask in 1..len {
        let lsb = mask & (!mask + 1);
        let idx = lsb.trailing_zeros() as usize;

        sum_problems[mask] = sum_problems[mask ^ lsb] + problems[idx + 1];
    }

    let mut masks = vec![Vec::new(); n];

    for mask in 0..len {
        let idx = mask.count_ones() as usize;
        masks[idx].push(mask);
    }

    let mut dp = vec![0; len];
    dp[0] = 1;

    for i in 0..m {
        let mut dp_next = vec![0; len];

        for &mask in masks[i].iter() {
            if dp[mask] == 0 {
                continue;
            }

            let denominator = sum - sum_problems[mask];
            let factor = inv[denominator as usize] % MOD;
            let mut rem = (len - 1) ^ mask;

            while rem != 0 {
                let lsb = rem & (!rem + 1);
                let idx = lsb.trailing_zeros() as usize;
                let prob = (problems[idx + 1] % MOD) * factor % MOD;
                let mask_next = mask | lsb;

                dp_next[mask_next] = (dp_next[mask_next] + dp[mask] * prob % MOD) % MOD;
                rem ^= lsb;
            }
        }

        dp = dp_next;
    }

    let mut ret = 0;

    for &val in dp.iter() {
        ret = (ret + val) % MOD;
    }

    writeln!(out, "{}", (MOD + 1 - ret) % MOD).unwrap();
}

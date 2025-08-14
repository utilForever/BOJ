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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, r) = (scan.token::<usize>(), scan.token::<usize>());
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut x = vec![0; n];
    let mut m = vec![0; n];

    for i in 0..n {
        (a[i], b[i], x[i], m[i]) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    let mut dp = vec![0; 1 << n];
    let mut sum = vec![0; 1 << n];

    for _ in 0..r {
        for i in 0..n {
            x[i] = (a[i] * x[i] + b[i]) % m[i];
        }

        let total = x.iter().sum::<i64>();

        sum[0] = 0;

        for mask in 1..(1 << n) {
            let mask_partial = mask & (mask - 1);
            let bit: usize = mask ^ mask_partial;
            let idx = bit.trailing_zeros() as usize;

            sum[mask] = sum[mask_partial] + x[idx];
        }

        let mut next = vec![i64::MIN / 4; 1 << n];

        for s in 0..(1 << n) {
            let base = dp[s];
            let sum_front = sum[s];

            for j in 0..n {
                let bit = 1 << j;
                let state_new = s ^ bit;

                let gain = if (s & bit) != 0 {
                    (total - sum_front) + x[j]
                } else {
                    sum_front + x[j]
                };

                let cand = base + gain;
                next[state_new] = next[state_new].max(cand);
            }
        }

        dp = next;
    }

    writeln!(out, "{}", dp.iter().max().unwrap()).unwrap();
}

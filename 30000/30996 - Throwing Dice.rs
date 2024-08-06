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
}

const NCOEF: usize = 28;
const COF: [f64; 28] = [
    -1.3026537197817094,
    6.4196979235649026e-1,
    1.9476473204185836e-2,
    -9.561514786808631e-3,
    -9.46595344482036e-4,
    3.66839497852761e-4,
    4.2523324806907e-5,
    -2.0278578112534e-5,
    -1.624290004647e-6,
    1.303655835580e-6,
    1.5626441722e-8,
    -8.5238095915e-8,
    6.529054439e-9,
    5.059343495e-9,
    -9.91364156e-10,
    -2.27365122e-10,
    9.6467911e-11,
    2.394038e-12,
    -6.886027e-12,
    8.94487e-13,
    3.13092e-13,
    -1.12708e-13,
    3.81e-16,
    7.106e-15,
    -1.523e-15,
    -9.4e-17,
    1.21e-16,
    -2.8e-17,
];

fn erfc_cheb(z: f64) -> f64 {
    let mut d = 0.0_f64;
    let mut dd = 0.0_f64;

    let t = 2.0_f64 / (2.0_f64 + z);
    let ty = 4.0_f64 * t - 2.0_f64;

    for j in (1..NCOEF - 1).rev() {
        let tmp = d;
        d = ty * d - dd + COF[j];
        dd = tmp;
    }

    t * (-z.powi(2) + 0.5 * (COF[0] + ty * d) - dd).exp()
}

fn erf(x: f64) -> f64 {
    if x >= 0.0_f64 {
        1.0 - erfc_cheb(x)
    } else {
        erfc_cheb(-x) - 1.0_f64
    }
}

fn cdf(cnt: usize, average: f64, variance: f64, val: f64) -> f64 {
    0.5 * (1.0 + erf((val - cnt as f64 * average) / (2.0 * cnt as f64 * variance).sqrt()))
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut dice = vec![0; n + 1];

    for i in 1..=n {
        dice[i] = scan.token::<i64>();
    }

    let dice_sum = dice.iter().sum::<i64>() as f64;

    if m >= 100 {
        let average = dice
            .iter()
            .enumerate()
            .map(|(idx, &num)| idx as f64 * num as f64)
            .sum::<f64>()
            / dice_sum;
        let variance = dice
            .iter()
            .enumerate()
            .map(|(idx, &num)| (idx as f64 - average) * (idx as f64 - average) * num as f64)
            .sum::<f64>()
            / dice_sum;

        for _ in 0..q {
            let x = scan.token::<f64>();
            let ret = if variance == 0.0 {
                if x == m as f64 * average {
                    1.0
                } else {
                    0.0
                }
            } else {
                cdf(m, average, variance, x + 0.5)
            };

            writeln!(out, "{:.10}", ret).unwrap();
        }
    } else {
        let mut dp = vec![0.0; n * m + 1];
        dp[0] = 1.0;

        for _ in 1..=m {
            let mut val = vec![0.0; n * m + 1];

            for i in 1..=n {
                for j in 1..=n * m {
                    if j < i {
                        continue;
                    }

                    val[j] += dp[j - i] * (dice[i] as f64 / dice_sum);
                }
            }

            dp = val;
        }

        let mut ret = vec![0.0; n * m + 1];

        for i in 1..=n * m {
            ret[i] = ret[i - 1] + dp[i];
        }

        for _ in 0..q {
            let x = scan.token::<usize>();
            writeln!(out, "{:.10}", ret[x]).unwrap();
        }
    }
}

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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const MOD: i64 = 1_000_000_007;

fn pow(x: i64, mut y: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % MOD;

    while y != 0 {
        if y & 1 != 0 {
            ret = ret * piv % MOD
        }

        piv = piv * piv % MOD;
        y >>= 1;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut balls = vec![0; n];

    for i in 0..n {
        balls[i] = scan.token::<i64>();
    }

    let mut prefix_sum = vec![0; n + 1];

    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i - 1] + balls[i - 1];
    }

    let mut delta = vec![0; n + 2];

    for _ in 0..q {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
        let total = prefix_sum[r] - prefix_sum[l - 1];
        let total_inv = pow(total, MOD - 2);

        delta[l] = (delta[l] + total_inv) % MOD;

        if r + 1 <= n {
            delta[r + 1] = (delta[r + 1] - total_inv + MOD) % MOD;
        }
    }

    let mut sum = vec![0; n + 1];

    for i in 1..=n {
        sum[i] = (sum[i - 1] + delta[i]) % MOD;
    }

    let mut ret = vec![0; n];

    for i in 0..n {
        let c = balls[i] * (balls[i] - 1) % MOD;
        ret[i] = c * sum[i + 1] % MOD;
    }

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}

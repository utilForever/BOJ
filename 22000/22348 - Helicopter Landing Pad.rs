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

    let t = scan.token::<usize>();
    let mut a = vec![0; t];
    let mut b = vec![0; t];
    let mut sum_max = 0;

    for i in 0..t {
        a[i] = scan.token::<usize>();
        b[i] = scan.token::<usize>();

        sum_max = sum_max.max(a[i] + b[i]);
    }

    let mut k = 0;
    let mut sum_k = 0;

    while sum_k + k + 1 <= sum_max {
        k += 1;
        sum_k += k;
    }

    let mut dp = vec![0; sum_k + 1];
    let mut prefix_sum = vec![0; sum_k + 1];
    let mut ret = vec![0; t];

    dp[0] = 1;

    let mut sum_radius = 0;

    for r in 1..=k {
        sum_radius += r;

        for i in (r..=sum_radius).rev() {
            dp[i] = (dp[i] + dp[i - r]) % MOD;
        }

        prefix_sum[0] = dp[0];

        for i in 1..=sum_radius {
            prefix_sum[i] = (prefix_sum[i - 1] + dp[i]) % MOD;
        }

        for i in 0..t {
            let u = if a[i] < sum_radius { a[i] } else { sum_radius };
            let v = if b[i] < sum_radius {
                sum_radius - b[i]
            } else {
                0
            };

            if u >= v {
                let mut val = prefix_sum[u];

                if v > 0 {
                    val = (val + MOD - prefix_sum[v - 1]) % MOD;
                }

                ret[i] = (ret[i] + val) % MOD;
            }
        }
    }

    for i in 0..t {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}

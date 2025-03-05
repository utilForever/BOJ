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

fn choose_two(n: i64) -> i64 {
    n * (n - 1) / 2
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut factorial = vec![1; 1_000_001];
    let mut factorial_inv = vec![1; 1_000_001];

    for i in 2..=1_000_000 {
        factorial[i] = factorial[i - 1] * i as i64 % MOD;
    }

    factorial_inv[1_000_000] = pow(factorial[1_000_000], MOD - 2);

    for i in (1..=1_000_000).rev() {
        factorial_inv[i - 1] = factorial_inv[i] * i as i64 % MOD;
    }

    let (n, m) = (scan.token::<i64>(), scan.token::<i64>());
    let mut nums = vec![0; n as usize + 1];
    let mut sum = 0;

    for i in 1..=n {
        nums[i as usize] = scan.token::<i64>();
        sum = (sum + nums[i as usize]) % MOD;
    }

    let q = scan.token::<char>();

    if q == 'S' {
        let temp = pow((choose_two(n) % MOD * 3 + n) % MOD, MOD - 2);
        let mut ret = 0;

        for i in 1..=n {
            let temp1 =
                (n - 1 + i + 3 * choose_two(i - 1) % MOD + 3 * choose_two(n - i) % MOD) % MOD;
            let temp2 = (choose_two(n) - choose_two(i - 1) - choose_two(n - i)) % MOD * i % MOD;
            let temp3 = choose_two(i) % MOD * (n - i + 1) % MOD + i * (n - i) % MOD;
            let temp4 = (n + i + 1) * (n - i) / 2 % MOD * i % MOD + i * (i - 1) % MOD;

            ret = (ret
                + nums[i as usize] * pow((temp1 + temp2 + temp3 + temp4) % MOD * temp % MOD, m)
                    % MOD)
                % MOD;
        }

        writeln!(out, "{}", (ret * pow(sum, MOD - 2)) % MOD).unwrap();
    } else {
        let temp = pow((choose_two(n) % MOD * 3 + n) % MOD, MOD - 2);
        let mut ret = n * (n + 1) / 2 % MOD;

        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        dp[1] = 2;

        let mut temp1 = 0;

        for i in 2..=n {
            dp[i as usize] = (dp[i as usize - 1] + factorial_inv[i as usize]) % MOD;
            temp1 = (temp1 + factorial[i as usize] * dp[i as usize - 2] % MOD) % MOD;
        }

        let mut temp2 = n - 1;

        for i in 2..=n - 1 {
            temp2 += i * i % MOD * pow(i - 1, MOD - 2) % MOD * (pow(i, n - i) - 1 + MOD) % MOD;
            temp2 %= MOD;
        }

        let mut temp3 = 0;

        for i in 2..=n {
            temp3 += i * i % MOD * pow(i - 1, MOD - 2) % MOD * (pow(i, i - 1) - 1 + MOD) % MOD;
            temp3 %= MOD;
        }

        ret = (ret + temp1 + temp2 + temp3) % MOD * temp % MOD;

        writeln!(out, "{}", pow(ret, m)).unwrap();
    }
}

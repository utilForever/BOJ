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

fn multiply(x: i64, y: i64, modular: i64) -> i64 {
    (x as i128 * y as i128 % modular as i128) as i64
}

fn pow(x: i64, mut y: i64, p: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % p;

    while y != 0 {
        if y & 1 != 0 {
            ret = multiply(ret, piv, p);
        }

        piv = multiply(piv, piv, p);
        y >>= 1;
    }

    ret
}

fn process(factors_inverse: &Vec<i64>, values: &Vec<i64>, m: i64, r: i64) -> i64 {
    if r < m + 2 {
        return values[r as usize];
    }

    let mut val = 1;
    let mut ret = 0;

    for i in 0..m + 2 {
        val = (val * (r - i)) % MOD;
    }

    for i in 1..m + 2 {
        let mut temp = val * pow(r - i, MOD - 2, MOD) % MOD;
        temp = temp * factors_inverse[i as usize] % MOD;
        temp = temp * factors_inverse[(m + 1 - i) as usize] % MOD;
        temp = temp * values[i as usize] % MOD;

        ret = if (m + 1 - i) % 2 == 1 {
            (ret + MOD - temp) % MOD
        } else {
            (ret + temp) % MOD
        };
    }

    ret
}

const MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut factors = vec![1];

    for i in 1..=1_000_001 {
        factors.push((factors.last().unwrap() * i) % MOD);
    }

    let mut factors_inverse = vec![0; 1_000_002];
    factors_inverse[1_000_001] = pow(factors[1_000_001], MOD - 2, MOD);

    for i in (0..1_000_001).rev() {
        factors_inverse[i] = (factors_inverse[i + 1] * (i + 1) as i64) % MOD;
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<i64>(), scan.token::<i64>());

        if m % 2 == 1 {
            writeln!(out, "0").unwrap();
            continue;
        } else if m == 0 {
            writeln!(out, "1").unwrap();
            continue;
        }

        let mut values = vec![0];

        for i in 1..=(n + 1).min(m + 2) {
            values.push((values.last().unwrap() + pow(i, m, MOD)) % MOD);
        }

        writeln!(
            out,
            "{}",
            if n % 2 == 1 {
                pow(n + 1, MOD - 2, MOD) * pow(pow(n, MOD - 2, MOD), m, MOD) % MOD
                    * 2
                    * (process(&factors_inverse, &values, m, n)
                        - pow(2, m, MOD) * process(&factors_inverse, &values, m, n / 2) % MOD
                        + MOD)
                    % MOD
            } else {
                pow(n + 1, MOD - 2, MOD) * pow(pow(n, MOD - 2, MOD), m, MOD) % MOD
                    * pow(2, m + 1, MOD)
                    % MOD
                    * process(&factors_inverse, &values, m, n / 2)
                    % MOD
            }
        )
        .unwrap();
    }
}

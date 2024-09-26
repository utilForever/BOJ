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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut factorial = vec![0; 5_000_001];
    let mut factorial_inv = vec![0; 5_000_001];

    factorial[0] = 1;
    factorial[1] = 1;

    for i in 2..=5_000_000 {
        factorial[i] = (factorial[i - 1] * i as i64) % MOD;
    }

    factorial_inv[5_000_000] = pow(factorial[5_000_000], MOD - 2);

    for i in (1..=5_000_000).rev() {
        factorial_inv[i - 1] = (factorial_inv[i] * i as i64) % MOD;
    }

    let mut d = vec![0; 5_000_001];

    d[0] = 1;
    d[1] = (m - n) as i64;

    for i in 2..=n {
        d[i] = (d[i - 2] * (i - 1) as i64 % MOD
            + d[i - 1] * (i - 1) as i64 % MOD
            + d[i - 1] * (m - n) as i64 % MOD)
            % MOD;
    }

    for i in 0..=n {
        let k = factorial[n] * factorial[m - n] % MOD * factorial_inv[n - i] % MOD
            * factorial_inv[i]
            % MOD
            * factorial_inv[m]
            % MOD;
        write!(out, "{} ", k * d[n - i] % MOD).unwrap();
    }

    writeln!(out).unwrap();
}

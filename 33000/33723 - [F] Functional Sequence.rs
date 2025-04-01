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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut b = vec![0; n];

    for i in 0..n {
        b[i] = scan.token::<i64>();
    }

    if k == 0 {
        for val in b {
            write!(out, "{} ", val % MOD).unwrap();
        }

        writeln!(out).unwrap();
        return;
    }

    let mut factorial = vec![1; n + 1];

    for i in 1..=n {
        factorial[i] = (factorial[i - 1] * i as i64) % MOD;
    }

    let mut factorial_inv = vec![1; n + 1];
    factorial_inv[n] = pow(factorial[n], MOD - 2);

    for i in (0..n).rev() {
        factorial_inv[i] = (factorial_inv[i + 1] * (i + 1) as i64) % MOD;
    }

    let mut power = vec![1; n + 1];

    for i in 1..=n {
        power[i] = (power[i - 1] * k) % MOD;
    }

    let mut a = vec![0; n];

    for i in 0..n {
        let mut sum = 0;

        for j in 0..=i {
            let bionomial = (factorial[i] * factorial_inv[j] % MOD) * factorial_inv[i - j] % MOD;
            sum = (sum + bionomial * power[i - j] % MOD * b[j] % MOD) % MOD;
        }

        a[i] = sum;
    }

    for val in a {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}

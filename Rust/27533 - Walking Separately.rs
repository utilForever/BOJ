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

fn multiply(x: i64, y: i64) -> i64 {
    (x as i128 * y as i128 % MOD as i128) as i64
}

fn pow(x: i64, mut y: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % MOD;

    while y != 0 {
        if y & 1 != 0 {
            ret = multiply(ret, piv);
        }

        piv = multiply(piv, piv);
        y >>= 1;
    }

    ret
}

fn combination(factorial: &Vec<i64>, n: usize, r: usize) -> i64 {
    factorial[n] * pow(factorial[r] * factorial[n - r] % MOD, MOD - 2) % MOD
}

fn get_narayana_number(n: usize, k: usize) -> i64 {
    let mut factorial = vec![0; n + k + 1];
    factorial[0] = 1;

    for i in 1..=n + k {
        factorial[i] = (i as i64) * factorial[i - 1] % MOD;
    }

    let ret1 = combination(&factorial, n + k - 4, n - 2) % MOD;
    let ret2 = combination(&factorial, n + k - 3, n - 2) % MOD;

    (ret1 * ret2) % MOD * pow(n as i64 - 1, MOD - 2) % MOD
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    writeln!(out, "{}", 2 * get_narayana_number(n, m) % MOD).unwrap();
}

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

const MOD: i64 = 998_244_353;

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

fn combination(factorial: &Vec<i64>, factorial_inv: &Vec<i64>, n: usize, k: usize) -> i64 {
    if k > n {
        return 0;
    }

    factorial[n] * factorial_inv[k] % MOD * factorial_inv[n - k] % MOD
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<i64>(), scan.token::<i64>());
    let _ = scan.token::<String>();

    if m >= 25 * n {
        writeln!(out, "{}", pow(26, n)).unwrap();
        return;
    }

    let k = (n + m) as usize;
    let mut factorial = vec![1; k + 1];

    for i in 2..=k {
        factorial[i] = factorial[i - 1] * i as i64 % MOD;
    }

    let mut factorial_inv = vec![1; k + 1];
    factorial_inv[k] = pow(factorial[k], MOD - 2);

    for i in (1..=k).rev() {
        factorial_inv[i - 1] = factorial_inv[i] * i as i64 % MOD;
    }

    let mut ret = 0;

    for i in 0..=n.min(m / 26) {
        let c1 = combination(&factorial, &factorial_inv, n as usize, i as usize);
        let c2 = combination(
            &factorial,
            &factorial_inv,
            (m - 26 * i) as usize + n as usize,
            n as usize,
        );

        if i % 2 == 0 {
            ret = (ret + c1 * c2).rem_euclid(MOD);
        } else {
            ret = (ret - c1 * c2 + MOD).rem_euclid(MOD);
        }
    }

    writeln!(out, "{ret}").unwrap();
}

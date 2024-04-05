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

const MOD: i64 = 998_244_353;

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut matrix = vec![vec![0; c]; r];

    for i in 0..r {
        for j in 0..c {
            matrix[i][j] = scan.token::<i64>();
        }
    }

    let mut permutations = vec![0; r];

    for i in 0..r {
        permutations[i] = scan.token::<i64>();
    }

    let mut factorial = vec![0; r * c + 1];
    factorial[0] = 1;
    factorial[1] = 1;

    for i in 2..=r * c {
        factorial[i] = factorial[i - 1] * i as i64 % MOD;
    }

    let mut ret = 1;
    let mut num = r * c;

    while num > 0 {
        ret = ret * combination(&factorial, num, c) % MOD;
        ret = multiply(ret, factorial[c]) % MOD;
        num -= c;
    }

    ret = ret * pow(factorial[r], MOD - 2) % MOD;

    writeln!(out, "{ret}").unwrap();
}

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

const A_MAX: usize = 3_000_003;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut factorial = vec![0; A_MAX + 1];
    factorial[0] = 1;

    for i in 1..=A_MAX {
        factorial[i] = factorial[i - 1] * i as i64 % MOD;
    }

    let mut factorial_inv = vec![0; A_MAX + 1];
    factorial_inv[A_MAX] = pow(factorial[A_MAX], MOD - 2);

    for i in (0..A_MAX).rev() {
        factorial_inv[i] = factorial_inv[i + 1] * (i + 1) as i64 % MOD;
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (a1, a2, a3) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if a1 != a3 + 2 {
            writeln!(out, "0").unwrap();
            continue;
        }

        // Case 1: The number of degrees of root is 1
        let c1 = pow(2, a2 as i64 + 1) * factorial[a2 + 2 * a3] % MOD * factorial_inv[a3 + 1] % MOD
            * factorial_inv[a2]
            % MOD
            * factorial_inv[a3]
            % MOD;
        // Case 2: The number of degrees of root is 2
        let c2 = if a2 == 0 {
            0
        } else {
            pow(2, a2 as i64) * factorial[a2 + 2 * a3] % MOD * factorial_inv[a3 + 2] % MOD
                * factorial_inv[a2 - 1]
                % MOD
                * factorial_inv[a3]
                % MOD
        };

        writeln!(out, "{}", (c1 + c2) % MOD).unwrap();
    }
}

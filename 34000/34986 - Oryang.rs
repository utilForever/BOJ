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

const N_MAX: usize = 2000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut factorial = vec![0; N_MAX + 1];
    factorial[0] = 1;

    for i in 1..=N_MAX {
        factorial[i] = factorial[i - 1] * i as i64 % MOD;
    }

    let mut factorial_inv = vec![0; N_MAX + 1];
    factorial_inv[N_MAX] = pow(factorial[N_MAX], MOD - 2);

    for i in (0..N_MAX).rev() {
        factorial_inv[i] = factorial_inv[i + 1] * (i + 1) as i64 % MOD;
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut notes = vec![0; n];

        for i in 0..n {
            notes[i] = scan.token::<i64>();
        }

        if notes.iter().all(|&x| x == 1) {
            writeln!(out, "1").unwrap();
            continue;
        }

        let mut ret = factorial[n];

        for i in 0..n {
            ret = ret * factorial_inv[notes[i] as usize] % MOD;
        }

        writeln!(out, "{ret}").unwrap();
    }
}

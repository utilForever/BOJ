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

fn comb(fact: &Vec<i64>, fact_inv: &Vec<i64>, a: usize, b: usize) -> i64 {
    fact[a] * fact_inv[a - b] % MOD * fact_inv[b] % MOD
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut fact = vec![1; 100001];
    let mut fact_inv = vec![1; 100001];

    for i in 1..=100000 {
        fact[i] = (fact[i - 1] * i as i64) % MOD;
    }

    fact_inv[100000] = pow(fact[100000], MOD - 2);

    for i in (1..=100000).rev() {
        fact_inv[i - 1] = (fact_inv[i] * i as i64) % MOD;
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, a, mut b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if b % 2 == 1 {
            writeln!(out, "0").unwrap();
            continue;
        }

        b /= 2;

        if b == 0 && c > 0 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let needed = 2 * a + 3 * b + c - 1;

        if needed > n {
            writeln!(out, "0").unwrap();
            continue;
        }

        let rest = n - needed;
        let mut ret = fact[a] * fact[2 * b] % MOD * fact[c] % MOD;

        ret = ret * comb(&fact, &fact_inv, b + c - 1, b - 1) % MOD;
        ret = ret * comb(&fact, &fact_inv, a + b, a) % MOD;
        ret = ret * comb(&fact, &fact_inv, rest + a + b, a + b) % MOD;

        writeln!(out, "{ret}").unwrap();
    }
}

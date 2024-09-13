use io::Write;
use std::{collections::HashMap, io, str};

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
const THRESHOLD: usize = 50_000_000;

fn s(n: usize, m: &mut HashMap<usize, i64>, iphi: &[i64]) -> i64 {
    if n < THRESHOLD {
        return iphi[n];
    }

    if let Some(&v) = m.get(&n) {
        return v;
    }

    let nm = n as i64 % MOD;
    let mut ret = nm * (nm + 1) % MOD * (2 * nm + 1) % MOD * 166_666_668 % MOD;
    let mut i = 2;
    let mut j;

    while i <= n as i64 {
        j = n as i64 / (n as i64 / i);
        ret -= (i + j) % MOD * ((j - i + 1) % MOD) % MOD * 500_000_004 % MOD
            * s(n / i as usize, m, iphi)
            % MOD;
        ret %= MOD;
        i = j + 1;
    }

    if ret < 0 {
        ret += MOD;
    }

    m.insert(n, ret);
    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut map = HashMap::new();
    let mut phi = vec![0; THRESHOLD];
    let mut iphi = vec![0; THRESHOLD];
    let mut check = vec![false; THRESHOLD];
    let mut prime_numbers = Vec::new();

    phi[1] = 1;
    iphi[1] = 1;

    for i in 2..THRESHOLD {
        if !check[i] {
            prime_numbers.push(i);
            phi[i] = (i as i64 - 1) % MOD;
        }

        for prime in prime_numbers.iter() {
            if i * prime >= THRESHOLD {
                break;
            }

            check[i * prime] = true;

            phi[i * prime] = if i % prime == 0 {
                phi[i] * *prime as i64 % MOD
            } else {
                phi[i] * phi[*prime] % MOD
            };
        }

        iphi[i] = (iphi[i - 1] + i as i64 * phi[i]) % MOD;
    }

    let mut ret = 3 * s(n, &mut map, &iphi) % MOD - 1;

    if ret < 0 {
        ret += MOD;
    }

    writeln!(out, "{ret}").unwrap();
}

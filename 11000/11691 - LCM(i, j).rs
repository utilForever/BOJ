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
const INV2: i64 = 500_000_004;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    if n == 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut phi = vec![0; n + 1];

    for i in 0..=n {
        phi[i] = i as i64;
    }

    for i in 2..=n {
        if phi[i] == i as i64 {
            let p = i as i64;
            let mut j = i;

            while j <= n {
                phi[j] = phi[j] / p * (p - 1);
                j += i;
            }
        }
    }

    let mut f = vec![0; n + 1];

    for d in 1..=n {
        let val = ((d as i64) * phi[d]) % MOD;
        let mut k = d;

        while k <= n {
            f[k] = (f[k] + val) % MOD;
            k += d;
        }
    }

    let mut h = 0;

    for j in 1..=n {
        let term = (j as i64 * ((f[j] + 1) % MOD)) % MOD;
        h = (h + term) % MOD;
    }

    h = (h * INV2) % MOD;

    let diag = n as i64 * ((n as i64 + 1) % MOD) % MOD * INV2 % MOD;
    let mut ret = h - diag;

    ret = ret.rem_euclid(MOD);

    writeln!(out, "{ret}").unwrap();
}

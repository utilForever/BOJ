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
const INV3: i64 = 332_748_118;
const INV5: i64 = 598_946_612;

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut x = vec![0; n];
    let mut y = vec![0; n];

    for i in 0..n {
        x[i] = scan.token::<i64>();
    }

    for i in 0..n {
        y[i] = scan.token::<i64>();
    }

    let mut den = 0;
    let mut num = 0;

    for i in 0..n - 1 {
        let dx = x[i + 1] - x[i];
        let a = y[i] % MOD;
        let b = y[i + 1] % MOD;
        let d: i64 = (b + MOD - a) % MOD;

        let s2 = (multiply(a, a) + multiply(a, b) + multiply(b, b)) % MOD;
        den = (den + multiply(dx % MOD, multiply(s2, INV3))) % MOD;

        let a2 = multiply(a, a);
        let a3 = multiply(a2, a);
        let a4 = multiply(a2, a2);

        let d2 = multiply(d, d);
        let d3 = multiply(d2, d);
        let d4 = multiply(d3, d);

        let s4 = (a4
            + multiply(2, multiply(a3, d))
            + multiply(2, multiply(a2, d2))
            + multiply(a, d3)
            + multiply(d4, INV5))
            % MOD;
        num = (num + multiply(dx % MOD, s4)) % MOD;
    }

    writeln!(out, "{}", multiply(num, pow(den, MOD - 2))).unwrap();
}

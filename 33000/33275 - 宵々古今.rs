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

fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = gcd_extended(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (_, x, _) = gcd_extended(a, m);
    (x % m + m) % m
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, mut k) = (scan.token::<usize>(), scan.token::<i64>());
    let s = scan.token::<String>();
    let s = s.as_bytes();

    if k == 1 {
        writeln!(out, "{}", n * (n + 1) / 2).unwrap();
        return;
    }

    let mut t = 0;

    while k % 2 == 0 {
        k /= 2;
        t += 1;
    }

    let mut cnt_zero_cons = vec![0; n + 1];

    for i in 1..=n {
        cnt_zero_cons[i] = if s[i - 1] == b'0' {
            cnt_zero_cons[i - 1] + 1
        } else {
            0
        };
    }

    if k == 1 {
        let mut zero_small = 0;

        if t > 0 {
            for i in 1..=n {
                zero_small += cnt_zero_cons[i].min(t - 1);
            }
        }

        let mut ret = 0;

        for i in 1..=n {
            if cnt_zero_cons[i] >= t {
                ret += i - t + 1;
            }
        }

        writeln!(out, "{}", zero_small + ret).unwrap();
        return;
    }

    let mut zero_small = 0;

    if t > 0 {
        for i in 1..=n {
            zero_small += cnt_zero_cons[i].min(t - 1);
        }
    }

    let inv = mod_inv(2, k);
    let mut inv_pow = vec![1; n + 1];

    for i in 1..=n {
        inv_pow[i] = (inv_pow[i - 1] * inv) % k;
    }

    let mut p = vec![0; n + 1];
    let mut a = vec![0; n + 1];

    for i in 1..=n {
        let bit = if s[i - 1] == b'0' { 0 } else { 1 };

        p[i] = ((p[i - 1] << 1) + bit) % k;
        a[i] = (p[i] * inv_pow[i]) % k;
    }

    let mut freq = vec![0; k as usize];
    let mut w = 0;
    let mut ret = 0;

    for i in 1..=n {
        let bound = (if i >= t { i - t } else { 0 }).min(i - 1);

        while w <= bound {
            let idx = if w == 0 { 0 } else { a[w] as usize };
            freq[idx] += 1;
            w += 1;
        }

        if cnt_zero_cons[i] >= t {
            let idx = a[i] as usize;
            ret += freq[idx];
        }
    }

    writeln!(out, "{}", zero_small + ret).unwrap();
}

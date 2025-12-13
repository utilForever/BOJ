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

fn mobius(mut n: i64) -> i64 {
    let mut m = 1;
    let mut p = 2;

    while p * p <= n {
        if n % p == 0 {
            n /= p;

            if n % p == 0 {
                return 0;
            }

            m = -m;
        }

        p += if p == 2 { 1 } else { 2 };
    }

    if n > 1 {
        m = -m;
    }

    m
}

// Reference: https://en.wikipedia.org/wiki/M%C3%B6bius_inversion_formula
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut prefix_mu = vec![0; 50001];

    for i in 1..=50000 {
        let mu = mobius(i as i64);
        prefix_mu[i] = prefix_mu[i - 1] + mu;
    }

    for _ in 0..n {
        let (a, b, d) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let (mut a, mut b) = (a / d, b / d);

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        let mut idx = 1;
        let mut ret = 0;

        while idx <= a {
            let next = (a / (a / idx)).min(b / (b / idx));
            let sum_mu = prefix_mu[next as usize] - prefix_mu[(idx - 1) as usize];

            ret += sum_mu * (a / idx) * (b / idx);
            idx = next + 1;
        }

        writeln!(out, "{ret}").unwrap();
    }
}

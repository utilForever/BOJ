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

fn pow_mod(n: u64, m: u64, d: u64) -> u64 {
    if n < 100 && d < 400_000_000 {
        pow_mod_inner(n, m, d)
    } else {
        pow_mod_inner(n as u128, m as u128, d as u128) as u64
    }
}

fn pow_mod_inner<T>(n: T, m: T, d: T) -> T
where
    T: Copy
        + std::cmp::PartialEq
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::convert::From<u64>,
{
    if m == 0.into() {
        if d == 1.into() {
            0.into()
        } else {
            1.into()
        }
    } else if m == 1.into() {
        n % d
    } else {
        let k = pow_mod_inner(n, m / 2.into(), d);

        if m % 2.into() == 0.into() {
            (k * k) % d
        } else {
            (k * k * n) % d
        }
    }
}

pub fn pihex(d: u64) -> String {
    let mut fraction = 0.0;

    for &(j, k) in &[
        (1, 16.0),
        (2, -16.0),
        (3, -8.0),
        (4, -16.0),
        (5, -4.0),
        (6, -4.0),
        (7, 2.0),
    ] {
        fraction += k * series_sum(d, j);
    }

    (0..4)
        .scan(fraction, |x, _| {
            *x = (*x - x.floor()) * 16.0;
            Some(format!("{:X}", x.floor() as u32))
        })
        .fold(String::new(), |s, t| s + &t)
}

fn series_sum(d: u64, j: u64) -> f64 {
    let fraction1: f64 = (0..d + 1)
        .map(|i| pow_mod(16, d - i, (8 * i + j).pow(2)) as f64 / (8 * i + j).pow(2) as f64)
        .fold(0.0, |x, y| (x + y).fract());
    let fraction2: f64 = (d + 1..)
        .map(|i| 16.0_f64.powi(-((i - d) as i32)) / ((8 * i + j).pow(2) as f64))
        .take_while(|&x| x > 1e-13_f64)
        .sum();

    fraction1 + fraction2
}

// Reference: https://github.com/itchyny/pihex-rs
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<u64>();
    let ret = pihex(n - 1);

    writeln!(out, "{}", ret.chars().nth(0).unwrap()).unwrap();
}

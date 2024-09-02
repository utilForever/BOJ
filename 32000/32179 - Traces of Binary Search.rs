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

const MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (mut n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut prev = scan.token::<usize>();
    let mut left = 1;
    let mut right = 100;
    let mut comb = vec![vec![0; 101]; 101];
    let mut ret = 1;

    for i in 0..=100 {
        for j in 0..=i {
            comb[i][j] = if j == 0 || j == i {
                1
            } else {
                (comb[i - 1][j - 1] + comb[i - 1][j]) % MOD
            };
        }
    }

    for _ in 1..k {
        let curr = scan.token::<usize>();

        if curr > prev {
            ret = (ret * comb[prev - left][n / 2 - if n % 2 == 0 { 1 } else { 0 }]) % MOD;
            n /= 2;
            left = prev + 1;
        } else if curr < prev {
            ret = (ret * comb[right - prev][n / 2]) % MOD;
            n = n / 2 - if n % 2 == 0 { 1 } else { 0 };
            right = prev - 1;
        }

        prev = curr;
    }

    ret = (ret * comb[prev - left][n / 2 - if n % 2 == 0 { 1 } else { 0 }]) % MOD;
    ret = (ret * comb[right - prev][n / 2]) % MOD;

    writeln!(out, "{ret}").unwrap();
}

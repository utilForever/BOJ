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

fn lucas_theorem(comb: &Vec<Vec<usize>>, mut n: usize, mut k: usize, p: usize) -> usize {
    let mut ret = 1;

    while n > 0 || k > 0 {
        ret = ret * comb[n % p][k % p] % p;
        n /= p;
        k /= p;
    }

    ret
}

// Reference: https://ps.mjstudio.net/lucas
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k, m) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut binomial = vec![vec![0; m]; m];

    for i in 0..m {
        for j in 0..=i {
            binomial[i][j] = if i == 0 || j == 0 {
                1
            } else {
                (binomial[i - 1][j - 1] + binomial[i - 1][j]) % m
            };
        }
    }

    writeln!(out, "{}", lucas_theorem(&binomial, n, k, m)).unwrap();
}

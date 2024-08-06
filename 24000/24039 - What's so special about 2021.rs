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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut is_prime = vec![true; 201];

    is_prime[1] = false;

    let mut idx = 2;

    while idx * idx <= 200 {
        if !is_prime[idx] {
            idx += 1;
            continue;
        }

        for j in (idx * idx..=200).step_by(idx) {
            is_prime[j] = false;
        }

        idx += 1;
    }

    let mut primes = Vec::new();

    for i in 1..=200 {
        if is_prime[i] {
            primes.push(i);
        }
    }

    for i in 0..primes.len() - 1 {
        if primes[i] * primes[i + 1] > n {
            writeln!(out, "{}", primes[i] * primes[i + 1]).unwrap();
            return;
        }
    }
}

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

    let mut n = scan.token::<usize>();

    if n == 1 || n == 2 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut is_prime = vec![true; 200_001];
    is_prime[1] = false;

    let mut i = 2;

    while i * i <= 200_000 {
        if !is_prime[i] {
            i += 1;
            continue;
        }

        for j in (i * i..=200_000).step_by(i) {
            is_prime[j] = false;
        }

        i += 1;
    }

    writeln!(out, "{}", n / 2).unwrap();

    let mut ret = Vec::with_capacity(n / 2);

    // If n is a positive integer, there exists at least one prime number greater than n and less than or equal to 2n.
    // Reference: https://en.wikipedia.org/wiki/Bertrand%27s_postulate (BOJ 4948)
    // Assume that n > 6 and n is an even number.
    // There is a prime number x such that floor(n / 3) < x < 2 * floor(n / 3).
    // Note that n / 3 = p + q where p is a integer and 0 <= q < 1.
    // Then, p < x < 2 * p -> Therefore, there is a another prime number y such that 3 * floor(n / 3) < 3 * x = y < 2 * n.
    while n > 6 {
        let mut left = 2 * (n / 3);
        let mut right = n;

        while !is_prime[left] {
            left -= 1;
        }

        left = 3 * left - n;
        n = left - 1;

        while left < right {
            ret.push((left, right));

            left += 1;
            right -= 1;
        }
    }

    if n == 3 {
        ret.push((1, 3));
    } else if n == 4 || n == 5 {
        ret.push((1, 3));
        ret.push((2, 4));
    } else if n == 6 {
        ret.push((1, 5));
        ret.push((2, 4));
        ret.push((3, 6));
    }

    for (a, b) in ret {
        writeln!(out, "{a} {b}").unwrap();
    }
}

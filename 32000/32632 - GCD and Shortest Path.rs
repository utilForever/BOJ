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

    let (k, n) = (scan.token::<i64>(), scan.token::<i64>());
    let mut cnt_coprime_k = 0;
    let mut cnt_gcd_2 = 0;

    {
        let mut primes = Vec::new();
        let mut k_clone = k;
        let mut idx = 2;

        while idx * idx <= k_clone {
            if k_clone % idx == 0 {
                primes.push(idx);

                while k_clone % idx == 0 {
                    k_clone /= idx;
                }
            }

            idx += 1;
        }

        if k_clone > 1 {
            primes.push(k_clone);
        }

        for subset in 0..(1 << primes.len()) {
            let mut mobius = 1;
            let mut multiply_of_primes = 1;

            for i in 0..primes.len() {
                if (subset >> i) & 1 == 1 {
                    mobius *= -1;
                    multiply_of_primes *= primes[i];
                }
            }

            cnt_coprime_k += mobius * (n / multiply_of_primes);
        }

        if k == 1 {
            cnt_coprime_k -= 1;
        }
    }

    {
        if k % 2 == 0 {
            let mut primes = Vec::new();
            let mut k_clone = k / 2;
            let mut idx = 2;

            while idx * idx <= k_clone {
                if k_clone % idx == 0 {
                    primes.push(idx);

                    while k_clone % idx == 0 {
                        k_clone /= idx;
                    }
                }

                idx += 1;
            }

            if k_clone > 1 {
                primes.push(k_clone);
            }

            for subset in 0..(1 << primes.len()) {
                let mut mobius = 1;
                let mut multiply_of_primes = 1;

                for i in 0..primes.len() {
                    if (subset >> i) & 1 == 1 {
                        mobius *= -1;
                        multiply_of_primes *= primes[i];
                    }
                }

                cnt_gcd_2 += mobius * (n / (2 * multiply_of_primes));
            }

            if k / 2 == 1 {
                cnt_gcd_2 -= 1;
            }
        }
    }

    writeln!(out, "{}", cnt_coprime_k + cnt_gcd_2).unwrap();
}

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

    let mut is_prime = vec![true; 318138];
    is_prime[1] = false;

    let mut i = 2;

    while i * i <= 318137 {
        if !is_prime[i] {
            i += 1;
            continue;
        }

        for j in (i * i..=318137).step_by(i) {
            is_prime[j] = false;
        }

        i += 1;
    }

    let mut super_primes = vec![0];
    let mut idx = 1;

    for i in 2..=318137 {
        if is_prime[i] {
            if is_prime[idx] {
                super_primes.push(i);
            }

            idx += 1;
        }
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        writeln!(out, "{}", super_primes[n]).unwrap();
    }
}

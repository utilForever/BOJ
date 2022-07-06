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

    let mut is_prime = vec![false; 10_000_001];
    is_prime[1] = true;

    for i in 2..=10_000_000 {
        if is_prime[i] {
            continue;
        }

        for j in (i * i..=10_000_000).step_by(i) {
            is_prime[j] = true;
        }
    }

    let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
    let mut ret = 0;

    for i in 2..=10_000_000 {
        if (i * i) as i64 > b {
            break;
        }

        if is_prime[i] {
            continue;
        }

        let mut val = (i * i) as i64;

        loop {
            if val > b {
                break;
            }

            if val >= a && val <= b {
                ret += 1;
            }

            val *= i as i64;

            if val % i as i64 != 0 {
                break;
            }
        }
    }

    writeln!(out, "{}", ret).unwrap();
}

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

    let (p, k) = (scan.token::<String>(), scan.token::<usize>());
    let mut check = vec![false; 1_000_001];
    let mut prime_numbers = Vec::new();

    for i in 2..=1_000_000 {
        if check[i] {
            continue;
        }

        prime_numbers.push(i);

        for j in (i * 2..=1_000_000).step_by(i) {
            check[j] = true;
        }
    }

    for prime in prime_numbers.iter() {
        let mut ret = 0;

        if *prime >= k {
            break;
        }

        for c in p.chars() {
            ret = (ret * 10 + (c as u8 - '0' as u8) as usize) % prime;
        }

        if ret == 0 {
            writeln!(out, "BAD {}", prime).unwrap();
            return;
        }
    }

    writeln!(out, "GOOD").unwrap();
}

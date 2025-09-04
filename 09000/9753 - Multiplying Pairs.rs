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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let mut is_prime = vec![true; 50_001];
    let mut prime_numbers = Vec::new();

    for i in 2..=50_000 {
        if is_prime[i] {
            prime_numbers.push(i);

            for j in (2 * i..=50_000).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    let mut multiplies = Vec::new();

    for i in 0..prime_numbers.len() {
        for j in i + 1..prime_numbers.len() {
            let multiply = prime_numbers[i] * prime_numbers[j];

            if multiply >= 100_000 {
                break;
            }

            multiplies.push(multiply);
        }
    }

    multiplies.push(100001);
    multiplies.sort_unstable();

    for _ in 0..t {
        let k = scan.token::<usize>();
        let pos = multiplies.partition_point(|&x| x < k);

        writeln!(out, "{}", multiplies[pos]).unwrap();
    }
}

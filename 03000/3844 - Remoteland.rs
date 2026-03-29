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

const MOD: i64 = 1_000_000_007;

fn pow(mut base: i64, mut exp: i64) -> i64 {
    let mut ret = 1;

    base %= MOD;

    while exp > 0 {
        if exp & 1 == 1 {
            ret = ret * base % MOD;
        }

        base = base * base % MOD;
        exp >>= 1;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut is_prime = vec![true; 10_000_001];
    is_prime[1] = false;

    let mut i = 2;

    while i * i <= 10_000_000 {
        if !is_prime[i] {
            i += 1;
            continue;
        }

        for j in (i * i..=10_000_000).step_by(i) {
            is_prime[j] = false;
        }

        i += 1;
    }

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut ret = 1;

        for i in 2..=n {
            if !is_prime[i] {
                continue;
            }

            let mut prime = i as i64;
            let mut cnt = 0;

            while n as i64 >= prime {
                cnt += n as i64 / prime;
                prime *= i as i64;
            }

            if cnt % 2 == 1 {
                cnt -= 1;
            }

            ret = ret * pow(i as i64, cnt) % MOD;
        }

        writeln!(out, "{ret}").unwrap();
    }
}

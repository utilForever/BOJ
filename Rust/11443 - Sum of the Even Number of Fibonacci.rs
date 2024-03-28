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

static MOD: i64 = 1_000_000_007;

fn multiply(a: Vec<Vec<i64>>, b: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut ret = vec![vec![0, 2]; 2];

    for i in 0..2 {
        for j in 0..2 {
            ret[i][j] = 0;

            for k in 0..2 {
                ret[i][j] += a[i][k] * b[k][j];
                ret[i][j] %= MOD;
            }
        }
    }

    ret
}

fn fibonacci(n: i64) -> Vec<Vec<i64>> {
    let multiplier = vec![vec![0, 1], vec![1, 1]];

    if n == 1 {
        return multiplier;
    }

    if n % 2 == 1 {
        let rest = fibonacci(n - 1);
        multiply(multiplier, rest)
    } else {
        let rest = fibonacci(n / 2);
        multiply(rest.clone(), rest)
    }
}

// Reference: https://en.wikipedia.org/wiki/Fibonacci_sequence
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut n = scan.token::<i64>();

    if n % 2 == 0 {
        n += 1;
    }

    writeln!(out, "{}", (fibonacci(n)[0][1] + MOD - 1) % MOD).unwrap();
}

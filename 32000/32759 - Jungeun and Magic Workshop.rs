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

const MOD: i32 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<i64>(), scan.token::<usize>());
    let mut first = i32::MIN;
    let mut second = i32::MIN;

    for _ in 0..m {
        let a = scan.token::<i32>();

        if a > first {
            second = first;
            first = a;
        } else if a > second {
            second = a;
        }
    }

    if first >= 0 && second >= 0 {
        for _ in 0..n {
            let sum = (first + second) % MOD;
            second = first;
            first = sum;
        }

        writeln!(out, "{first}").unwrap();
    } else if first <= 0 && second <= 0 {
        writeln!(out, "{}", (first + second + MOD) % MOD).unwrap();
    } else {
        let mut idx = 0;

        while second < 0 && idx < n - 1 {
            let sum = first + second;
            second = sum;

            idx += 1;
        }

        if idx == n - 1 {
            writeln!(out, "{}", (first + second + MOD) % MOD).unwrap();
            return;
        }

        while idx < n {
            let sum = (first + second) % MOD;
            second = first;
            first = sum;

            idx += 1;
        }

        writeln!(out, "{first}").unwrap();
    }
}

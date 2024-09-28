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

    let mut p = vec![0.0; 11];
    let mut q = vec![0.0; 11];
    let mut dp = vec![0.0; 100_001];

    p[0] = 1.0;

    for i in 1..=10 {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
        p[i] = a as f64 / (a + b) as f64;
        q[i] = b as f64 / (a + b) as f64;
    }

    let (n, mut number) = (
        scan.token::<usize>(),
        scan.token::<String>().chars().collect::<Vec<_>>(),
    );
    number.insert(0, '!');

    for i in 1..=n {
        if number[i - 1] == number[i] {
            dp[i] = dp[i - 1] * p[number[i] as usize - b'0' as usize]
                + dp[i - 2] * q[number[i] as usize - b'0' as usize]
                + 1.0;
        } else {
            dp[i] = dp[i - 1] + p[number[i] as usize - b'0' as usize];
            dp[i] += if p[10] == 0.0 {
                3.0 * q[number[i] as usize - b'0' as usize]
            } else if q[10] == 0.0 {
                2.0 * q[number[i] as usize - b'0' as usize]
            } else {
                (1.0 + (p[10]
                    + q[10] * (2.0 + q[number[i] as usize - b'0' as usize] * p[10])
                        / (1.0 - q[10] * q[number[i] as usize - b'0' as usize]))
                    .min(2.0 * q[10] + 2.0 * (1.0 + q[10]) * p[10] / q[10]))
                    * q[number[i] as usize - b'0' as usize]
            }
        }
    }

    writeln!(out, "{:.9}", dp[n]).unwrap();
}

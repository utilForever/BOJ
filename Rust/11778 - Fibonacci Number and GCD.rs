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

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        let val = max;

        max = min;
        min = val;
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn multiply_matrix(first: [[i64; 2]; 2], second: [[i64; 2]; 2]) -> [[i64; 2]; 2] {
    let mut result = [[0; 2]; 2];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                result[i][j] = (result[i][j] + first[i][k] * second[k][j]) % 1_000_000_007;
            }
        }
    }

    result
}

fn calculate_fibonacci(n: i64) -> [[i64; 2]; 2] {
    if n == 1 {
        return [[1, 1], [1, 0]];
    }

    if n % 2 == 0 {
        let ret = calculate_fibonacci(n / 2);
        return multiply_matrix(ret, ret);
    } else {
        let ret = calculate_fibonacci(n - 1);
        return multiply_matrix(ret, [[1, 1], [1, 0]]);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let ret = gcd(n as i64, m as i64);

    writeln!(out, "{}", calculate_fibonacci(ret)[0][1]).unwrap();
}

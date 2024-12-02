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
        std::mem::swap(&mut min, &mut max);
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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, mut d) = (scan.token::<i64>(), scan.token::<i64>());
    let is_negative = n < 0;
    let mut n = n.abs();

    let g = gcd(n, d);
    n /= g;
    d /= g;

    // Check if d is a power of 2
    if d & (d - 1) != 0 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut ret = String::new();

    // If n < d, add D until d = 1
    while d > 1 {
        ret.push('D');
        d /= 2;
    }

    while n > 0 {
        // If nth bit is 1, add L (negative) or R (positive)
        if n % 2 == 1 {
            ret.push(if is_negative { 'L' } else { 'R' });
        }

        // Add U for next bit
        ret.push('U');
        n /= 2;
    }

    writeln!(out, "{}", ret.len()).unwrap();
    writeln!(out, "{ret}").unwrap();
}

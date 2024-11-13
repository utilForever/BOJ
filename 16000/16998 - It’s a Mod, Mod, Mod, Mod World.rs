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

fn floor_sum_ap(a: i64, b: i64, c: i64, n: i64) -> i64 {
    if a == 0 {
        return (b / c) * (n + 1);
    }

    if a >= c || b >= c {
        return ((n * (n + 1)) / 2) * (a / c)
            + (n + 1) * (b / c)
            + floor_sum_ap(a % c, b % c, c, n);
    }

    let m = (a * n + b) / c;

    m * n - floor_sum_ap(c, c - b - 1, a, m - 1)
}

// Reference: https://asfjwd.github.io/2020-04-24-floor-sum-ap/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    // NOTE: a mod b = a - b * floor(a / b)
    //    => sigma(i = 1 to n) ((p * i) mod q)
    //     = sigma(i = 1 to n) ((p * i) - q * floor((p * i) / q))
    //     = p * sigma(i = 1 to n) (i) - q * sigma(i = 1 to n) floor((p * i) / q)
    //     = p * n * (n + 1) / 2 - q * sigma(i = 1 to n) floor((p * i) / q)  
    for _ in 0..t {
        let (p, q, n) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let g = gcd(p, q);

        writeln!(
            out,
            "{}",
            p * n * (n + 1) / 2 - q * floor_sum_ap(p / g, 0, q / g, n)
        )
        .unwrap();
    }
}

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

    let n = scan.token::<usize>();
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut c = vec![0; n];
    let mut d = vec![0; n];

    for i in 0..n {
        (a[i], b[i], c[i], d[i]) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    // Observation
    // a + b + c + d = 0 -> a + b = -(c + d)
    // First, calculate all possible sum of a + b and c + d
    // Then, sort both arrays
    // For each element in a + b, find the number of elements in c + d
    // that satisfy the condition using lower_bound and upper_bound

    let mut sum_ab = vec![0; n * n];
    let mut sum_cd = vec![0; n * n];

    for i in 0..n {
        for j in 0..n {
            sum_ab[i * n + j] = a[i] + b[j];
            sum_cd[i * n + j] = c[i] + d[j];
        }
    }

    sum_ab.sort();
    sum_cd.sort();

    let mut left = 0 as i64;
    let mut right = (n * n - 1) as i64;
    let limit_left = (n * n) as i64;
    let limit_right = -1;

    let mut ret = 0;

    while left < limit_left && right > limit_right {
        let sum = sum_ab[left as usize] + sum_cd[right as usize];

        if sum == 0 {
            let mut l = 1;
            let mut r = 1;

            while left + l < limit_left && sum_ab[left as usize] == sum_ab[(left + l) as usize] {
                l += 1;
            }

            while right - r > limit_right && sum_cd[right as usize] == sum_cd[(right - r) as usize]
            {
                r += 1;
            }

            ret += l * r;
            left += l;
            right -= r;
        } else if sum < 0 {
            left += 1;
        } else {
            right -= 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}

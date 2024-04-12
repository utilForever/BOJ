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

    if n == 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut primes = vec![true; n + 1];
    let limit = ((n + 1) as f64).sqrt() as usize;

    for i in 2..=limit {
        if !primes[i] {
            continue;
        }

        for j in (i * i..=n).step_by(i) {
            primes[j] = false;
        }
    }

    let mut nums = Vec::with_capacity(limit);

    for i in 2..=n {
        if primes[i] {
            nums.push(i);
        }
    }

    let mut left = 0;
    let mut right = 0;
    let mut sum = nums[0];
    let mut ret = 0;

    while left <= right {
        if sum == n {
            ret += 1;
            left += 1;
            right += 1;

            if right >= nums.len() {
                break;
            }

            sum -= nums[left - 1];
            sum += nums[right];
        } else if sum > n {
            left += 1;
            sum -= nums[left - 1];
        } else if sum < n {
            right += 1;

            if right >= nums.len() {
                break;
            }

            sum += nums[right];
        }
    }

    writeln!(out, "{ret}").unwrap();
}

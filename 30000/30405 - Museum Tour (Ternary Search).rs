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

fn calculate_dist_total(plans: &Vec<(i64, i64)>, position: i64) -> i64 {
    let mut ret = 0;

    for (start, end) in plans {
        ret += (start - position).abs() + (end - position).abs();
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut plans = vec![(0, 0); n];

    for i in 0..n {
        let k = scan.token::<i64>();
        let mut start = 0;
        let mut end = 0;

        for j in 0..k {
            if j == 0 {
                start = scan.token::<i64>();
            } else if j == k - 1 {
                end = scan.token::<i64>();
            } else {
                _ = scan.token::<i64>();
            }
        }

        plans[i] = (start, end);
    }

    let mut left = 1;
    let mut right = m;

    while left + 3 <= right {
        let p1 = (left * 2 + right) / 3;
        let p2 = (left + right * 2) / 3;

        let p1_dist = calculate_dist_total(&plans, p1);
        let p2_dist = calculate_dist_total(&plans, p2);

        if p1_dist <= p2_dist {
            right = p2;
        } else {
            left = p1;
        }
    }

    let mut dist_min = i64::MAX;
    let mut ret = 0;

    for i in left..=right {
        let dist = calculate_dist_total(&plans, i);

        if dist < dist_min {
            dist_min = dist;
            ret = i;
        }
    }

    writeln!(out, "{ret}").unwrap();
}

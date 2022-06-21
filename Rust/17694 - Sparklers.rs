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

fn is_possible(dists: &Vec<i64>, n: usize, k: usize, t: usize, speed: i64) -> bool {
    let mut interval = vec![0; n + 1];

    for i in 1..=n {
        interval[i] = dists[i] - 2 * speed * t as i64 * i as i64;
    }

    let mut left = 1;
    let mut right = n;

    for i in 1..=k {
        left = if interval[i] > interval[left] { i } else { left };
    }

    for i in k..=n {
        right = if interval[i] < interval[right] { i } else { right };
    }

    let mut query_left = k;
    let mut query_right = k;
    let mut value_left = interval[query_left];
    let mut value_right = interval[query_right];

    while query_left >= left || query_right <= right {
        let temp_left = query_left;
        let temp_right = query_right;

        while query_right <= right && value_left >= interval[query_right] {
            value_right = value_right.min(interval[query_right]);
            query_right += 1;
        }

        while query_left >= left && value_right <= interval[query_left] {
            value_left = value_left.max(interval[query_left]);
            query_left -= 1;
        }

        if temp_left == query_left && temp_right == query_right {
            return false;
        }
    }

    query_left = 1;
    query_right = n;
    value_left = interval[query_left];
    value_right = interval[query_right];

    while query_left <= left || query_right >= right {
        let temp_left = query_left;
        let temp_right = query_right;

        while query_right >= right && value_left >= interval[query_right] {
            value_right = value_right.min(interval[query_right]);
            query_right -= 1;
        }

        while query_left <= left && value_right <= interval[query_left] {
            value_left = value_left.max(interval[query_left]);
            query_left += 1;
        }

        if temp_left == query_left && temp_right == query_right {
            return false;
        }
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k, t) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut dists = vec![0; n + 1];

    for i in 1..=n {
        dists[i] = scan.token::<i64>();
    }

    let mut left = -1;
    let mut right = 1_000_000_007;

    while left + 1 < right {
        let mid = (left + right) / 2;

        if is_possible(&dists, n, k, t, mid) {
            right = mid;
        } else {
            left = mid;
        }
    }

    writeln!(out, "{}", right).unwrap();
}

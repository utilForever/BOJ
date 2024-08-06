use io::Write;
use std::{cmp::Ordering, io, str};
use Ordering::Less;

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

pub trait Ext {
    type Item;

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    fn lower_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;
}

impl<T> Ext for [T] {
    type Item = T;
    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.lower_bound_by(|y| y.cmp(x))
    }
    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0;
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Less { mid } else { base };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp == Less) as usize
    }
}

// Reference: https://www.secmem.org/blog/2021/07/19/distinct-value-query/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut original_arr: Vec<usize> = vec![0; 1_000_001];

    for i in 1..=n {
        original_arr[i] = scan.token();
    }

    let mut compressed_arr = original_arr.clone();
    compressed_arr.sort();
    compressed_arr.dedup();

    let mut arr = vec![0; 1_000_001];
    for i in 1..=n {
        arr[i] = compressed_arr.lower_bound(&original_arr[i]);
    }

    let mut prev = vec![0; 1_000_001];
    let mut next = vec![n + 1; 1_000_001];
    let mut last = vec![0; 1_000_001];

    let bucket = 512;
    let num_bucket = 1954;
    let mut prefix_sum = vec![vec![0; num_bucket + 1]; num_bucket + 1];

    for i in 1..=n {
        prev[i] = last[arr[i]];
        next[last[arr[i]]] = i;
        last[arr[i]] = i;

        if prev[i] != 0 {
            prefix_sum[(i - 1) / bucket + 1][(prev[i] - 1) / bucket + 1] += 1;
        }
    }

    for i in 1..=num_bucket {
        for j in 1..=num_bucket {
            prefix_sum[i][j] +=
                prefix_sum[i - 1][j] + prefix_sum[i][j - 1] - prefix_sum[i - 1][j - 1];
        }
    }

    let q = scan.token::<usize>();
    let mut ans_q = 0;

    for _ in 0..q {
        let (x, r) = (scan.token::<i32>(), scan.token::<usize>());
        let l = (x + ans_q) as usize;

        let mut left = (l - 1 + bucket - 1) / bucket * bucket + 1;
        let mut right = r / bucket * bucket;
        let mut ret = 0;

        let sqrt_left = (left - 1) / bucket + 1;
        let sqrt_right = (right - 1 + bucket) / bucket;

        if sqrt_left <= sqrt_right {
            ret = (right - left + 1)
                - (prefix_sum[sqrt_right][sqrt_right]
                    - prefix_sum[sqrt_right][sqrt_left - 1]
                    - prefix_sum[sqrt_left - 1][sqrt_right]
                    + prefix_sum[sqrt_left - 1][sqrt_left - 1]);
        } else {
            left = l;
            right = l - 1;
        }

        while l < left {
            left -= 1;

            if right < next[left] {
                ret += 1;
            }
        }

        while r > right {
            right += 1;

            if left > prev[right] {
                ret += 1;
            }
        }

        ans_q = ret as i32;

        writeln!(out, "{}", ret).unwrap();
    }
}

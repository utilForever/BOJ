use io::Write;
use std::{cmp::Ordering, io, str};
use Ordering::{Greater, Less};

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

#[derive(Debug)]
struct Bucket {
    start: usize,
    end: usize,
    lazy: usize,
    sum: usize,
    vals: Vec<usize>,
    vals_sorted: Vec<usize>,
}

impl Bucket {
    fn new(vals: Vec<usize>, start: usize, end: usize) -> Self {
        let sum = vals.iter().sum();
        let mut vals_sorted = vals.clone();

        vals_sorted.sort_unstable();

        Self {
            start,
            end,
            lazy: 0,
            sum,
            vals,
            vals_sorted,
        }
    }

    fn propagate(&mut self, k: usize) {
        if self.lazy != 0 {
            for val in self.vals.iter_mut() {
                *val = (*val + self.lazy) % k;
            }

            self.lazy = 0;
            self.sum = self.vals.iter().sum();
            self.vals_sorted = self.vals.clone();

            self.vals_sorted.sort_unstable();
        }
    }

    fn update_full(&mut self, k: usize) {
        let target = if k - 1 >= self.lazy {
            k - 1 - self.lazy
        } else {
            0
        };
        let count = count_equal(&self.vals_sorted, target);

        self.sum = self.sum + self.vals.len() - count * k;
        self.lazy = (self.lazy + 1) % k;
    }

    fn update_partial(&mut self, left: usize, right: usize, k: usize) {
        self.propagate(k);

        for i in left..=right {
            let val = self.vals[i] + 1;
            self.vals[i] = if val < k { val } else { val - k };
        }

        self.sum = self.vals.iter().sum();
        self.vals_sorted = self.vals.clone();

        self.vals_sorted.sort_unstable();
    }

    fn query(&mut self, left: usize, right: usize, k: usize) -> usize {
        let mut ret = 0;

        for i in left..=right {
            let val = self.vals[i] + self.lazy;
            ret += if val >= k { val - k } else { val };
        }

        ret
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

    fn upper_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    fn upper_bound_by<'a, F>(&'a self, f: F) -> usize
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

    fn upper_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.upper_bound_by(|y| y.cmp(x))
    }

    fn upper_bound_by<'a, F>(&'a self, mut f: F) -> usize
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
            base = if cmp == Greater { base } else { mid };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp != Greater) as usize
    }
}

fn count_equal(sorted: &[usize], target: usize) -> usize {
    let lb = sorted.lower_bound(&target);
    let ub = sorted.upper_bound(&target);

    if lb < sorted.len() && sorted[lb] == target {
        ub - lb
    } else {
        0
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<usize>() % k;
    }

    let bucket_size = (n as f64).sqrt() as usize;
    let mut buckets = Vec::new();
    let mut idx = 0;

    while idx < n {
        let right = if idx + bucket_size < n {
            idx + bucket_size
        } else {
            n
        };
        let vals = nums[idx..right].to_vec();

        buckets.push(Bucket::new(vals, idx, right));
        idx = right;
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (command, s, e) = (
            scan.token::<i64>(),
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
        );

        if command == 1 {
            for bucket in buckets.iter_mut() {
                if bucket.end <= s || bucket.start > e {
                    continue;
                }

                if s <= bucket.start && e >= bucket.end - 1 {
                    bucket.update_full(k);
                } else {
                    let left = if s > bucket.start {
                        s - bucket.start
                    } else {
                        0
                    };
                    let right = if e < bucket.end - 1 {
                        e - bucket.start
                    } else {
                        bucket.vals.len() - 1
                    };

                    bucket.update_partial(left, right, k);
                }
            }
        } else {
            let mut ret = 0;

            for bucket in buckets.iter_mut() {
                if bucket.end <= s || bucket.start > e {
                    continue;
                }

                if s <= bucket.start && e >= bucket.end - 1 {
                    ret += bucket.sum;
                } else {
                    let left = if s > bucket.start {
                        s - bucket.start
                    } else {
                        0
                    };
                    let right = if e < bucket.end - 1 {
                        e - bucket.start
                    } else {
                        bucket.vals.len() - 1
                    };

                    ret += bucket.query(left, right, k);
                }
            }

            writeln!(out, "{ret}").unwrap();
        }
    }
}

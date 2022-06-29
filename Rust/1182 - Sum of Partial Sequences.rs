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

fn make_left(
    left: &mut Vec<i64>,
    nums: &Vec<i64>,
    ret: &mut usize,
    n: usize,
    s: i64,
    idx: usize,
    mut sum: i64,
) {
    if idx >= n / 2 {
        return;
    }
    sum += nums[idx];

    if sum == s {
        *ret += 1;
    }

    left.push(sum);

    make_left(left, nums, ret, n, s, idx + 1, sum - nums[idx]);
    make_left(left, nums, ret, n, s, idx + 1, sum);
}

fn make_right(
    right: &mut Vec<i64>,
    nums: &Vec<i64>,
    ret: &mut usize,
    n: usize,
    s: i64,
    idx: usize,
    mut sum: i64,
) {
    if idx >= n {
        return;
    }

    sum += nums[idx];

    if sum == s {
        *ret += 1;
    }

    right.push(sum);

    make_right(right, nums, ret, n, s, idx + 1, sum - nums[idx]);
    make_right(right, nums, ret, n, s, idx + 1, sum);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, s) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut ret = 0;

    make_left(&mut left, &nums, &mut ret, n, s, 0, 0);
    make_right(&mut right, &nums, &mut ret, n, s, n / 2, 0);

    left.sort();
    right.sort();

    for i in 0..left.len() {
        let sum = s - left[i];
        let lower = right.lower_bound(&sum);
        let upper = right.upper_bound(&sum);

        ret += upper - lower;
    }

    writeln!(out, "{}", ret).unwrap();
}

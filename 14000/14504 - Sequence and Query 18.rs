use io::Write;
use std::{
    cmp::Ordering::{self, Greater, Less},
    io, str,
};

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n + 1];

    for i in 1..=n {
        nums[i] = scan.token::<i64>();
    }

    let cnt_buckets = (n as f64).sqrt() as usize;
    let mut buckets = vec![Vec::new(); 317];

    for i in 1..=n {
        buckets[i / cnt_buckets].push(nums[i]);
    }

    for i in 0..=n / cnt_buckets {
        buckets[i].sort();
    }

    let m = scan.token::<i64>();

    for _ in 0..m {
        let command = scan.token::<i64>();

        if command == 1 {
            let (i, j, k) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            let mut ret = 0;

            if i / cnt_buckets == j / cnt_buckets {
                for l in i..=j {
                    if nums[l] > k {
                        ret += 1;
                    }
                }
            } else {
                for l in i..cnt_buckets * (i / cnt_buckets + 1) {
                    if nums[l] > k {
                        ret += 1;
                    }
                }

                for l in cnt_buckets * (j / cnt_buckets)..=j {
                    if nums[l] > k {
                        ret += 1;
                    }
                }

                for l in i / cnt_buckets + 1..j / cnt_buckets {
                    ret += buckets[l].len() - buckets[l].upper_bound(&k);
                }
            }

            writeln!(out, "{ret}").unwrap();
        } else {
            let (i, k) = (scan.token::<usize>(), scan.token::<i64>());

            for j in 0..buckets[i / cnt_buckets].len() {
                if buckets[i / cnt_buckets][j] == nums[i] {
                    buckets[i / cnt_buckets][j] = k;
                    break;
                }
            }

            nums[i] = k;
            buckets[i / cnt_buckets].sort();
        }
    }
}

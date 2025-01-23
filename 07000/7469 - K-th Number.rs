use io::Write;
use std::{cmp::Ordering, io, str};
use Ordering::Greater;

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

    fn upper_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    fn upper_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;
}

impl<T> Ext for [T] {
    type Item = T;
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

struct MergeSortTree {
    size: usize,
    data: Vec<Vec<i64>>,
}

impl MergeSortTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![Vec::new(); real_n * 4],
        }
    }

    pub fn update(&mut self, idx: usize, val: i64) {
        self.update_internal(idx, val, 1, 1, self.size);
    }

    fn update_internal(
        &mut self,
        idx: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if idx < node_start || idx > node_end {
            return;
        }

        self.data[node].push(val);

        if node_start == node_end {
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(idx, val, node * 2, node_start, mid);
        self.update_internal(idx, val, node * 2 + 1, mid + 1, node_end);
    }

    fn query(&mut self, start: usize, end: usize, idx: i64) -> i64 {
        self.query_internal(start, end, 1, 1, self.size, idx)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
        idx: i64,
    ) -> i64 {
        if end < node_start || node_end < start {
            return 0;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].upper_bound(&idx) as i64;
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid, idx);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end, idx);

        left + right
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut tree = MergeSortTree::new(n + 1);

    for i in 1..=n {
        let num = scan.token::<i64>();
        tree.update(i, num);
    }

    for i in 1..=4 * n {
        tree.data[i].sort_unstable();
    }

    for _ in 0..q {
        let (i, j, k) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        let mut left = -1_000_000_000;
        let mut right = 1_000_000_000;

        while left <= right {
            let mid = (left + right) / 2;
            let cnt = tree.query(i, j, mid);

            if cnt < k {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        writeln!(out, "{left}").unwrap();
    }
}

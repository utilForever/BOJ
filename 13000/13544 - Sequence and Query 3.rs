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

fn update(tree: &mut Vec<Vec<i32>>, node: i32, start: i32, end: i32, i: i32, value: i32) {
    if i < start || i > end {
        return;
    }

    tree[node as usize].push(value);

    if start != end {
        update(tree, 2 * node, start, (start + end) / 2, i, value);
        update(tree, 2 * node + 1, (start + end) / 2 + 1, end, i, value);
    }
}

fn query(
    tree: &mut Vec<Vec<i32>>,
    node: i32,
    value: i32,
    start: i32,
    end: i32,
    i: i32,
    j: i32,
) -> i32 {
    if i > end || j < start {
        return 0;
    }

    if i <= start && j >= end {
        return (tree[node as usize].len() - tree[node as usize].upper_bound(&value)) as i32;
    }

    let mid = (start + end) / 2;

    let left = query(tree, 2 * node, value, start, mid, i, j);
    let right = query(tree, 2 * node + 1, value, mid + 1, end, i, j);

    left + right
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token();
    let mut tree = vec![Vec::new(); 4 * 100_001];

    for i in 1..=n {
        let a = scan.token();
        update(&mut tree, 1, 1, n, i, a);
    }

    for i in 1..(4 * 100_001) {
        tree[i as usize].sort();
    }

    let m = scan.token();
    let mut last_ans = 0;

    for _ in 0..m {
        let (a, b, c): (i32, i32, i32) = (scan.token(), scan.token(), scan.token());
        let ans = query(&mut tree, 1, c ^ last_ans, 1, n, a ^ last_ans, b ^ last_ans);

        last_ans = ans;
        writeln!(out, "{}", ans).unwrap();
    }
}

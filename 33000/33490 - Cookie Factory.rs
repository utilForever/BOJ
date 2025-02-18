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

const NEG_INF: i64 = -1_000_000_000_000_000_000;

#[derive(Debug, Default, Clone, Copy)]
struct Node {
    val: i64,
    base: i64,
}

impl Node {
    fn new(val: i64, day: i64) -> Self {
        if val == 0 {
            Self {
                val: 0,
                base: NEG_INF,
            }
        } else {
            Self {
                val,
                base: day + val - 1,
            }
        }
    }

    fn merge(&mut self, other: &Self) -> Node {
        Node {
            val: self.val + other.val,
            base: (self.base + other.val).max(other.base),
        }
    }
}

struct SegmentTree {
    size: usize,
    data: Vec<Node>,
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        let mut n_real = 1;

        while n_real < n {
            n_real *= 2;
        }

        Self {
            size: n_real,
            data: vec![Node::default(); n_real * 2],
        }
    }

    pub fn update(&mut self, idx: usize, val: Node) {
        let mut idx = idx + self.size;

        self.data[idx] = val;

        while idx > 1 {
            idx /= 2;

            let mut left = self.data[idx * 2];
            let right = self.data[idx * 2 + 1];

            self.data[idx] = left.merge(&right);
        }
    }

    pub fn query(&mut self) -> Node {
        self.data[1]
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
    let mut companies = vec![(0, 0); n];
    let mut days = Vec::new();

    for i in 0..n {
        let (c, s) = (scan.token::<i64>(), scan.token::<i64>());

        companies[i] = (s, c);
        days.push(s);
    }

    let q = scan.token::<usize>();
    let mut queries = vec![(0, 0, 0); q];

    for i in 0..q {
        let (idx, c, s) = (
            scan.token::<usize>() - 1,
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        queries[i] = (idx, c, s);
        days.push(s);
    }

    days.sort();
    days.dedup();

    let mut aggregate = vec![0; days.len()];

    for &(s, c) in companies.iter() {
        let idx = days.lower_bound(&s);
        aggregate[idx] += c;
    }

    let mut tree = SegmentTree::new(days.len());

    for i in 0..days.len() {
        let node = Node::new(aggregate[i], days[i]);
        tree.update(i, node);
    }

    let ret = tree.query();

    writeln!(out, "{}", ret.val.max(ret.base)).unwrap();

    for (idx, c_new, s_new) in queries {
        let (s_old, c_old) = companies[idx];

        let idx_old = days.lower_bound(&s_old);
        aggregate[idx_old] -= c_old;
        tree.update(idx_old, Node::new(aggregate[idx_old], days[idx_old]));

        companies[idx] = (s_new, c_new);

        let idx_new = days.lower_bound(&s_new);
        aggregate[idx_new] += c_new;
        tree.update(idx_new, Node::new(aggregate[idx_new], days[idx_new]));

        let ret = tree.query();

        writeln!(out, "{}", ret.val.max(ret.base)).unwrap();
    }
}

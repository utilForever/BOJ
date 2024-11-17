use io::Write;
use std::{
    cmp::Ordering::{self, Less},
    collections::BTreeSet,
    io, str, vec,
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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
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

struct Set {
    data: BTreeSet<(i64, i64)>,
}

impl Set {
    fn new() -> Self {
        Self {
            data: BTreeSet::new(),
        }
    }

    fn compare(&self, b: &(i64, i64)) -> Ordering {
        let mut iter = self.data.range(..b).rev();

        if let Some(a) = iter.next() {
            if a.1 < b.1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Greater
        }
    }

    fn add(&mut self, val: &(i64, i64)) {
        self.data.insert(*val);

        let mut to_remove = vec![];

        for elem in self.data.range(val..).skip(1) {
            if val.1 < elem.1 {
                to_remove.push(*elem);
            } else {
                break;
            }
        }

        for elem in to_remove {
            self.data.remove(&elem);
        }
    }
}

// Reference: https://cgiosy.github.io/posts/lis
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (m, n) = (scan.token::<usize>(), scan.token::<usize>());

    if m == 2 {
        let mut matrix = vec![(0, 0); n];

        for i in 0..n {
            matrix[i].0 = scan.token::<i64>();
        }

        for i in 0..n {
            matrix[i].1 = scan.token::<i64>();
        }

        matrix.sort();

        let mut ret = Vec::with_capacity(n);

        for (_, y) in matrix {
            let pos = ret.lower_bound(&y);

            if pos == ret.len() {
                ret.push(y);
            } else {
                ret[pos] = y;
            }
        }

        writeln!(out, "{}", ret.len()).unwrap();
    } else {
        let mut matrix = vec![(0, (0, 0)); n];

        for i in 0..n {
            matrix[i].0 = scan.token::<i64>();
        }

        for i in 0..n {
            matrix[i].1 .0 = scan.token::<i64>();
        }

        for i in 0..n {
            matrix[i].1 .1 = scan.token::<i64>();
        }

        matrix.sort();

        let mut ret: Vec<Set> = Vec::with_capacity(n);

        for (_, y) in matrix {
            let idx = ret
                .binary_search_by(|set| set.compare(&y))
                .unwrap_or_else(|i| i);

            if idx == ret.len() {
                let mut set = Set::new();
                set.add(&y);

                ret.push(set);
            } else {
                ret[idx].add(&y);
            }
        }

        writeln!(out, "{}", ret.len()).unwrap();
    }
}

use io::Write;
use std::{cmp::Ordering, collections::HashMap, io, str};
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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token();
    let mut wires = vec![(0, 0); n];
    let mut sorted_wires = vec![0; n];
    let mut map = HashMap::new();

    for i in 0..n {
        let (a, b) = (scan.token(), scan.token());
        wires[i] = (a, b);
        map.insert(b, a);
    }

    wires.sort();

    for i in 0..n {
        sorted_wires[i] = wires[i].1;
    }

    let mut binary_search = Vec::new();
    let mut indexes = Vec::new();

    for i in 0..n {
        if i == 0 {
            binary_search.push(sorted_wires[i]);
            indexes.push(1);
            continue;
        }

        let idx = binary_search.lower_bound(&sorted_wires[i]);
        let len = binary_search.len();

        if idx == len {
            binary_search.push(sorted_wires[i]);
        } else {
            binary_search[idx] = sorted_wires[i];
        }

        indexes.push(idx + 1);
    }

    let len = binary_search.len();
    let mut ans = vec![0; len];
    let mut idx = len;

    for i in (0..n).rev() {
        if indexes[i] == idx {
            idx -= 1;
            ans[idx] = sorted_wires[i];
        }
    }

    for i in 0..len {
        map.insert(ans[i], -1);
    }

    writeln!(out, "{}", n - len).unwrap();

    for i in 0..n {
        let key = sorted_wires[i];
        if map.get(&key).unwrap() == &-1 {
            continue;
        }

        writeln!(out, "{}", map.get(&key).unwrap()).unwrap();
    }
}

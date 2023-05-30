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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (_, q) = (scan.token::<usize>(), scan.token::<i64>());
    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();

    let mut red = Vec::new();
    let mut blue = Vec::new();

    for (i, c) in s.iter().enumerate() {
        if *c == 'R' {
            red.push(i);
        } else if *c == 'B' {
            blue.push(i);
        }
    }

    for _ in 0..q {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());

        let a = if red.lower_bound(&l) < red.len() {
            red[red.lower_bound(&l)]
        } else {
            r + 1
        };
        let b = if red.lower_bound(&(a + 1)) < red.len() {
            red[red.lower_bound(&(a + 1))]
        } else {
            r + 1
        };
        let c = if blue.lower_bound(&(b + 1)) < blue.len() {
            blue[blue.lower_bound(&(b + 1))]
        } else {
            r + 1
        };
        let d = if blue.lower_bound(&(c + 1)) < blue.len() {
            blue[blue.lower_bound(&(c + 1))]
        } else {
            r + 1
        };

        if d > r {
            writeln!(out, "-1").unwrap();
        } else {
            writeln!(out, "{a} {b} {c} {d}").unwrap();
        }
    }
}

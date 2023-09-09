use io::Write;
use std::{
    cmp::Ordering::{self, Less},
    collections::BTreeMap,
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

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut map: BTreeMap<i64, i64> = BTreeMap::new();

    for _ in 0..n {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());

        if let Some(x) = map.get_mut(&a) {
            *x = (*x).max(b);
        } else {
            map.insert(a, b);
        }
    }

    if map.contains_key(&1) && map[&1] <= 0 {
        map.remove(&1);
    }

    let calculate = |x: (i64, i64), y: (i64, i64)| -> i64 {
        let (a, b) = x;
        let (c, d) = y;

        (d - b) / (a - c) + ((b - d) % (c - a) > 0) as i64
    };

    let mut container: Vec<(i64, (i64, i64))> = Vec::new();

    for (a, b) in map {
        while !container.is_empty()
            && calculate((*container.last().unwrap()).1, (a, b)) <= container.last().unwrap().0
        {
            container.pop();
        }

        if container.is_empty() {
            container.push((if a > 1 { -b / (a - 1) + 1 } else { 1 }, (a, b)));
        } else {
            container.push((calculate((*container.last().unwrap()).1, (a, b)), (a, b)));
        }
    }

    for _ in 0..q {
        let (mut x, y) = (scan.token::<i64>(), scan.token::<i64>());
        let mut ret = 0;

        while x < y {
            let mut index = container.lower_bound(&(x + 1, (0, 0)));

            if index == 0 {
                break;
            }

            let x_next = if index == container.len() {
                y
            } else {
                y.min(container[index].0)
            };

            index -= 1;

            if container[index].1 .0 == 1 {
                let cnt = (x_next - x) / container[index].1 .1
                    + ((x_next - x) % container[index].1 .1 > 0) as i64;
                x += cnt * container[index].1 .1;
                ret += cnt;
            } else {
                while x < x_next {
                    x *= container[index].1 .0;
                    x += container[index].1 .1;
                    ret += 1;
                }
            }
        }

        writeln!(out, "{}", if x < y { -1 } else { ret }).unwrap();
    }
}

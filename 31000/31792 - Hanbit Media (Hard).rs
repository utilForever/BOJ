use io::Write;
use std::{
    borrow::Borrow,
    collections::{
        btree_map::{self},
        BTreeMap,
    },
    io,
    ops::{Bound, RangeBounds},
    str,
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

#[derive(Debug, PartialEq, Clone)]
pub struct MultiSet<T> {
    size: usize,
    btree_map: BTreeMap<T, usize>,
}

impl<T: Ord> Default for MultiSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> From<Vec<T>> for MultiSet<T> {
    fn from(value: Vec<T>) -> Self {
        let size = value.len();

        let mut btree_map = BTreeMap::default();
        for key in value {
            *btree_map.entry(key).or_insert(0) += 1;
        }

        Self { size, btree_map }
    }
}

impl<T: Ord> MultiSet<T> {
    pub fn clear(&mut self) {
        self.size = 0;
        self.btree_map.clear();
    }

    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.btree_map.contains_key(value)
    }

    pub fn insert(&mut self, value: T) {
        self.size += 1;
        *self.btree_map.entry(value).or_insert(0) += 1;
    }

    pub fn first(&self) -> Option<&T> {
        if let Some((key, _)) = self.btree_map.iter().next() {
            Some(key)
        } else {
            None
        }
    }

    pub fn last(&self) -> Option<&T> {
        if let Some((key, _)) = self.btree_map.iter().next_back() {
            Some(key)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.btree_map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.btree_map.len()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn marge(&mut self, other: &mut MultiSet<T>)
    where
        T: Clone,
    {
        self.size += other.size;

        for (key, val) in other.btree_map.iter() {
            if let Some(prev) = self.btree_map.get_mut(key) {
                *prev += *val;
            } else {
                self.btree_map.insert(key.clone(), *val);
            }
        }
    }

    pub fn new() -> Self {
        Self {
            size: 0,
            btree_map: BTreeMap::new(),
        }
    }

    pub fn pop_first(&mut self) -> Option<T>
    where
        T: Clone,
    {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;

            let first = self.first().unwrap().clone();
            self.remove(&first);
            Some(first)
        }
    }

    pub fn pop_last(&mut self) -> Option<T>
    where
        T: Clone,
    {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;

            let last = self.last().unwrap().clone();
            self.remove(&last);
            Some(last)
        }
    }

    pub fn remove(&mut self, value: &T) -> bool
    where
        T: Clone,
    {
        self.btree_map.entry(value.clone()).and_modify(|e| *e -= 1);
        if let Some(&cnt) = self.btree_map.get(&value) {
            if cnt == 0 {
                self.btree_map.remove(&value);
            }

            self.size -= 1;
            true
        } else {
            false
        }
    }

    pub fn lower_bound<Q>(&self, bound: Bound<&Q>) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Ord,
    {
        match bound {
            Bound::Unbounded => unreachable!(),
            _ => {
                if let Some((key, _)) = self.btree_map.range((bound, Bound::Unbounded)).next() {
                    Some(key)
                } else {
                    None
                }
            }
        }
    }

    pub fn upper_bound<Q>(&self, bound: Bound<&Q>) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Ord,
    {
        match bound {
            Bound::Unbounded => unreachable!(),
            _ => {
                if let Some((key, _)) = self.btree_map.range((Bound::Unbounded, bound)).next_back()
                {
                    Some(key)
                } else {
                    None
                }
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            range: self.range(..),
        }
    }

    pub fn range<U: ?Sized, R>(&self, range: R) -> Range<'_, T>
    where
        U: Ord,
        T: Borrow<U> + Ord,
        R: RangeBounds<U>,
    {
        Range {
            last: None,
            counter: 0,
            range: self.btree_map.range(range),
        }
    }

    pub fn count<Q>(&self, value: &Q) -> usize
    where
        T: Borrow<Q>,
        Q: Ord,
    {
        if let Some(&cnt) = self.btree_map.get(value) {
            cnt
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Range<'a, T>
where
    T: 'a,
{
    last: Option<&'a T>,
    counter: usize,
    range: btree_map::Range<'a, T, usize>,
}

impl<'a, T> Iterator for Range<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == 0 {
            if let Some((elem, &cnt)) = self.range.next() {
                self.last = Some(elem);
                self.counter = cnt - 1;
                Some(elem)
            } else {
                None
            }
        } else {
            self.counter -= 1;
            self.last
        }
    }
}

impl<'a, T> DoubleEndedIterator for Range<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.counter == 0 {
            if let Some((elem, &cnt)) = self.range.next_back() {
                self.last = Some(elem);
                self.counter = cnt;
                Some(elem)
            } else {
                None
            }
        } else {
            self.counter -= 1;
            self.last
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Iter<'a, T>
where
    T: 'a,
{
    range: Range<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.range.next()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back()
    }
}

// Reference: https://zenn.dev/silva/articles/3af7df176849c2
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let q = scan.token::<i64>();
    let mut prices = MultiSet::new();

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let s = scan.token::<i64>();
            prices.insert(s);
        } else if command == 2 {
            let s = scan.token::<i64>();
            prices.remove(&s);
        } else {
            if prices.is_empty() {
                writeln!(out, "0").unwrap();
                continue;
            }

            let mut curr = *prices.iter().next().unwrap();
            let mut ret = 1;

            loop {
                let next = prices.lower_bound(Bound::Included(&(curr * 2)));

                if let Some(next) = next {
                    curr = *next;
                    ret += 1;
                } else {
                    break;
                }
            }

            writeln!(out, "{ret}").unwrap();
        }
    }
}

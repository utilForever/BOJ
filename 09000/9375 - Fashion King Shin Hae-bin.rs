use io::Write;
use std::borrow::Borrow;
use std::collections::{btree_map, BTreeMap};
use std::{io, str};

#[derive(Debug, Clone)]
pub struct MultiSet<T> {
    freq: BTreeMap<T, usize>,
    len: usize,
}

pub struct Iter<'a, T> {
    iter: btree_map::Iter<'a, T, usize>,
    front: Option<&'a T>,
    front_to_dispatch: usize,
    back: Option<&'a T>,
    back_to_dispatch: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_to_dispatch == 0 {
            if let Some((k, &v)) = self.iter.next() {
                self.front = Some(k);
                self.front_to_dispatch = v;
            } else if self.back_to_dispatch > 0 {
                self.back_to_dispatch -= 1;
                return self.back;
            }
        }
        if self.front_to_dispatch > 0 {
            self.front_to_dispatch -= 1;
            return self.front;
        }
        None
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back_to_dispatch == 0 {
            if let Some((k, &v)) = self.iter.next_back() {
                self.back = Some(k);
                self.back_to_dispatch = v;
            } else if self.front_to_dispatch > 0 {
                self.front_to_dispatch -= 1;
                return self.front;
            }
        }
        if self.back_to_dispatch > 0 {
            self.back_to_dispatch -= 1;
            return self.back;
        }
        None
    }
}

impl<T: Ord> Default for MultiSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MultiSet<T> {
    pub fn new() -> Self
    where
        T: Ord,
    {
        Self {
            freq: BTreeMap::new(),
            len: 0,
        }
    }

    pub fn insert(&mut self, val: T)
    where
        T: Ord,
    {
        *self.freq.entry(val).or_insert(0) += 1;
        self.len += 1;
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.freq.is_empty()
    }

    pub fn contains<Q: ?Sized>(&self, val: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.freq.contains_key(val)
    }

    pub fn remove<Q: ?Sized>(&mut self, val: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        if self.contains(val) {
            *self.freq.get_mut(val).unwrap() -= 1;
            if self.freq[val] == 0 {
                self.freq.remove(val);
            }
            self.len -= 1;
            return true;
        }
        false
    }

    pub fn iter(&self) -> Iter<T>
    where
        T: Ord,
    {
        Iter {
            iter: self.freq.iter(),
            front: None,
            front_to_dispatch: 0,
            back: None,
            back_to_dispatch: 0,
        }
    }
}

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<usize>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut queue = MultiSet::new();
        let mut kinds = HashSet::new();
        let mut count = 1;

        for i in 0..n {
            let (name, kind) = (scan.token::<String>(), scan.token::<String>());
            clothes.insert(kind);
            kinds.insert(kind);
        }

        for kind in kinds.iter() {
            count *= clothes.count(kind) + 1;
        }

        writeln!(out, "{}", count - 1).unwrap();
    }
}

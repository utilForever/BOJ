use io::Write;
use std::{
    collections::{HashMap, HashSet},
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

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];

        for i in 0..n {
            parent[i] = i;
        }

        UnionFind { parent }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            self.parent[root_y] = root_x;
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut rectangles = vec![(0, 0); n];
    let mut lengths = HashSet::new();

    for i in 0..n {
        let (s, t) = (scan.token::<i64>(), scan.token::<i64>());

        rectangles[i] = if s <= t { (s, t) } else { (t, s) };
        lengths.insert(s);
        lengths.insert(t);
    }

    let mut lengths = lengths.into_iter().collect::<Vec<_>>();
    lengths.sort();

    let lengths_compressed = lengths
        .iter()
        .enumerate()
        .map(|(idx, &length)| (length, idx))
        .collect::<HashMap<i64, usize>>();
    let mut rectangles_compressed: HashMap<usize, Vec<usize>> = HashMap::new();

    for (idx, &(s, t)) in rectangles.iter().enumerate() {
        let idx_s = *lengths_compressed.get(&s).unwrap();
        let idx_t = *lengths_compressed.get(&t).unwrap();

        rectangles_compressed.entry(idx_s).or_default().push(idx);
        rectangles_compressed.entry(idx_t).or_default().push(idx);
    }

    let mut union_find = UnionFind::new(n);

    for rects in rectangles_compressed.values() {
        for i in 1..rects.len() {
            union_find.union(rects[0], rects[i]);
        }
    }

    let mut rectangles_group: HashMap<usize, Vec<usize>> = HashMap::new();

    for idx in 0..n {
        let root = union_find.find(idx);
        rectangles_group.entry(root).or_default().push(idx);
    }

    let mut ret = 0;

    for group in rectangles_group.values() {
        let mut widths = HashSet::new();

        for &idx in group {
            let (s, t) = rectangles[idx];
            widths.insert(s);
            widths.insert(t);

            ret += s + t;
        }

        let mut widths = widths.into_iter().collect::<Vec<_>>();
        widths.sort();

        for i in 0..group.len().min(widths.len()) {
            ret -= widths[i];
        }
    }

    writeln!(out, "{ret}").unwrap();
}

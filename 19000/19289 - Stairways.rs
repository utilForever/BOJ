use io::Write;
use std::{
    cmp::Ordering::{self, Less},
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

const INF: i64 = 1_000_000_000_000_000_000;

#[derive(Clone, Default)]
struct Line {
    slope: i64,
    ret: i64,
}

impl Line {
    fn new(a: i64, b: i64) -> Self {
        Self { slope: a, ret: b }
    }

    fn compare(mut a: Line, mut b: Line) -> i64 {
        if a.ret > b.ret {
            std::mem::swap(&mut a, &mut b);
        }

        if a.slope <= b.slope {
            return INF;
        }

        if a.ret == b.ret {
            return INF;
        }

        (a.ret - b.ret) / (b.slope - a.slope) + 1
    }
}

struct KineticSegmentTree {
    size: usize,
    data: Vec<Line>,
    melt: Vec<i64>,
    tag: Vec<bool>,
    time: Vec<i64>,
    intercept: Vec<i64>,
}

impl KineticSegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;

        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![Line::default(); real_n * 4],
            melt: vec![0; real_n * 4],
            tag: vec![false; real_n * 4],
            time: vec![0; real_n * 4],
            intercept: vec![0; real_n * 4],
        }
    }

    pub fn construct(&mut self, lines: &[i64], start: usize, end: usize) {
        self.construct_internal(lines, 1, start, end);
    }

    fn construct_internal(&mut self, lines: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = Line::new(lines[start], 0);
            self.melt[node] = INF;
            return;
        }

        let mid = (start + end) / 2;

        self.construct_internal(lines, node * 2, start, mid);
        self.construct_internal(lines, node * 2 + 1, mid + 1, end);

        self.merge(node);
    }

    fn merge(&mut self, node: usize) {
        let left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();

        self.data[node] =
            if left.ret < right.ret || (left.ret == right.ret && left.slope < right.slope) {
                left.clone()
            } else {
                right.clone()
            };
        self.melt[node] = self.melt[node * 2]
            .min(self.melt[node * 2 + 1])
            .min(Line::compare(left, right));
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if !self.tag[node] {
            return;
        }

        self.data[node].ret += self.data[node].slope * self.time[node] + self.intercept[node];
        self.melt[node] -= self.time[node];

        if start == end {
            self.tag[node] = false;
            self.time[node] = 0;
            self.intercept[node] = 0;
            return;
        }

        self.tag[node * 2] = true;
        self.time[node * 2] += self.time[node];
        self.intercept[node * 2] += self.intercept[node];

        self.tag[node * 2 + 1] = true;
        self.time[node * 2 + 1] += self.time[node];
        self.intercept[node * 2 + 1] += self.intercept[node];

        self.tag[node] = false;
        self.time[node] = 0;
        self.intercept[node] = 0;
    }

    fn add_intercept(&mut self, start: usize, end: usize, val: i64) {
        self.add_intercept_internal(start, end, val, 1, 1, self.size);
    }

    fn add_intercept_internal(
        &mut self,
        start: usize,
        end: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return;
        }

        if start <= node_start && node_end <= end {
            self.tag[node] = true;
            self.intercept[node] += val;
            self.propagate(node, node_start, node_end);
            return;
        }

        let mid = (node_start + node_end) / 2;

        self.add_intercept_internal(start, end, val.clone(), node * 2, node_start, mid);
        self.add_intercept_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.merge(node);
    }

    fn update(&mut self, start: usize, end: usize, val: i64) {
        self.update_internal(start, end, val, 1, 1, self.size);
    }

    fn update_internal(
        &mut self,
        start: usize,
        end: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return;
        }

        if start <= node_start && node_end <= end {
            self.data[node].ret = val;
            self.melt[node] = INF;
            return;
        }

        let mid = (node_start + node_end) / 2;

        self.update_internal(start, end, val.clone(), node * 2, node_start, mid);
        self.update_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.merge(node);
    }

    fn heaten(&mut self, start: usize, end: usize, time: i64) {
        self.heaten_internal(1, start, end, 1, self.size, time);
    }

    fn heaten_internal(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        node_start: usize,
        node_end: usize,
        time: i64,
    ) {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return;
        }

        if start <= node_start && node_end <= end && self.melt[node] > time {
            self.tag[node] = true;
            self.time[node] += 1;
            self.melt[node] -= 1;
            self.propagate(node, node_start, node_end);
            return;
        }

        let mid = (node_start + node_end) / 2;

        self.heaten_internal(node * 2, start, end, node_start, mid, time);
        self.heaten_internal(node * 2 + 1, start, end, mid + 1, node_end, time);

        self.merge(node);
    }

    pub fn query(&mut self, start: usize, end: usize) -> i64 {
        self.query_internal(start, end, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> i64 {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return INF;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].ret;
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.min(right)
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

// Reference: https://koosaga.com/307
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut times = vec![0; n + 1];
    let mut compressed = Vec::with_capacity(n + 2);

    compressed.push(-1);
    compressed.push(0);

    for i in 1..=n {
        times[i] = scan.token::<i64>();
        compressed.push(times[i]);
    }

    compressed.sort_unstable();
    compressed.dedup();

    let cnt = compressed.len() - 1;
    let mut tree = KineticSegmentTree::new(cnt);
    tree.construct(&compressed, 1, cnt);

    let mut max = 0;

    for i in 1..=n {
        let val = times[i];

        if val >= max {
            max = val;
            continue;
        }

        let idx = compressed.lower_bound(&val);
        let time = tree.query(1, idx);

        tree.update(idx, idx, time);
        tree.add_intercept(1, idx - 1, max - val);
        tree.heaten(idx + 1, cnt, 1);
        tree.add_intercept(idx + 1, cnt, -val);
    }

    writeln!(out, "{}", tree.data[1].ret).unwrap();
}

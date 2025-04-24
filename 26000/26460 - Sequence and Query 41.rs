use io::Write;
use std::{io, str};

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

const INF: i64 = 100_000_000_000_000_000;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd)]
struct Line {
    slope: i64,
    ret: i64,
}

impl Line {
    fn new(a: i64, b: i64) -> Self {
        Self { slope: a, ret: b }
    }

    fn compare(mut a: Line, mut b: Line) -> i64 {
        if a.ret < b.ret || (a.ret == b.ret && a.slope < b.slope) {
            std::mem::swap(&mut a, &mut b);
        }

        if a.slope >= b.slope {
            return INF;
        }

        (a.ret - b.ret) / (b.slope - a.slope)
    }
}

impl std::default::Default for Line {
    fn default() -> Self {
        Self {
            slope: 0,
            ret: -INF,
        }
    }
}

impl std::ops::Add for Line {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            slope: self.slope + other.slope,
            ret: self.ret + other.ret,
        }
    }
}

impl std::cmp::Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.ret.cmp(&other.ret);
    }
}

#[derive(Clone, Copy)]
struct Node {
    min_first: i64,
    min_second: i64,
    lazy: i64,
    melt: i64,

    max_left: Line,
    max_right: Line,
    sum: Line,
    val: Line,
}

impl std::default::Default for Node {
    fn default() -> Self {
        Self {
            min_first: INF,
            min_second: INF,
            lazy: -INF,
            melt: INF,

            max_left: Line::default(),
            max_right: Line::default(),
            sum: Line::default(),
            val: Line::default(),
        }
    }
}

impl Node {
    fn merge(mut left: Node, mut right: Node) -> Node {
        let mut ret = Node::default();

        ret.min_first = left.min_first.min(right.min_first);
        ret.min_second = if ret.min_first == left.min_first {
            left.min_second
        } else {
            left.min_first
        }
        .min(if ret.min_first == right.min_first {
            right.min_second
        } else {
            right.min_first
        });

        if ret.min_first != left.min_first {
            left.max_left.slope = 0;
            left.max_right.slope = 0;
            left.sum.slope = 0;
            left.val.slope = 0;
        }

        if ret.min_first != right.min_first {
            right.max_left.slope = 0;
            right.max_right.slope = 0;
            right.sum.slope = 0;
            right.val.slope = 0;
        }

        ret.max_left = left.max_left.max(left.sum + right.max_left);
        ret.max_right = right.max_right.max(right.sum + left.max_right);
        ret.sum = left.sum + right.sum;
        ret.val = left.val.max(right.val).max(left.max_right + right.max_left);

        ret.melt = Line::compare(ret.max_left, left.sum + right.max_left)
            .min(Line::compare(ret.max_right, right.sum + left.max_right));
        ret.melt = ret
            .melt
            .min(Line::compare(ret.val, left.val))
            .min(Line::compare(ret.val, right.val))
            .min(Line::compare(ret.val, left.max_right + right.max_left));
        ret.melt = (ret.melt + ret.min_first).min(left.melt).min(right.melt);

        ret
    }
}

struct KineticSegmentTree {
    size: usize,
    data: Vec<Node>,
}

impl KineticSegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;

        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![Node::default(); real_n * 4],
        }
    }

    pub fn construct(&mut self, lines: &[i64], start: usize, end: usize) {
        self.construct_internal(lines, 1, start, end);
    }

    fn construct_internal(&mut self, lines: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            let line = Line::new(1, lines[start]);

            self.data[node].max_left = line;
            self.data[node].max_right = line;
            self.data[node].sum = line;
            self.data[node].val = line;
            self.data[node].min_first = lines[start];

            return;
        }

        let mid = (start + end) / 2;

        self.construct_internal(lines, node * 2, start, mid);
        self.construct_internal(lines, node * 2 + 1, mid + 1, end);

        self.data[node] = Node::merge(self.data[node * 2], self.data[node * 2 + 1]);
    }

    fn propagate(&mut self, node: usize) {
        if self.data[node].lazy == -INF {
            return;
        }

        if self.data[node].lazy >= self.data[node * 2].min_first {
            let diff = self.data[node].lazy - self.data[node * 2].min_first;

            self.data[node * 2].min_first = self.data[node].lazy;
            self.data[node * 2].lazy = self.data[node * 2].lazy.max(self.data[node].lazy);
            self.data[node * 2].melt -= diff;

            self.data[node * 2].max_left.ret += self.data[node * 2].max_left.slope * diff;
            self.data[node * 2].max_right.ret += self.data[node * 2].max_right.slope * diff;
            self.data[node * 2].sum.ret += self.data[node * 2].sum.slope * diff;
            self.data[node * 2].val.ret += self.data[node * 2].val.slope * diff;
        }

        if self.data[node].lazy >= self.data[node * 2 + 1].min_first {
            let diff = self.data[node].lazy - self.data[node * 2 + 1].min_first;

            self.data[node * 2 + 1].min_first = self.data[node].lazy;
            self.data[node * 2 + 1].lazy = self.data[node * 2 + 1].lazy.max(self.data[node].lazy);
            self.data[node * 2 + 1].melt -= diff;

            self.data[node * 2 + 1].max_left.ret += self.data[node * 2 + 1].max_left.slope * diff;
            self.data[node * 2 + 1].max_right.ret += self.data[node * 2 + 1].max_right.slope * diff;
            self.data[node * 2 + 1].sum.ret += self.data[node * 2 + 1].sum.slope * diff;
            self.data[node * 2 + 1].val.ret += self.data[node * 2 + 1].val.slope * diff;
        }

        self.data[node].lazy = -INF;
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
        self.propagate(node);

        if end < node_start || node_end < start || val <= self.data[node].min_first {
            return;
        }

        if start <= node_start
            && node_end <= end
            && val < self.data[node].min_second
            && val <= self.data[node].melt
        {
            let diff = val - self.data[node].min_first;

            self.data[node].max_left.ret += self.data[node].max_left.slope * diff;
            self.data[node].max_right.ret += self.data[node].max_right.slope * diff;
            self.data[node].sum.ret += self.data[node].sum.slope * diff;
            self.data[node].val.ret += self.data[node].val.slope * diff;

            self.data[node].min_first = val;
            self.data[node].lazy = self.data[node].lazy.max(val);
            self.data[node].melt -= diff;

            return;
        }

        let mid = (node_start + node_end) / 2;

        self.update_internal(start, end, val.clone(), node * 2, node_start, mid);
        self.update_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = Node::merge(self.data[node * 2], self.data[node * 2 + 1]);
    }

    pub fn query(&mut self, start: usize, end: usize) -> Node {
        self.query_internal(start, end, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> Node {
        if end < node_start || node_end < start {
            return Node::default();
        }

        if start <= node_start && node_end <= end {
            return self.data[node];
        }

        self.propagate(node);

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        Node::merge(left, right)
    }
}

// Reference: https://koosaga.com/307
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = vec![0; n + 1];

    for i in 1..=n {
        nums[i] = scan.token::<i64>();
    }

    let mut tree = KineticSegmentTree::new(n);
    tree.construct(&nums, 1, n);

    for _ in 0..q {
        let cmd = scan.token::<i64>();

        if cmd == 0 {
            let (l, r, x) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            tree.update(l, r, x);
        } else {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(out, "{}", tree.query(l, r).val.ret.max(0)).unwrap();
        }
    }
}

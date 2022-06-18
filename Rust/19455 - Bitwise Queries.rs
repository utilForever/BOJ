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

#[derive(Clone, Default)]
struct Node {
    one: i64,
    zero: i64,
    min: i64,
}

struct LazySegmentTree {
    size: usize,
    data: Vec<Node>,
    lazy_and: Vec<i64>,
    lazy_or: Vec<i64>,
}

impl LazySegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![Node::default(); real_n * 4],
            lazy_and: vec![0; real_n * 4],
            lazy_or: vec![0; real_n * 4],
        }
    }

    fn merge(a: &Node, b: &Node) -> Node {
        Node {
            one: a.one & b.one,
            zero: a.zero & b.zero,
            min: a.min.min(b.min),
        }
    }

    pub fn construct(&mut self, arr: &[i64], start: usize, end: usize) {
        self.construct_internal(arr, 1, start, end);
    }

    fn construct_internal(&mut self, arr: &[i64], node: usize, start: usize, end: usize) -> Node {
        if start == end {
            self.data[node] = Node {
                one: arr[start],
                zero: !arr[start],
                min: arr[start],
            };
            self.data[node].clone()
        } else {
            let mid = (start + end) / 2;

            let left = self.construct_internal(arr, node * 2, start, mid);
            let right = self.construct_internal(arr, node * 2 + 1, mid + 1, end);

            self.data[node] = LazySegmentTree::merge(&left, &right);
            self.data[node].clone()
        }
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        self.data[node].one |= self.lazy_or[node];
        self.data[node].one &= !self.lazy_and[node];

        self.data[node].zero &= !self.lazy_or[node];
        self.data[node].zero |= self.lazy_and[node];

        self.data[node].min |= self.lazy_or[node];
        self.data[node].min &= !self.lazy_and[node];

        if start == end {
            self.lazy_and[node] = 0;
            self.lazy_or[node] = 0;
            return;
        }

        self.lazy_or[node * 2] &= !self.lazy_and[node];
        self.lazy_or[node * 2] |= self.lazy_or[node];

        self.lazy_and[node * 2] &= !self.lazy_or[node];
        self.lazy_and[node * 2] |= self.lazy_and[node];

        self.lazy_or[node * 2 + 1] &= !self.lazy_and[node];
        self.lazy_or[node * 2 + 1] |= self.lazy_or[node];

        self.lazy_and[node * 2 + 1] &= !self.lazy_or[node];
        self.lazy_and[node * 2 + 1] |= self.lazy_and[node];

        self.lazy_and[node] = 0;
        self.lazy_or[node] = 0;
    }

    pub fn update_and(&mut self, start: usize, end: usize, val: i64) {
        self.update_and_internal(start, end, val, 1, 1, self.size);
    }

    fn update_and_internal(
        &mut self,
        start: usize,
        end: usize,
        mut val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return;
        }

        val &= !self.data[node].zero;

        if start <= node_start && node_end <= end && (val & self.data[node].one) != 0 {
            self.lazy_and[node] = val & self.data[node].one;
            val &= !self.data[node].one;
            self.propagate(node, node_start, node_end);
        }

        if val == 0 {
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_and_internal(start, end, val, node * 2, node_start, mid);
        self.update_and_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = LazySegmentTree::merge(&self.data[node * 2], &self.data[node * 2 + 1]);
    }

    pub fn update_or(&mut self, start: usize, end: usize, val: i64) {
        self.update_or_internal(start, end, val, 1, 1, self.size);
    }

    fn update_or_internal(
        &mut self,
        start: usize,
        end: usize,
        mut val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return;
        }

        val &= !self.data[node].one;

        if start <= node_start && node_end <= end && (val & self.data[node].zero) != 0 {
            self.lazy_or[node] = val & self.data[node].zero;
            val &= !self.data[node].zero;
            self.propagate(node, node_start, node_end);
        }

        if val == 0 {
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_or_internal(start, end, val, node * 2, node_start, mid);
        self.update_or_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = LazySegmentTree::merge(&self.data[node * 2], &self.data[node * 2 + 1]);
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
            return i64::MAX;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].min;
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.min(right)
    }
}

// Reference: https://justicehui.github.io/hard-algorithm/2019/10/10/segment-tree-beats/
// Reference: https://www.secmem.org/blog/2019/10/19/Segment-Tree-Beats/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    let mut tree = LazySegmentTree::new(n);
    let mut arr = vec![0; n + 1];

    for i in 1..=n {
        arr[i] = scan.token::<i64>();
    }

    tree.construct(&arr, 1, n);

    let m = scan.token::<usize>();

    for _ in 0..m {
        let command = scan.token::<String>();

        if command.starts_with("&") {
            let (l, r, x) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            tree.update_and(l, r, !x);
        } else if command.starts_with("|") {
            let (l, r, x) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            tree.update_or(l, r, x);
        } else {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(out, "{}", tree.query(l, r)).unwrap();
        }
    }
}

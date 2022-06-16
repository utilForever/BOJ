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
    max_first: i64,
    max_second: i64,
    num_max_first: i64,
    sum: i64,
}

struct LazySegmentTree {
    size: usize,
    data: Vec<Node>,
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
        }
    }

    fn merge(a: &Node, b: &Node) -> Node {
        if a.max_first == b.max_first {
            Node {
                max_first: a.max_first,
                max_second: a.max_second.max(b.max_second),
                num_max_first: a.num_max_first + b.num_max_first,
                sum: a.sum + b.sum,
            }
        } else if a.max_first > b.max_first {
            Node {
                max_first: a.max_first,
                max_second: a.max_second.max(b.max_first),
                num_max_first: a.num_max_first,
                sum: a.sum + b.sum,
            }
        } else {
            Node {
                max_first: b.max_first,
                max_second: a.max_first.max(b.max_second),
                num_max_first: b.num_max_first,
                sum: a.sum + b.sum,
            }
        }
    }

    pub fn construct(&mut self, arr: &[i64], start: usize, end: usize) {
        self.construct_internal(arr, 1, start, end);
    }

    fn construct_internal(&mut self, arr: &[i64], node: usize, start: usize, end: usize) -> Node {
        if start == end {
            self.data[node] = Node {
                max_first: arr[start],
                max_second: -1,
                num_max_first: 1,
                sum: arr[start],
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
        if start == end {
            return;
        }

        if self.data[node].max_first < self.data[node * 2].max_first {
            self.data[node * 2].sum -= self.data[node * 2].num_max_first
                * (self.data[node * 2].max_first - self.data[node].max_first);
            self.data[node * 2].max_first = self.data[node].max_first;
        }

        if self.data[node].max_first < self.data[node * 2 + 1].max_first {
            self.data[node * 2 + 1].sum -= self.data[node * 2 + 1].num_max_first
                * (self.data[node * 2 + 1].max_first - self.data[node].max_first);
            self.data[node * 2 + 1].max_first = self.data[node].max_first;
        }
    }

    pub fn update(&mut self, start: usize, end: usize, val: i64) {
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

        if end < node_start || node_end < start || self.data[node].max_first <= val {
            return;
        }

        if start <= node_start && node_end <= end && self.data[node].max_second < val {
            self.data[node].sum -=
                self.data[node].num_max_first * (self.data[node].max_first - val);
            self.data[node].max_first = val;
            self.propagate(node, node_start, node_end);
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(start, end, val, node * 2, node_start, mid);
        self.update_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = LazySegmentTree::merge(&self.data[node * 2], &self.data[node * 2 + 1]);
    }

    pub fn query_max(&mut self, start: usize, end: usize) -> i64 {
        self.query_max_internal(start, end, 1, 1, self.size)
    }

    fn query_max_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> i64 {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return 0;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].max_first;
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_max_internal(start, end, node * 2, node_start, mid);
        let right = self.query_max_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.max(right)
    }

    pub fn query_sum(&mut self, start: usize, end: usize) -> i64 {
        self.query_sum_internal(start, end, 1, 1, self.size)
    }

    fn query_sum_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> i64 {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return 0;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].sum;
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_sum_internal(start, end, node * 2, node_start, mid);
        let right = self.query_sum_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left + right
    }
}

// Reference: https://justicehui.github.io/hard-algorithm/2019/10/10/segment-tree-beats/
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
        let command = scan.token::<usize>();

        if command == 1 {
            let (l, r, x) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            tree.update(l, r, x);
        } else if command == 2 {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(out, "{}", tree.query_max(l, r)).unwrap();
        } else {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(out, "{}", tree.query_sum(l, r)).unwrap();
        }
    }
}

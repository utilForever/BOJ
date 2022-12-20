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

static MIN: i64 = -1_000_000_000_000_000_000;

#[derive(Clone)]
struct Max {
    first: i64,
    history: i64,
}

#[derive(Clone)]
struct Value {
    value: i64,
    max: i64,
}

#[derive(Clone)]
struct Node {
    max: Max,
    a: Value,
}

impl Node {
    fn new() -> Self {
        Self {
            max: Max {
                first: MIN,
                history: 0,
            },
            a: Value { value: 0, max: 0 },
        }
    }
}

#[derive(Clone, Default)]
struct Add {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    val: i64,
}

#[derive(Clone, Default)]
struct Query {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

struct LazySegmentTree {
    size: usize,
    data: Vec<Node>,
    list_add: Vec<Vec<Add>>,
    list_query: Vec<Vec<Query>>,
}

impl LazySegmentTree {
    pub fn new(n: usize, q: usize) -> Self {
        Self {
            size: n,
            data: vec![Node::new(); n * 4],
            list_add: vec![Vec::new(); n * 4],
            list_query: vec![Vec::new(); q],
        }
    }

    fn merge(node: &Node, a: &Node, b: &Node) -> Node {
        let mut ret = node.clone();

        ret.max.first = a.max.first.max(b.max.first);
        ret.max.history = a.max.history.max(b.max.history);

        ret
    }

    pub fn construct(&mut self, start: usize, end: usize) {
        self.construct_internal(1, start, end);
    }

    fn construct_internal(&mut self, node: usize, start: usize, end: usize) -> Node {
        if start == end {
            self.data[node] = Node {
                max: Max {
                    first: 0,
                    history: 0,
                },
                a: Value { value: 0, max: 0 },
            };
            self.data[node].clone()
        } else {
            let mid = (start + end) / 2;

            let left = self.construct_internal(node * 2, start, mid);
            let right = self.construct_internal(node * 2 + 1, mid + 1, end);

            self.data[node] = LazySegmentTree::merge(&self.data[node], &left, &right);
            self.data[node].clone()
        }
    }

    fn process_add(&mut self, node: usize, val: i64) {
        self.data[node].max.first += val;

        self.data[node].max.history = self.data[node].max.history.max(self.data[node].max.first);

        self.data[node].a.value += val;
        self.data[node].a.max = self.data[node].a.max.max(self.data[node].a.value);
    }

    fn update_a(&mut self, node: usize, val: Value) {
        self.data[node].a.max = self.data[node].a.max.max(self.data[node].a.value + val.max);
        self.data[node].a.value += val.value;
        self.data[node].max.history = self.data[node]
            .max
            .history
            .max(self.data[node].max.first + val.max);

        self.data[node].max.first += val.value;
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if start == end {
            return;
        }

        self.update_a(node * 2, self.data[node].a.clone());
        self.update_a(node * 2 + 1, self.data[node].a.clone());

        self.data[node].a = Value { value: 0, max: 0 };
    }

    pub fn push_add(&mut self, node: usize, start: usize, end: usize, node_start: usize, node_end: usize, add: Add) {
        if end < node_start || node_end < start {
            return;
        }

        self.list_add[node].push(add.clone());

        if start <= node_start && node_end <= end {
            return;
        }

        let mid = (node_start + node_end) / 2;

        self.push_add(node * 2, start, end, node_start, mid, add.clone());
        self.push_add(node * 2 + 1, start, end, mid + 1, node_end, add);
    }

    pub fn push_query(&mut self, node: usize, start: usize, end: usize, node_start: usize, node_end: usize, query: Query) {
        if end < node_start || node_end < start {
            return;
        }

        let mid = (node_start + node_end) / 2;

        if start <= mid && mid + 1 < end {
            self.list_query[node].push(query.clone());
        } else if start == mid + 1 || end == mid {
            self.list_query[node].push(query.clone());
        }

        self.push_query(node * 2, start, end, node_start, mid, query.clone());
        self.push_query(node * 2 + 1, start, end, mid + 1, node_end, query);
    }

    pub fn update_add(&mut self, start: usize, end: usize, val: i64) {
        self.update_add_internal(start, end, val, 1, 1, self.size);
    }

    fn update_add_internal(
        &mut self,
        start: usize,
        end: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if end < node_start || node_end < start {
            return;
        }

        if start <= node_start && node_end <= end {
            self.process_add(node, val);
            self.propagate(node, node_start, node_end);
            return;
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;
        self.update_add_internal(start, end, val, node * 2, node_start, mid);
        self.update_add_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = LazySegmentTree::merge(
            &self.data[node],
            &self.data[node * 2],
            &self.data[node * 2 + 1],
        );
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
            return MIN;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].max.history;
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_max_internal(start, end, node * 2, node_start, mid);
        let right = self.query_max_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.max(right)
    }
}

// Reference: https://justicehui.github.io/hard-algorithm/2019/10/10/segment-tree-beats/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m1, m2) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    let mut tree = LazySegmentTree::new(n, m2);
    tree.construct(1, n);

    for _ in 0..m1 {
        let (x1, y1, x2, y2, w) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        if w == 0 {
            continue;
        }

        tree.push_add(1, x1, x2, 1, n, Add{ x1, y1, x2, y2, val: w });
    }

    for _ in 0..m2 {
        let (x1, y1, x2, y2) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        tree.push_query(1, x1, x2, 1, n, Query{ x1, y1, x2, y2 });
    }
}

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

static INF: i64 = 50_000_000_000_000;

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
                first: 0,
                history: 0,
            },
            a: Value { value: 0, max: 0 },
        }
    }
}

#[derive(Clone, Default)]
struct Update {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    val: i64,
}

#[derive(Clone, Default, Debug)]
struct Query {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    idx: usize,
}

struct LazySegmentTree {
    size: usize,
    data: Vec<Node>,
    list_add: Vec<Vec<Update>>,
    list_update: Vec<Vec<Update>>,
    list_query: Vec<Vec<Query>>,

    adds: Vec<Vec<(usize, usize, i64)>>,
    removes: Vec<Vec<(usize, usize, i64)>>,
    queries: Vec<Vec<(usize, usize, usize)>>,
}

impl LazySegmentTree {
    pub fn new(n: usize) -> Self {
        Self {
            size: n,
            data: vec![Node::new(); n * 4],
            list_add: vec![Vec::new(); n * 4],
            list_update: vec![Vec::new(); n * 4],
            list_query: vec![Vec::new(); n * 4],

            adds: vec![Vec::new(); n + 2],
            removes: vec![Vec::new(); n + 2],
            queries: vec![Vec::new(); n + 2],
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

    pub fn push_add(&mut self, node: usize, start: usize, end: usize, add: Update) {
        if add.x1 <= start && end <= add.x2 {
            self.list_add[node].push(add.clone());
            return;
        }

        self.list_update[node].push(add.clone());

        let mid = (start + end) / 2;

        if add.x1 <= mid {
            self.push_add(node * 2, start, mid, add.clone());
        }
        
        if mid + 1 <= add.x2 {
            self.push_add(node * 2 + 1, mid + 1, end, add);
        }
    }

    pub fn push_query(&mut self, node: usize, start: usize, end: usize, query: Query) {
        let mid = (start + end) / 2;

        if query.x1 <= mid && mid + 1 <= query.x2 {
            return self.list_query[node].push(query.clone());
        } else if query.x1 == mid + 1 || query.x2 == mid {
            return self.list_query[node].push(query.clone());
        }

        if query.x1 <= mid {
            self.push_query(node * 2, start, mid, query.clone());
        } else {
            self.push_query(node * 2 + 1, mid + 1, end, query);
        }
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
            return 0;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].max.history;
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_max_internal(start, end, node * 2, node_start, mid);
        let right = self.query_max_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.max(right)
    }

    fn solve(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        offset: &mut i64,
        ret: &mut Vec<i64>,
    ) {
        if start == end {
            return;
        }

        let iter_add = self.list_add[node].clone();
        let iter_update = self.list_update[node].clone();
        let iter_query = self.list_query[node].clone();

        for add in iter_add.iter() {
            self.update_add(add.y1, add.y2, add.val);
        }

        for i in (start - 1)..=(end + 1) {
            self.adds[i].clear();
            self.removes[i].clear();
            self.queries[i].clear();
        }

        let mid = (start + end) / 2;

        for update in iter_update.iter() {
            let left = update.x1.max(start);
            let right = update.x2.min(mid);

            if left <= right {
                self.adds[right].push((update.y1, update.y2, update.val));
                self.removes[left - 1].push((update.y1, update.y2, -update.val));
            }
        }

        for query in iter_query.iter() {
            if query.x2 >= mid {
                self.queries[query.x1].push((query.y1, query.y2, query.idx));
            }
        }

        self.update_add(1, self.size, INF);
        *offset += INF;

        for i in ((start - 1)..=mid).rev() {
            let adds = self.adds[i].clone();
            let removes = self.removes[i].clone();
            let queries = self.queries[i].clone();

            for val in removes.iter() {
                self.update_add(val.0, val.1, val.2);
            }

            for val in adds.iter() {
                self.update_add(val.0, val.1, val.2);
            }

            for val in queries.iter() {
                ret[val.2] = ret[val.2].max(self.query_max(val.0, val.1) - *offset);
            }
        }

        for update in iter_update.iter() {
            let left = update.x1.max(mid + 1);
            let right = update.x2.min(end);

            if left <= right {
                self.adds[left].push((update.y1, update.y2, update.val));
                self.removes[right + 1].push((update.y1, update.y2, -update.val));
            }
        }

        for query in iter_query.iter() {
            if query.x1 <= mid + 1 {
                self.queries[query.x2].push((query.y1, query.y2, query.idx));
            }
        }

        self.update_add(1, self.size, INF);
        *offset += INF;

        for i in (mid + 1)..=(end + 1) {
            let adds = self.adds[i].clone();
            let removes = self.removes[i].clone();
            let queries = self.queries[i].clone();

            for val in removes.iter() {
                self.update_add(val.0, val.1, val.2);
            }

            for val in adds.iter() {
                self.update_add(val.0, val.1, val.2);
            }

            for val in queries.iter() {
                ret[val.2] = ret[val.2].max(self.query_max(val.0, val.1) - *offset);
            }
        }

        self.solve(node * 2, start, mid, offset, ret);
        self.solve(node * 2 + 1, mid + 1, end, offset, ret);

        for val in iter_add.iter() {
            self.update_add(val.y1, val.y2, -val.val);
        }
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

    let mut tree = LazySegmentTree::new(n);
    tree.construct(1, n);

    for _ in 1..=m1 {
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

        tree.push_add(
            1,
            1,
            n,
            Update {
                x1,
                y1,
                x2,
                y2,
                val: w,
            },
        );
    }

    for i in 1..=m2 {
        let (x1, y1, x2, y2) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        tree.push_query(
            1,
            1,
            n,
            Query {
                x1,
                y1,
                x2,
                y2,
                idx: i,
            },
        );
    }

    let mut ret = vec![0; m2 + 1];
    let mut offset = 0;

    tree.solve(1, 1, n, &mut offset, &mut ret);

    for i in 1..=m2 {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}

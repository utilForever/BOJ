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
static MAX: i64 = 1_000_000_000_000_000_000;

#[derive(Clone)]
struct Max {
    first: i64,
    second: i64,
    history: i64,
}

#[derive(Clone)]
struct Min {
    first: i64,
    second: i64,
    history: i64,
}

#[derive(Clone)]
struct Value {
    value: i64,
    max: i64,
    min: i64,
}

#[derive(Clone)]
struct Node {
    max: Max,
    min: Min,
    a: Value,
    b: Value,
    c: Value,
}

impl Node {
    fn new() -> Self {
        Self {
            max: Max {
                first: MIN,
                second: MIN,
                history: 0,
            },
            min: Min {
                first: MAX,
                second: MAX,
                history: 0,
            },
            a: Value {
                value: 0,
                max: 0,
                min: 0,
            },
            b: Value {
                value: 0,
                max: 0,
                min: 0,
            },
            c: Value {
                value: 0,
                max: 0,
                min: 0,
            },
        }
    }
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
            data: vec![Node::new(); real_n * 4],
        }
    }

    fn merge(node: &Node, a: &Node, b: &Node) -> Node {
        let mut ret = node.clone();

        ret.max.first = a.max.first.max(b.max.first);
        ret.max.second = a.max.second.max(b.max.second);
        ret.max.history = a.max.history.max(b.max.history);

        if a.max.first != ret.max.first {
            ret.max.second = ret.max.second.max(a.max.first);
        }

        if b.max.first != ret.max.first {
            ret.max.second = ret.max.second.max(b.max.first);
        }

        ret.min.first = a.min.first.min(b.min.first);
        ret.min.second = a.min.second.min(b.min.second);
        ret.min.history = a.min.history.min(b.min.history);

        if a.min.first != ret.min.first {
            ret.min.second = ret.min.second.min(a.min.first);
        }

        if b.min.first != ret.min.first {
            ret.min.second = ret.min.second.min(b.min.first);
        }

        ret
    }

    pub fn construct(&mut self, arr: &[i64], start: usize, end: usize) {
        self.construct_internal(arr, 1, start, end);
    }

    fn construct_internal(&mut self, arr: &[i64], node: usize, start: usize, end: usize) -> Node {
        if start == end {
            self.data[node] = Node {
                max: Max {
                    first: arr[start],
                    second: MIN,
                    history: arr[start],
                },
                min: Min {
                    first: arr[start],
                    second: MAX,
                    history: arr[start],
                },
                a: Value {
                    value: 0,
                    max: 0,
                    min: 0,
                },
                b: Value {
                    value: 0,
                    max: 0,
                    min: 0,
                },
                c: Value {
                    value: 0,
                    max: 0,
                    min: 0,
                },
            };
            self.data[node].clone()
        } else {
            let mid = (start + end) / 2;

            let left = self.construct_internal(arr, node * 2, start, mid);
            let right = self.construct_internal(arr, node * 2 + 1, mid + 1, end);

            self.data[node] = LazySegmentTree::merge(&self.data[node], &left, &right);
            self.data[node].clone()
        }
    }

    fn process_add(&mut self, node: usize, val: i64) {
        self.data[node].max.first += val;
        self.data[node].min.first += val;

        if self.data[node].max.second != MIN {
            self.data[node].max.second += val;
        }
        if self.data[node].min.second != MAX {
            self.data[node].min.second += val;
        }

        self.data[node].max.history = self.data[node].max.history.max(self.data[node].max.first);
        self.data[node].min.history = self.data[node].min.history.min(self.data[node].min.first);

        self.data[node].a.value += val;
        self.data[node].a.max = self.data[node].a.max.max(self.data[node].a.value);
        self.data[node].a.min = self.data[node].a.min.min(self.data[node].a.value);

        self.data[node].b.value += val;
        self.data[node].b.max = self.data[node].b.max.max(self.data[node].b.value);
        self.data[node].b.min = self.data[node].b.min.min(self.data[node].b.value);

        self.data[node].c.value += val;
        self.data[node].c.max = self.data[node].c.max.max(self.data[node].c.value);
        self.data[node].c.min = self.data[node].c.min.min(self.data[node].c.value);
    }

    fn update_a(&mut self, node: usize, min: Min, max: Max, val: Value) {
        self.data[node].min.history = self.data[node]
            .min
            .history
            .min(self.data[node].min.first + val.min);

        if min.first == max.second {
            self.data[node].max.second += val.value;
        }

        self.data[node].min.first += val.value;

        self.data[node].a.max = self.data[node].a.max.max(self.data[node].a.value + val.max);
        self.data[node].a.min = self.data[node].a.min.min(self.data[node].a.value + val.min);
        self.data[node].a.value += val.value;
    }

    fn update_b(&mut self, node: usize, min: Min, max: Max, val: Value) {
        if max.first != min.second && self.data[node].min.second != MAX {
            self.data[node].min.second += val.value;
        }

        if max.second != min.first && self.data[node].max.second != MIN {
            self.data[node].max.second += val.value;
        }

        self.data[node].b.max = self.data[node].b.max.max(self.data[node].b.value + val.max);
        self.data[node].b.min = self.data[node].b.min.min(self.data[node].b.value + val.min);
        self.data[node].b.value += val.value;
    }

    fn update_c(&mut self, node: usize, min: Min, max: Max, val: Value) {
        self.data[node].max.history = self.data[node]
            .max
            .history
            .max(self.data[node].max.first + val.max);

        if max.first == min.second {
            self.data[node].min.second += val.value;
        }

        self.data[node].max.first += val.value;

        self.data[node].c.max = self.data[node].c.max.max(self.data[node].c.value + val.max);
        self.data[node].c.min = self.data[node].c.min.min(self.data[node].c.value + val.min);
        self.data[node].c.value += val.value;
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if start == end {
            return;
        }

        let value_min = self.data[node * 2]
            .min
            .first
            .min(self.data[node * 2 + 1].min.first);
        let value_max = self.data[node * 2]
            .max
            .first
            .max(self.data[node * 2 + 1].max.first);

        let min = self.data[node * 2].min.clone();
        let max = self.data[node * 2].max.clone();

        if self.data[node * 2].min.first == value_min {
            self.update_a(
                node * 2,
                min.clone(),
                max.clone(),
                self.data[node].a.clone(),
            );
        } else if self.data[node * 2].min.first == value_max {
            self.update_a(
                node * 2,
                min.clone(),
                max.clone(),
                self.data[node].c.clone(),
            );
        } else {
            self.update_a(
                node * 2,
                min.clone(),
                max.clone(),
                self.data[node].b.clone(),
            );
        }

        self.update_b(
            node * 2,
            min.clone(),
            max.clone(),
            self.data[node].b.clone(),
        );

        if self.data[node * 2].max.first == value_max {
            self.update_c(
                node * 2,
                min.clone(),
                max.clone(),
                self.data[node].c.clone(),
            );
        } else if self.data[node * 2].max.first == value_min {
            self.update_c(
                node * 2,
                min.clone(),
                max.clone(),
                self.data[node].a.clone(),
            );
        } else {
            self.update_c(
                node * 2,
                min.clone(),
                max.clone(),
                self.data[node].b.clone(),
            );
        }

        let min = self.data[node * 2 + 1].min.clone();
        let max = self.data[node * 2 + 1].max.clone();

        if self.data[node * 2 + 1].min.first == value_min {
            self.update_a(
                node * 2 + 1,
                min.clone(),
                max.clone(),
                self.data[node].a.clone(),
            );
        } else if self.data[node * 2 + 1].min.first == value_max {
            self.update_a(
                node * 2 + 1,
                min.clone(),
                max.clone(),
                self.data[node].c.clone(),
            );
        } else {
            self.update_a(
                node * 2 + 1,
                min.clone(),
                max.clone(),
                self.data[node].b.clone(),
            );
        }

        self.update_b(
            node * 2 + 1,
            min.clone(),
            max.clone(),
            self.data[node].b.clone(),
        );

        if self.data[node * 2 + 1].max.first == value_max {
            self.update_c(
                node * 2 + 1,
                min.clone(),
                max.clone(),
                self.data[node].c.clone(),
            );
        } else if self.data[node * 2 + 1].max.first == value_min {
            self.update_c(
                node * 2 + 1,
                min.clone(),
                max.clone(),
                self.data[node].a.clone(),
            );
        } else {
            self.update_c(
                node * 2 + 1,
                min.clone(),
                max.clone(),
                self.data[node].b.clone(),
            );
        }

        self.data[node].a = Value {
            value: 0,
            max: 0,
            min: 0,
        };
        self.data[node].b = Value {
            value: 0,
            max: 0,
            min: 0,
        };
        self.data[node].c = Value {
            value: 0,
            max: 0,
            min: 0,
        };
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

    pub fn update_max(&mut self, start: usize, end: usize, val: i64) {
        self.update_max_internal(start, end, val, 1, 1, self.size);
    }

    fn update_max_internal(
        &mut self,
        start: usize,
        end: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if end < node_start || node_end < start || self.data[node].min.first >= val {
            return;
        }

        if start <= node_start && node_end <= end && self.data[node].min.second > val {
            let value = Value {
                value: val - self.data[node].min.first,
                min: 0,
                max: val - self.data[node].min.first,
            };
            let min = self.data[node].min.clone();
            let max = self.data[node].max.clone();

            if self.data[node].max.first == self.data[node].min.first {
                self.update_c(node, min.clone(), max.clone(), value.clone());
            }

            self.update_a(node, min, max, value);

            self.propagate(node, node_start, node_end);
            return;
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;
        self.update_max_internal(start, end, val, node * 2, node_start, mid);
        self.update_max_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = LazySegmentTree::merge(
            &self.data[node],
            &self.data[node * 2],
            &self.data[node * 2 + 1],
        );
    }

    pub fn update_min(&mut self, start: usize, end: usize, val: i64) {
        self.update_min_internal(start, end, val, 1, 1, self.size);
    }

    fn update_min_internal(
        &mut self,
        start: usize,
        end: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if end < node_start || node_end < start || self.data[node].max.first <= val {
            return;
        }

        if start <= node_start && node_end <= end && self.data[node].max.second < val {
            let value = Value {
                value: val - self.data[node].max.first,
                min: val - self.data[node].max.first,
                max: 0,
            };
            let min = self.data[node].min.clone();
            let max = self.data[node].max.clone();

            if self.data[node].max.first == self.data[node].min.first {
                self.update_a(node, min.clone(), max.clone(), value.clone());
            }

            self.update_c(node, min, max, value);

            self.propagate(node, node_start, node_end);
            return;
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;
        self.update_min_internal(start, end, val, node * 2, node_start, mid);
        self.update_min_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = LazySegmentTree::merge(
            &self.data[node],
            &self.data[node * 2],
            &self.data[node * 2 + 1],
        );
    }

    pub fn query_min_a(&mut self, start: usize, end: usize) -> i64 {
        self.query_min_a_internal(start, end, 1, 1, self.size)
    }

    fn query_min_a_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> i64 {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return MAX;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].min.first;
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_min_a_internal(start, end, node * 2, node_start, mid);
        let right = self.query_min_a_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.min(right)
    }

    pub fn query_min_b(&mut self, start: usize, end: usize) -> i64 {
        self.query_min_b_internal(start, end, 1, 1, self.size)
    }

    fn query_min_b_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> i64 {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return MAX;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].min.history;
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_min_b_internal(start, end, node * 2, node_start, mid);
        let right = self.query_min_b_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.min(right)
    }

    pub fn query_max_c(&mut self, start: usize, end: usize) -> i64 {
        self.query_max_c_internal(start, end, 1, 1, self.size)
    }

    fn query_max_c_internal(
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
        let left = self.query_max_c_internal(start, end, node * 2, node_start, mid);
        let right = self.query_max_c_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.max(right)
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

            if x == 0 {
                continue;
            }

            tree.update_add(l, r, x);
        } else if command == 2 {
            let (l, r, y) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            tree.update_max(l, r, y);
        } else if command == 3 {
            let (l, r, y) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            tree.update_min(l, r, y);
        } else if command == 4 {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(out, "{}", tree.query_min_a(l, r)).unwrap();
        } else if command == 5 {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(out, "{}", tree.query_min_b(l, r)).unwrap();
        } else {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(out, "{}", tree.query_max_c(l, r)).unwrap();
        }
    }
}

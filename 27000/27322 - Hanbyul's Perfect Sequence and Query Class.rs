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

#[derive(Clone, Default, Debug)]
struct Node {
    min_first: i64,
    min_second: i64,
    num_min_first: i64,
    max_first: i64,
    max_second: i64,
    num_max_first: i64,
    sum: i64,
}

struct LazySegmentTree {
    size: usize,
    data: Vec<Node>,
    lazy_add: Vec<i64>,
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
            lazy_add: vec![0; real_n * 4],
        }
    }

    fn merge(a: &Node, b: &Node) -> Node {
        let mut ret = Node::default();

        match a.min_first.cmp(&b.min_first) {
            std::cmp::Ordering::Less => {
                ret.min_first = a.min_first;
                ret.num_min_first = a.num_min_first;
                ret.min_second = a.min_second.min(b.min_first);
            }
            std::cmp::Ordering::Greater => {
                ret.min_first = b.min_first;
                ret.num_min_first = b.num_min_first;
                ret.min_second = b.min_second.min(a.min_first);
            }
            std::cmp::Ordering::Equal => {
                ret.min_first = a.min_first;
                ret.num_min_first = a.num_min_first + b.num_min_first;
                ret.min_second = a.min_second.min(b.min_second);
            }
        }

        match a.max_first.cmp(&b.max_first) {
            std::cmp::Ordering::Less => {
                ret.max_first = b.max_first;
                ret.num_max_first = b.num_max_first;
                ret.max_second = b.max_second.max(a.max_first);
            }
            std::cmp::Ordering::Greater => {
                ret.max_first = a.max_first;
                ret.num_max_first = a.num_max_first;
                ret.max_second = a.max_second.max(b.max_first);
            }
            std::cmp::Ordering::Equal => {
                ret.max_first = a.max_first;
                ret.num_max_first = a.num_max_first + b.num_max_first;
                ret.max_second = a.max_second.max(b.max_second);
            }
        }

        ret.sum = a.sum + b.sum;

        ret
    }

    pub fn construct(&mut self, arr: &[i64], start: usize, end: usize) {
        self.construct_internal(arr, 1, start, end);
    }

    fn construct_internal(&mut self, arr: &[i64], node: usize, start: usize, end: usize) -> Node {
        if start == end {
            self.data[node] = Node {
                min_first: arr[start],
                min_second: MAX,
                num_min_first: 1,
                max_first: arr[start],
                max_second: MIN,
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

    fn process_min(&mut self, node: usize, start: usize, end: usize, val: i64) {
        if val >= self.data[node].max_first {
            return;
        }

        if start == end {
            self.data[node].min_first = val;
            self.data[node].max_first = val;
            self.data[node].sum = val;
            return;
        }

        self.data[node].sum += self.data[node].num_max_first * (val - self.data[node].max_first);
        self.data[node].max_first = val;

        if val <= self.data[node].min_first {
            self.data[node].min_first = val;
        } else if val < self.data[node].min_second {
            self.data[node].min_second = val;
        }
    }

    fn process_max(&mut self, node: usize, start: usize, end: usize, val: i64) {
        if val <= self.data[node].min_first {
            return;
        }

        if start == end {
            self.data[node].min_first = val;
            self.data[node].max_first = val;
            self.data[node].sum = val;
            return;
        }

        self.data[node].sum += self.data[node].num_min_first * (val - self.data[node].min_first);
        self.data[node].min_first = val;

        if val >= self.data[node].max_first {
            self.data[node].max_first = val;
        } else if val > self.data[node].max_second {
            self.data[node].max_second = val;
        }
    }

    fn process_add(&mut self, node: usize, start: usize, end: usize, val: i64) {
        if val == 0 {
            return;
        }

        self.data[node].min_first += val;
        self.data[node].max_first += val;
        self.data[node].sum += (end - start + 1) as i64 * val;

        if self.data[node].min_second != MAX {
            self.data[node].min_second += val;
        }

        if self.data[node].max_second != MIN {
            self.data[node].max_second += val;
        }

        self.lazy_add[node] += val;
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if start == end {
            return;
        }

        let mid = (start + end) / 2;

        self.process_add(node * 2, start, mid, self.lazy_add[node]);
        self.process_add(node * 2 + 1, mid + 1, end, self.lazy_add[node]);

        self.lazy_add[node] = 0;

        self.process_max(node * 2, start, mid, self.data[node].min_first);
        self.process_max(node * 2 + 1, mid + 1, end, self.data[node].min_first);

        self.process_min(node * 2, start, mid, self.data[node].max_first);
        self.process_min(node * 2 + 1, mid + 1, end, self.data[node].max_first);
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
        if end < node_start || node_end < start || val >= self.data[node].max_first {
            return;
        }

        if start <= node_start && node_end <= end && val > self.data[node].max_second {
            self.process_min(node, node_start, node_end, val);
            return;
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;
        self.update_min_internal(start, end, val, node * 2, node_start, mid);
        self.update_min_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = LazySegmentTree::merge(&self.data[node * 2], &self.data[node * 2 + 1]);
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
        if end < node_start || node_end < start || val <= self.data[node].min_first {
            return;
        }

        if start <= node_start && node_end <= end && val < self.data[node].min_second {
            self.process_max(node, node_start, node_end, val);
            return;
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;
        self.update_max_internal(start, end, val, node * 2, node_start, mid);
        self.update_max_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = LazySegmentTree::merge(&self.data[node * 2], &self.data[node * 2 + 1]);
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
            self.process_add(node, node_start, node_end, val);
            return;
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;
        self.update_add_internal(start, end, val, node * 2, node_start, mid);
        self.update_add_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = LazySegmentTree::merge(&self.data[node * 2], &self.data[node * 2 + 1]);
    }

    pub fn query_min(&mut self, start: usize, end: usize) -> i64 {
        self.query_min_internal(start, end, 1, 1, self.size)
    }

    fn query_min_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> i64 {
        if end < node_start || node_end < start {
            return i64::MAX;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].min_first;
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;
        let left = self.query_min_internal(start, end, node * 2, node_start, mid);
        let right = self.query_min_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.min(right)
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
        if end < node_start || node_end < start {
            return 0;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].max_first;
        }

        self.propagate(node, node_start, node_end);

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
        if end < node_start || node_end < start {
            return 0;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].sum;
        }

        self.propagate(node, node_start, node_end);

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
            tree.update_min(l, r, x);
        } else if command == 2 {
            let (l, r, x) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            tree.update_max(l, r, x);
        } else if command == 3 {
            let (l, r, x) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            tree.update_add(l, r, x);
        } else if command == 4 {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(out, "{}", tree.query_min(l, r)).unwrap();
        } else if command == 5 {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(out, "{}", tree.query_max(l, r)).unwrap();
        } else {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(out, "{}", tree.query_sum(l, r)).unwrap();
        }
    }
}

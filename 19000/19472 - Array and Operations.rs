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
    min: i64,
    max: i64,
    sum: i64,
}

struct LazySegmentTree {
    size: usize,
    data: Vec<Node>,
    lazy_add: Vec<i64>,
    lazy_sqrt: Vec<i64>,
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
            lazy_sqrt: vec![0; real_n * 4],
        }
    }

    fn merge(a: &Node, b: &Node) -> Node {
        Node {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
            sum: a.sum + b.sum,
        }
    }

    pub fn construct(&mut self, arr: &[i64], start: usize, end: usize) {
        self.construct_internal(arr, 1, start, end);
    }

    fn construct_internal(&mut self, arr: &[i64], node: usize, start: usize, end: usize) -> Node {
        if start == end {
            self.data[node] = Node {
                min: arr[start],
                max: arr[start],
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
        if self.lazy_add[node] == 0 && self.lazy_sqrt[node] == 0 {
            return;
        }

        if self.lazy_sqrt[node] == 0 {
            self.data[node].min += self.lazy_add[node];
            self.data[node].max += self.lazy_add[node];
            self.data[node].sum += self.lazy_add[node] * (end - start + 1) as i64;

            if start != end {
                self.lazy_add[node * 2] += self.lazy_add[node];
                self.lazy_add[node * 2 + 1] += self.lazy_add[node];
            }
        } else {
            self.data[node].min = self.lazy_add[node] + self.lazy_sqrt[node];
            self.data[node].max = self.lazy_add[node] + self.lazy_sqrt[node];
            self.data[node].sum =
                (self.lazy_add[node] + self.lazy_sqrt[node]) * (end - start + 1) as i64;

            if start != end {
                self.lazy_add[node * 2] = self.lazy_add[node];
                self.lazy_add[node * 2 + 1] = self.lazy_add[node];
                self.lazy_sqrt[node * 2] = self.lazy_sqrt[node];
                self.lazy_sqrt[node * 2 + 1] = self.lazy_sqrt[node];
            }
        }

        self.lazy_add[node] = 0;
        self.lazy_sqrt[node] = 0;
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
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return;
        }

        if start <= node_start && node_end <= end {
            self.lazy_add[node] = val;
            self.propagate(node, node_start, node_end);
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_add_internal(start, end, val, node * 2, node_start, mid);
        self.update_add_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = LazySegmentTree::merge(&self.data[node * 2], &self.data[node * 2 + 1]);
    }

    pub fn update_sqrt(&mut self, start: usize, end: usize) {
        self.update_sqrt_internal(start, end, 1, 1, self.size);
    }

    fn update_sqrt_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return;
        }

        if start <= node_start && node_end <= end {
            if (self.data[node].min as f64).sqrt().floor() as i64
                == (self.data[node].max as f64).sqrt().floor() as i64
            {
                self.lazy_sqrt[node] = (self.data[node].max as f64).sqrt().floor() as i64;
                self.propagate(node, node_start, node_end);
                return;
            }

            if self.data[node].min + 1 == self.data[node].max {
                self.lazy_add[node] =
                    (self.data[node].min as f64).sqrt().floor() as i64 - self.data[node].min;
                self.propagate(node, node_start, node_end);
                return;
            }
        }

        let mid = (node_start + node_end) / 2;
        self.update_sqrt_internal(start, end, node * 2, node_start, mid);
        self.update_sqrt_internal(start, end, node * 2 + 1, mid + 1, node_end);

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
            return 0;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].sum;
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left + right
    }
}

// Reference: https://justicehui.github.io/hard-algorithm/2019/10/10/segment-tree-beats/
// Reference: https://justicehui.github.io/ps/2019/10/29/BOJ17476/
// Reference: https://www.secmem.org/blog/2019/10/19/Segment-Tree-Beats/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    let mut tree = LazySegmentTree::new(n);
    let mut arr = vec![0; n + 1];

    for i in 1..=n {
        arr[i] = scan.token::<i64>();
    }

    tree.construct(&arr, 1, n);

    for _ in 0..m {
        let command = scan.token::<usize>();

        if command == 1 {
            let (l, r, x) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            tree.update_add(l, r, x);
        } else if command == 2 {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            tree.update_sqrt(l, r);
        } else {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(out, "{}", tree.query(l, r)).unwrap();
        }
    }
}

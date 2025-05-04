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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const INF: i64 = 9_000_000_000_000_000_000;

#[derive(Clone, Debug)]
struct Node {
    sum_min: i64,
    sum_max: i64,
    diff_max: i64,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            sum_min: INF,
            sum_max: -INF,
            diff_max: -INF,
        }
    }
}

impl Node {
    fn new(val: i64) -> Self {
        Node {
            sum_min: val,
            sum_max: val,
            diff_max: -INF,
        }
    }

    fn merge(&self, other: &Self) -> Self {
        Node {
            sum_min: self.sum_min.min(other.sum_min),
            sum_max: self.sum_max.max(other.sum_max),
            diff_max: self
                .diff_max
                .max(other.diff_max)
                .max(other.sum_max - self.sum_min),
        }
    }
}

struct SegmentTree {
    size: usize,
    data: Vec<Node>,
}

impl SegmentTree {
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

    pub fn construct(&mut self, arr: &[i64], start: usize, end: usize) {
        self.construct_internal(arr, 1, start, end);
    }

    fn construct_internal(&mut self, arr: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = Node::new(arr[start]);
            return;
        }

        let mid = (start + end) / 2;

        self.construct_internal(arr, node * 2, start, mid);
        self.construct_internal(arr, node * 2 + 1, mid + 1, end);

        let left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();

        self.data[node] = left.merge(&right);
    }

    pub fn query(&mut self, start: usize, end: usize) -> Node {
        self.query_internal(start, end, 1, 0, self.size - 1)
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
            return self.data[node].clone();
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.merge(&right)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut prefix_sum = vec![0; n + 1];

    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + nums[i];
    }

    let mut tree = SegmentTree::new(n + 1);
    tree.construct(&prefix_sum, 0, n);

    let m = scan.token::<usize>();

    for _ in 0..m {
        let x1 = scan.token::<usize>();
        let y1 = scan.token::<usize>();
        let x2 = scan.token::<usize>();
        let y2 = scan.token::<usize>();

        let mut ret = -INF;

        if x2 <= y1 {
            let left1 = tree.query(x1 - 1, x2 - 1);
            let right1 = tree.query(x2, y1);
            let val1 = (right1.sum_max - left1.sum_min).max(right1.diff_max);

            ret = ret.max(val1);

            if y1 < y2 {
                let left2 = tree.query(x1 - 1, y1 - 1);
                let right2 = tree.query(y1 + 1, y2);
                let val2 = right2.sum_max - left2.sum_min;

                ret = ret.max(val2);
            }
        } else {
            let left = tree.query(x1 - 1, y1 - 1);
            let right = tree.query(x2, y2);
            let val = right.sum_max - left.sum_min;

            ret = ret.max(val);
        }

        writeln!(out, "{ret}").unwrap();
    }
}

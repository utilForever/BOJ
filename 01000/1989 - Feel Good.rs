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

#[derive(Clone, Debug)]
struct Node {
    sum: i64,
    min: (usize, i64),
}

impl Node {
    fn new() -> Self {
        Self {
            sum: 0,
            min: (0, i64::MAX),
        }
    }

    fn merge(&mut self, other: &Self) -> Node {
        let mut ret = Node::new();
        ret.sum = self.sum + other.sum;
        ret.min = if self.min.1 < other.min.1 {
            self.min
        } else {
            other.min
        };

        ret
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
            data: vec![Node::new(); real_n * 4],
        }
    }

    pub fn update(&mut self, index: usize, val: i64) {
        self.update_internal(index, val, 1, 1, self.size);
    }

    fn update_internal(
        &mut self,
        index: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if index > node_end || index < node_start {
            return;
        }

        if node_start == node_end {
            self.data[node] = Node {
                sum: val,
                min: (index, val),
            };
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(index, val, node * 2, node_start, mid);
        self.update_internal(index, val, node * 2 + 1, mid + 1, node_end);

        let mut left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();
        self.data[node] = left.merge(&right);
    }

    fn query(&mut self, start: usize, end: usize) -> (Node, usize, usize) {
        self.query_internal(start, end, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> (Node, usize, usize) {
        if end < node_start || node_end < start {
            return (Node::new(), 0, 0);
        }

        if start <= node_start && node_end <= end {
            return (self.data[node].clone(), node_start, node_end);
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        (
            Node {
                sum: left.0.sum + right.0.sum,
                min: if left.0.min.1 < right.0.min.1 {
                    left.0.min
                } else {
                    right.0.min
                },
            },
            node_start,
            node_end,
        )
    }
}

fn calculate_max(tree: &mut SegmentTree, left: usize, right: usize) -> (i64, usize, usize) {
    let node = tree.query(left, right);
    let mut ret = (node.0.sum * node.0.min.1, left, right);

    if left < node.0.min.0 {
        let ret_left = calculate_max(tree, left, node.0.min.0 - 1);

        if ret_left.0 > ret.0 {
            ret = ret_left;
        }
    }

    if node.0.min.0 < right {
        let ret_right = calculate_max(tree, node.0.min.0 + 1, right);

        if ret_right.0 > ret.0 {
            ret = ret_right;
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut tree = SegmentTree::new(n);
    let mut arr = vec![0; n + 1];

    for i in 1..=n {
        arr[i] = scan.token::<i64>();
        tree.update(i, arr[i]);
    }

    let ret = calculate_max(&mut tree, 1, n);
    writeln!(out, "{}", ret.0).unwrap();
    writeln!(out, "{} {}", ret.1, ret.2).unwrap();
}

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
}

impl Node {
    fn new(val: i64) -> Self {
        Self { sum: val }
    }

    fn merge(&mut self, other: &Self) -> Node {
        let mut ret = Node::new(0);
        ret.sum = self.sum + other.sum;

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
            data: vec![Node::new(0); real_n * 4],
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
            self.data[node] = Node::new(val);
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(index, val, node * 2, node_start, mid);
        self.update_internal(index, val, node * 2 + 1, mid + 1, node_end);

        let mut left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();
        self.data[node] = left.merge(&right);
    }

    fn query(&mut self, start: usize, end: usize) -> Node {
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
            return Node { sum: 0 };
        }

        if start <= node_start && node_end <= end {
            return self.data[node].clone();
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        let sum = left.sum + right.sum;

        Node { sum }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = vec![0; n + 1];
    let mut tree_odd = SegmentTree::new((n + 1) / 2);
    let mut tree_even = SegmentTree::new(n / 2);

    for i in 1..=n {
        let val = scan.token::<i64>();
        nums[i] = val;

        if i % 2 == 1 {
            tree_odd.update((i + 1) / 2, val);
        } else {
            tree_even.update(i / 2, val);
        }
    }

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            let (sum_positive, sum_negative) = if l % 2 == 1 {
                (
                    tree_odd.query((l + 1) / 2, (r + 1) / 2).sum,
                    tree_even.query((l + 1) / 2, r / 2).sum,
                )
            } else {
                (
                    tree_even.query(l / 2, r / 2).sum,
                    tree_odd.query(l / 2 + 1, (r + 1) / 2).sum,
                )
            };

            writeln!(out, "{}", (sum_positive - sum_negative).abs()).unwrap();
        } else {
            let (l, x) = (scan.token::<usize>(), scan.token::<i64>());
            nums[l] += x;

            if l % 2 == 1 {
                tree_odd.update((l + 1) / 2, nums[l]);
            } else {
                tree_even.update(l / 2, nums[l]);
            }
        }
    }
}

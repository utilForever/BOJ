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

#[derive(Debug, Clone, Default)]
struct Node {
    val: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self { val }
    }

    fn merge(&self, other: &Self) -> Node {
        Node {
            val: self.val.max(other.val),
        }
    }
}

const NEG_INF: i64 = -1_000_000_000;

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
            data: vec![Node::new(NEG_INF); real_n * 4],
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

        let left = self.data[node * 2].clone();
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
            return Node::new(NEG_INF);
        }

        if start <= node_start && node_end <= end {
            return self.data[node].clone();
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        Node::merge(&left, &right)
    }
}

fn lower_bound(v: &Vec<i64>, x: i64) -> usize {
    let mut left = 0;
    let mut right = v.len();

    while left < right {
        let mid = (left + right) >> 1;

        if v[mid] < x {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut prefix_sum = vec![0; n + 1];

    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
    }

    let mut compressed = prefix_sum.clone();
    compressed.sort_unstable();
    compressed.dedup();

    let mut tree = SegmentTree::new(compressed.len());
    let mut dp = vec![0; n + 2];
    let mut ret = 0;

    for i in (1..=n).rev() {
        let pos = lower_bound(&compressed, prefix_sum[i]);
        tree.update(pos + 1, dp[i + 1]);

        let t = prefix_sum[i - 1] + k;
        let pos = lower_bound(&compressed, t);

        let val = if pos >= compressed.len() {
            NEG_INF
        } else {
            tree.query(pos + 1, compressed.len()).val
        };

        dp[i] = if val < 0 { 0 } else { val + 1 };
        ret = ret.max(dp[i]);
    }

    writeln!(out, "{ret}").unwrap();
}

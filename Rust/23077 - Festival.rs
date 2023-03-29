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
    count: i64,
    sum: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self { count: 0, sum: val }
    }

    fn merge(&mut self, other: &Self) -> Node {
        let mut ret = Node::new(0);
        ret.sum = self.sum + other.sum;
        ret.count = self.count + other.count;

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
        self.update_internal(index, val, 1, 0, self.size - 1);
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
            self.data[node].sum += val;
            self.data[node].count += if val > 0 { 1 } else { -1 };
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(index, val, node * 2, node_start, mid);
        self.update_internal(index, val, node * 2 + 1, mid + 1, node_end);

        let mut left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();
        self.data[node] = left.merge(&right);
    }

    fn query(&mut self, limit: i64) -> i64 {
        let idx = self.query_internal_index(1, 0, self.size - 1, limit);
        let count = self.query_internal_count(idx as usize + 1, self.size, 1, 0, self.size - 1);
        let sum = self.query_internal_sum(idx as usize + 1, self.size, 1, 0, self.size - 1);

        sum + (limit - count) * idx
    }

    fn query_internal_index(
        &mut self,
        node: usize,
        node_start: usize,
        node_end: usize,
        limit: i64,
    ) -> i64 {
        if node_start == node_end {
            return node_start as i64;
        }

        let mid = (node_start + node_end) / 2;

        if self.data[node * 2 + 1].count < limit {
            self.query_internal_index(
                node * 2,
                node_start,
                mid,
                limit - self.data[node * 2 + 1].count,
            )
        } else {
            self.query_internal_index(node * 2 + 1, mid + 1, node_end, limit)
        }
    }

    fn query_internal_count(
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
            return self.data[node].count;
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal_count(start, end, node * 2, node_start, mid);
        let right = self.query_internal_count(start, end, node * 2 + 1, mid + 1, node_end);

        left + right
    }

    fn query_internal_sum(
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

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal_sum(start, end, node * 2, node_start, mid);
        let right = self.query_internal_sum(start, end, node * 2 + 1, mid + 1, node_end);

        left + right
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let (_, n, k) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        let mut attractions = vec![(0, 0); 2 * n];

        for i in 0..n {
            let (h, s, e) = (
                scan.token::<i64>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
            );
            attractions[2 * i] = (h, s);
            attractions[2 * i + 1] = (-h, e);
        }

        attractions.sort_by(|a, b| {
            if a.1 == b.1 {
                b.0.cmp(&a.0)
            } else {
                a.1.cmp(&b.1)
            }
        });

        let mut tree = SegmentTree::new(300_001);
        let mut ret = 0;

        for (attraction, _) in attractions {
            let idx_attraction = attraction.unsigned_abs() as usize;
            tree.update(idx_attraction, attraction);
            ret = ret.max(tree.query(k));
        }

        writeln!(out, "Case #{i}: {ret}").unwrap();
    }
}

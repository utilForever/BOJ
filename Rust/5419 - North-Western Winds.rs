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
    cnt: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self { cnt: val }
    }

    fn merge(&mut self, other: &Self) -> Node {
        let mut ret = Node::new(0);
        ret.cnt = self.cnt + other.cnt;

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
            self.data[node].cnt += val;
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
            return Node { cnt: 0 };
        }

        if start <= node_start && node_end <= end {
            return self.data[node].clone();
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        let cnt = left.cnt + right.cnt;

        Node { cnt }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut points = vec![(0, 0); n];

        for i in 0..n {
            points[i] = (scan.token::<i64>(), scan.token::<i64>());
        }

        points.sort_by(|a, b| a.1.cmp(&b.1));

        let mut idx_points = vec![0; 75000];
        let mut idx = 1;

        for i in 0..n {
            if i != 0 && points[i].1 != points[i - 1].1 {
                idx += 1;
            }

            idx_points[i] = idx;
        }

        for i in 0..n {
            points[i].1 = idx_points[i];
        }

        points.sort_by(|a, b| {
            if a.0 == b.0 {
                b.1.cmp(&a.1)
            } else {
                a.0.cmp(&b.0)
            }
        });

        let mut tree = SegmentTree::new(n + 1);
        let mut ret = 0;

        for i in 0..n {
            ret += tree.query(points[i].1 as usize, n).cnt;
            tree.update(points[i].1 as usize, 1);
        }

        writeln!(out, "{ret}").unwrap();
    }
}

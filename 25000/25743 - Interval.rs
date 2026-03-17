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
    sum: i64,
    prefix_sum: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self {
            sum: val,
            prefix_sum: val.max(0),
        }
    }

    fn merge(&self, other: &Self) -> Node {
        Node {
            sum: self.sum + other.sum,
            prefix_sum: self.prefix_sum.max(self.sum + other.prefix_sum),
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

        let left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();
        self.data[node] = left.merge(&right);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut intervals = vec![(0, 0); n];
    let mut coords = Vec::with_capacity(2 * n);

    for i in 0..n {
        let (l, r) = (scan.token::<i64>(), scan.token::<i64>());

        intervals[i] = (l, r);
        coords.push(l);
        coords.push(r + 1);
    }

    coords.sort_unstable();
    coords.dedup();

    let mut intervals = intervals
        .into_iter()
        .map(|(l, r)| {
            let idx_l = coords.binary_search(&l).unwrap() + 1;
            let idx_r = coords.binary_search(&(r + 1)).unwrap() + 1;

            (r - l, idx_l, idx_r)
        })
        .collect::<Vec<_>>();

    intervals.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut tree = SegmentTree::new(coords.len());
    let mut diff = vec![0; coords.len() + 1];

    let mut idx_right = 0;
    let mut ret = i64::MAX;

    for idx_left in 0..n {
        while idx_right < n && tree.data[1].prefix_sum < m {
            let (_, idx_l, idx_r) = intervals[idx_right];

            diff[idx_l] += 1;
            tree.update(idx_l, diff[idx_l]);

            diff[idx_r] -= 1;
            tree.update(idx_r, diff[idx_r]);

            idx_right += 1;
        }

        if tree.data[1].prefix_sum < m {
            break;
        }

        ret = ret.min(intervals[idx_right - 1].0 - intervals[idx_left].0);

        let (_, idx_l, idx_r) = intervals[idx_left];

        diff[idx_l] -= 1;
        tree.update(idx_l, diff[idx_l]);

        diff[idx_r] += 1;
        tree.update(idx_r, diff[idx_r]);
    }

    writeln!(out, "{}", if ret == i64::MAX { -1 } else { ret }).unwrap();
}

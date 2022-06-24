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
    max: i64,
    min: i64,
    num_max: i64,
    num_min: i64,
    mine: i64,
    num_mine: i64,
}

struct LazySegmentTree {
    size: usize,
    k: usize,
    data: Vec<Node>,
}

impl LazySegmentTree {
    pub fn new(n: usize, k: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n - k + 1,
            k,
            data: vec![Node::default(); real_n * 4],
        }
    }

    fn merge(a: &Node, b: &Node, k: usize) -> Node {
        let mut ret: Node = Default::default();

        if a.max < b.max {
            ret.max = b.max;
            ret.num_max = b.num_max;
        } else if a.max > b.max {
            ret.max = a.max;
            ret.num_max = a.num_max;
        } else {
            ret.max = a.max;
            ret.num_max = a.num_max + b.num_max;
        }

        if a.min < b.min {
            ret.min = a.min;
            ret.num_min = a.num_min;
        } else if a.min > b.min {
            ret.min = b.min;
            ret.num_min = b.num_min;
        } else {
            ret.min = a.min;
            ret.num_min = a.num_min + b.num_min;
        }

        if ret.min * 2 == k as i64 {
            ret.num_mine = ret.num_min;
        } else if ret.max * 2 == k as i64 {
            ret.num_mine = ret.num_max;
        }

        ret
    }

    pub fn construct(&mut self, start: usize, end: usize) {
        self.construct_internal(1, start, end);
    }

    fn construct_internal(&mut self, node: usize, start: usize, end: usize) -> Node {
        if start == end {
            self.data[node] = Node {
                max: 0,
                min: 0,
                num_max: 1,
                num_min: 1,
                mine: 0,
                num_mine: 1,
            };
            self.data[node].clone()
        } else {
            let mid = (start + end) / 2;

            let left = self.construct_internal(node * 2, start, mid);
            let right = self.construct_internal(node * 2 + 1, mid + 1, end);

            self.data[node] = LazySegmentTree::merge(&left, &right, self.k);
            self.data[node].clone()
        }
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        self.data[node].max += self.data[node].mine;
        self.data[node].min += self.data[node].mine;

        if start != end {
            self.data[node * 2].mine += self.data[node].mine;
            self.data[node * 2 + 1].mine += self.data[node].mine;
        }

        self.data[node].mine = 0;
    }

    pub fn update(&mut self, start: usize, end: usize, val: i64) {
        self.update_internal(start, end, val, 1, 1, self.size);
    }

    fn update_internal(
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
            self.data[node].mine += val;
            self.propagate(node, node_start, node_end);
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(start, end, val, node * 2, node_start, mid);
        self.update_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] =
            LazySegmentTree::merge(&self.data[node * 2], &self.data[node * 2 + 1], self.k);
    }
}

fn query(tree1: &LazySegmentTree, tree2: &LazySegmentTree, k: i64) -> Vec<(i64, i64)> {
    let node_x = tree1.data[1].clone();
    let node_y = tree2.data[1].clone();
    let max_x = k - 2 * node_x.max;
    let max_y = k - 2 * node_y.max;
    let min_x = k - 2 * node_x.min;
    let min_y = k - 2 * node_y.min;

    let mut ret = Vec::new();
    ret.push((max_x * max_y, node_x.num_max * node_y.num_max));

    if max_y != min_y {
        ret.push((max_x * min_y, node_x.num_max * node_y.num_min));
    }

    if max_x != min_x {
        ret.push((min_x * max_y, node_x.num_min * node_y.num_max));
    }

    if max_x != min_x && max_y != min_y {
        ret.push((min_x * min_y, node_x.num_min * node_y.num_min));
    }

    ret.sort();

    ret
}

// Reference: https://justicehui.github.io/hard-algorithm/2019/10/10/segment-tree-beats/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    let mut tree1 = LazySegmentTree::new(n, k);
    let mut tree2 = LazySegmentTree::new(n, k);
    let mut pos_x = vec![0; n + 1];
    let mut pos_y = vec![0; n + 1];

    tree1.construct(1, n - k + 1);
    tree2.construct(1, n - k + 1);

    for _ in 0..q {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

        if a == 1 {
            pos_x[b] = 1 - pos_x[b];
            tree1.update(
                if b >= k { b - k + 1 } else { 1 },
                b,
                if pos_x[b] == 1 { 1 } else { -1 },
            );
        } else if a == 2 {
            pos_y[b] = 1 - pos_y[b];
            tree2.update(
                if b >= k { b - k + 1 } else { 1 },
                b,
                if pos_y[b] == 1 { 1 } else { -1 },
            );
        }

        let ret = query(&tree1, &tree2, k as i64);

        if ret[0].0 == 0 {
            writeln!(
                out,
                "{} {}",
                k * k / 2,
                (tree1.data[1].num_mine + tree2.data[1].num_mine) * (n - k + 1) as i64
                    - tree1.data[1].num_mine * tree2.data[1].num_mine
            )
            .unwrap();
        } else {
            let mut sum = 0;

            for val in ret.iter() {
                if val.0 == ret[0].0 {
                    sum += val.1;
                }
            }

            writeln!(out, "{} {}", ((k * k) as i64 - ret[0].0) / 2, sum).unwrap();
        }
    }
}

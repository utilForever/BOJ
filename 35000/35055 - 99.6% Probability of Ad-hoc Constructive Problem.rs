use io::Write;
use std::{cmp::Ordering, collections::BTreeSet, io, str};

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NodeType {
    UpdateEdge,
    Query,
}

#[derive(Clone, Copy)]
struct Node {
    x: i64,
    y: i64,
    z: usize,
    r#type: NodeType,
    val: i64,
    query_id: usize,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            r#type: NodeType::UpdateEdge,
            val: 0,
            query_id: 0,
        }
    }
}

impl Node {
    fn new(x: i64, y: i64, z: usize, r#type: NodeType, val: i64, query_id: usize) -> Self {
        Self {
            x,
            y,
            z,
            r#type,
            val,
            query_id,
        }
    }
}

struct FenwickTree {
    n: usize,
    version: u64,
    bit: Vec<i64>,
    visited: Vec<u64>,
    updated: Vec<usize>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            version: 1,
            bit: vec![INF; n + 1],
            visited: vec![0; n + 1],
            updated: Vec::new(),
        }
    }

    #[inline]
    fn update(&mut self, mut idx: usize, val: i64) {
        while idx <= self.n {
            if val < self.bit[idx] {
                self.bit[idx] = val;

                if self.visited[idx] != self.version {
                    self.visited[idx] = self.version;
                    self.updated.push(idx);
                }
            }

            idx += idx & (!idx + 1);
        }
    }

    #[inline]
    fn query(&self, mut idx: usize) -> i64 {
        let mut ret = INF;

        while idx > 0 {
            ret = ret.min(self.bit[idx]);
            idx -= idx & (!idx + 1);
        }

        ret
    }

    #[inline]
    fn clear(&mut self) {
        for &idx in self.updated.iter() {
            self.bit[idx] = INF;
        }

        self.updated.clear();
        self.version = self.version.wrapping_add(1);

        if self.version == 0 {
            self.version = 1;
            self.visited.fill(0);
        }
    }
}

fn cdq(
    nodes: &mut Vec<Node>,
    temp: &mut Vec<Node>,
    fenwick_tree: &mut FenwickTree,
    ret: &mut Vec<i64>,
    left: usize,
    right: usize,
) {
    if left >= right {
        return;
    }

    let mid = (left + right) / 2;

    cdq(nodes, temp, fenwick_tree, ret, left, mid);
    cdq(nodes, temp, fenwick_tree, ret, mid + 1, right);

    let mut i = left;
    let mut j = mid + 1;
    let mut idx = left;

    while i <= mid && j <= right {
        let ley_left = (nodes[i].y, nodes[i].r#type);
        let key_right = (nodes[j].y, nodes[j].r#type);

        if ley_left <= key_right {
            if nodes[i].r#type == NodeType::UpdateEdge {
                fenwick_tree.update(nodes[i].z, nodes[i].val);
            }

            temp[idx] = nodes[i];
            i += 1;
        } else {
            if nodes[j].r#type == NodeType::Query {
                let val = fenwick_tree.query(nodes[j].z);
                let id = nodes[j].query_id;

                ret[id] = ret[id].min(val);
            }

            temp[idx] = nodes[j];
            j += 1;
        }

        idx += 1;
    }

    while j <= right {
        if nodes[j].r#type == NodeType::Query {
            let val = fenwick_tree.query(nodes[j].z);
            let id = nodes[j].query_id;

            ret[id] = ret[id].min(val);
        }

        temp[idx] = nodes[j];
        j += 1;
        idx += 1;
    }

    while i <= mid {
        if nodes[i].r#type == NodeType::UpdateEdge {
            fenwick_tree.update(nodes[i].z, nodes[i].val);
        }

        temp[idx] = nodes[i];
        i += 1;
        idx += 1;
    }

    fenwick_tree.clear();

    nodes[left..right + 1].copy_from_slice(&temp[left..right + 1]);
}

const INF: i64 = 1_000_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![0; n + 1];

    for i in 1..=n {
        nums[i] = scan.token::<i64>();
    }

    let mut idxes = (1..=n).collect::<Vec<_>>();

    idxes.sort_unstable_by(|&i, &j| {
        if nums[i] != nums[j] {
            nums[j].cmp(&nums[i])
        } else {
            i.cmp(&j)
        }
    });

    let mut set: BTreeSet<usize> = BTreeSet::new();
    let mut edges = Vec::with_capacity(n * 2);

    for &idx in idxes.iter() {
        if let Some(&prev) = set.range(..idx).next_back() {
            let i = prev.min(idx);
            let j = prev.max(idx);

            edges.push((i, j, nums[i] + nums[j], (j - i) as i64));
        }

        if let Some(&next) = set.range((idx + 1)..).next() {
            let i = next.min(idx);
            let j = next.max(idx);

            edges.push((i, j, nums[i] + nums[j], (j - i) as i64));
        }

        set.insert(idx);
    }

    let mut nodes = Vec::with_capacity(edges.len() + q);

    for (i, j, sum, dist) in edges {
        nodes.push(Node::new(
            -sum,
            -(i as i64),
            j,
            NodeType::UpdateEdge,
            dist,
            0,
        ));
    }

    for i in 0..q {
        let (l, r, k) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        nodes.push(Node::new(-k, -(l as i64), r, NodeType::Query, 0, i));
    }

    nodes.sort_unstable_by(|a, b| {
        if a.x.cmp(&b.x) != Ordering::Equal {
            a.x.cmp(&b.x)
        } else {
            a.r#type.cmp(&b.r#type)
        }
    });

    let mut fenwick_tree = FenwickTree::new(n);
    let mut temp = vec![Node::default(); nodes.len()];
    let len = nodes.len();
    let mut ret = vec![INF; q];

    cdq(
        &mut nodes,
        &mut temp,
        &mut fenwick_tree,
        &mut ret,
        0,
        len - 1,
    );

    for i in 0..q {
        writeln!(out, "{}", if ret[i] == INF { -1 } else { ret[i] }).unwrap();
    }
}

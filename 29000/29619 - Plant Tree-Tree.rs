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
    val: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self { val }
    }

    fn merge(&mut self, other: &Self) -> Node {
        let mut ret = Node::new(0);
        ret.val = self.val + other.val;

        ret
    }
}

#[derive(Clone)]
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
            self.data[node].val += val;
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
            return Node { val: 0 };
        }

        if start <= node_start && node_end <= end {
            return self.data[node].clone();
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        let val = left.val + right.val;

        Node { val }
    }
}

#[derive(Clone)]
enum Query {
    Graft(usize, usize, i64),
    Harvest(usize),
}

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    idxes_in: &mut Vec<usize>,
    idxes_out: &mut Vec<usize>,
    idx: &mut i64,
    curr: usize,
) {
    *idx += 1;
    idxes_in[curr] = *idx as usize;

    for next in graph[curr].iter() {
        process_dfs(graph, idxes_in, idxes_out, idx, *next);
    }

    idxes_out[curr] = *idx as usize;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); 200_001];
    let mut weights = vec![0; n + 1];
    let mut num_vertices = n;

    for i in 1..=n {
        let parent = scan.token::<i64>();

        if parent != -1 {
            graph[parent as usize].push(i);
        }
    }

    for i in 1..=n {
        weights[i] = scan.token::<i64>();
    }

    let mut queries = Vec::new();

    for _ in 0..m {
        let command = scan.token::<i64>();

        if command == 1 {
            let (i, j, w) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            queries.push(Query::Graft(i, j, w));

            graph[i].push(j);
            num_vertices = num_vertices.max(i).max(j);
        } else {
            let i = scan.token::<usize>();
            queries.push(Query::Harvest(i));
        }
    }

    let mut idxes_in = vec![0; num_vertices + 1];
    let mut idxes_out = vec![0; num_vertices + 1];
    let mut idx = -1;

    process_dfs(&graph, &mut idxes_in, &mut idxes_out, &mut idx, 1);

    let mut tree = SegmentTree::new(num_vertices + 1);

    for i in 1..=n {
        tree.update(idxes_in[i], weights[i]);
    }

    for query in queries {
        match query {
            Query::Graft(_, j, w) => {
                tree.update(idxes_in[j], w);
            }
            Query::Harvest(i) => {
                let ret = tree.query(idxes_in[i], idxes_out[i]);
                writeln!(out, "{}", if ret.val == 0 { -1 } else { ret.val }).unwrap();
            }
        }
    }
}

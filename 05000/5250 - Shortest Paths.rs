use io::Write;
use std::{collections::BinaryHeap, io, str};

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

const MAX: i64 = 1_000_000_000;

struct LazySegmentTree {
    size: usize,
    data: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![MAX; real_n * 4],
            lazy: vec![MAX; real_n * 4],
        }
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] == MAX {
            return;
        }

        if start != end {
            self.lazy[node * 2] = self.lazy[node * 2].min(self.lazy[node]);
            self.lazy[node * 2 + 1] = self.lazy[node * 2 + 1].min(self.lazy[node]);
        }

        self.data[node] = self.data[node].min(self.lazy[node]);
        self.lazy[node] = MAX;
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
            self.lazy[node] = self.lazy[node].min(val);
            self.propagate(node, node_start, node_end);
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(start, end, val, node * 2, node_start, mid);
        self.update_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = self.data[node * 2].min(self.data[node * 2 + 1]);
    }

    pub fn query(&mut self, start: usize, end: usize) -> i64 {
        self.query_internal(start, end, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> i64 {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return MAX;
        }

        if start <= node_start && node_end <= end {
            return self.data[node];
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.min(right)
    }
}

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Vec<i64> {
    let mut ret = vec![MAX; graph.len()];
    ret[from] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, from));

    while !queue.is_empty() {
        let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
        cost_curr *= -1;

        if ret[vertex_curr] < cost_curr {
            continue;
        }

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;

            cost_next += cost_curr;

            if ret[vertex_next] > cost_next {
                ret[vertex_next] = cost_next;
                queue.push((-cost_next, vertex_next));
            }
        }
    }

    ret
}

fn process_dfs(
    parent: &mut Vec<usize>,
    graph: &Vec<Vec<(usize, i64)>>,
    lucky_paths_inv: &Vec<usize>,
    dist: &Vec<i64>,
    curr: usize,
    mut from: usize,
) {
    if parent[curr] != 0 {
        return;
    }

    if lucky_paths_inv[curr] != 0 {
        from = curr;
    }

    parent[curr] = from;

    for &(next, cost) in graph[curr].iter() {
        if dist[curr] + cost != dist[next] {
            continue;
        }

        if lucky_paths_inv[curr] == 0 && lucky_paths_inv[next] != 0 {
            continue;
        }

        process_dfs(parent, graph, lucky_paths_inv, dist, next, from);
    }
}

// Reference: https://usaco.guide/problems/balkan-oi-2012shortest-paths/solution
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, a, b) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (u, v, w) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let k = scan.token::<usize>();
    let mut lucky_paths = vec![0; n + 1];
    let mut lucky_paths_inv = vec![0; n + 1];

    for i in 1..=k {
        lucky_paths[i] = scan.token::<usize>();
        lucky_paths_inv[lucky_paths[i]] = i;
    }

    let dist_from_a = process_dijkstra(&graph, a);
    let dist_from_b = process_dijkstra(&graph, b);

    let mut parent_a = vec![0; n + 1];
    let mut parent_b = vec![0; n + 1];

    process_dfs(
        &mut parent_a,
        &graph,
        &lucky_paths_inv,
        &dist_from_a,
        lucky_paths[1],
        lucky_paths[1],
    );
    process_dfs(
        &mut parent_b,
        &graph,
        &lucky_paths_inv,
        &dist_from_b,
        lucky_paths[k],
        lucky_paths[k],
    );

    let mut tree = LazySegmentTree::new(n + 1);

    for i in 1..=n {
        for &(j, cost) in graph[i].iter() {
            if lucky_paths_inv[i] != 0
                && lucky_paths_inv[j] != 0
                && lucky_paths_inv[j] - lucky_paths_inv[i] == 1
            {
                continue;
            }

            let start = lucky_paths_inv[parent_a[i]];
            let end = lucky_paths_inv[parent_b[j]] - 1;
            let val = dist_from_a[i] + cost + dist_from_b[j];

            tree.update(start, end, val);
        }
    }

    for i in 1..k {
        let ret = tree.query(i, i);
        writeln!(out, "{}", if ret == MAX { -1 } else { ret }).unwrap();
    }
}

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

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    weight: i64,
}

impl Edge {
    fn new(to: usize, weight: i64) -> Self {
        Self { to, weight }
    }
}

struct RootInfo {
    parent: Vec<usize>,
    parent_weight: Vec<i64>,
    depth: Vec<i64>,
    euler_in: Vec<usize>,
    euler_out: Vec<usize>,
    size: Vec<usize>,
}

impl RootInfo {
    fn new(n: usize) -> Self {
        Self {
            parent: vec![0; n + 1],
            parent_weight: vec![0; n + 1],
            depth: vec![0; n + 1],
            euler_in: vec![0; n + 1],
            euler_out: vec![0; n + 1],
            size: vec![0; n + 1],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum QueryType {
    KSubWeight,
    KSub2Weight,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Query {
    depth: i64,
    node_child: usize,
    r#type: QueryType,
}

impl Query {
    fn new(depth: i64, node_child: usize, r#type: QueryType) -> Self {
        Self {
            depth,
            node_child,
            r#type,
        }
    }
}

struct FenwickTree {
    n: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            n,
            data: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.data[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut ret = 0;

        while idx > 0 {
            ret += self.data[idx];
            idx -= idx & (!idx + 1);
        }

        ret
    }

    fn query_range(&self, left: usize, right: usize) -> i64 {
        self.query(right) - self.query(left - 1)
    }
}

fn precompute_root(roads: &Vec<Vec<Edge>>, root: usize, n: usize) -> RootInfo {
    let mut info = RootInfo::new(n);
    let mut timer = 0;
    let mut stack = Vec::with_capacity(n * 2);

    stack.push((root, 0, 0, 0));

    while let Some((node, parent, state, weight)) = stack.pop() {
        if state == 0 {
            info.parent[node] = parent;
            info.parent_weight[node] = weight;

            if parent != 0 {
                info.depth[node] = info.depth[parent] + weight;
            }

            timer += 1;
            info.euler_in[node] = timer;

            stack.push((node, parent, 1, weight));

            for i in 0..roads[node].len() {
                let next = roads[node][i].to;

                if next != parent {
                    stack.push((next, node, 0, roads[node][i].weight));
                }
            }
        } else {
            let mut size = 1;

            for i in 0..roads[node].len() {
                let next = roads[node][i].to;

                if next != parent {
                    size += info.size[next];
                }
            }

            info.size[node] = size;
            info.euler_out[node] = timer;
        }
    }

    info
}

fn precompute_subtree_counts(root_info: &RootInfo, n: usize, k: i64) -> (Vec<i64>, Vec<i64>) {
    let mut nodes_by_depth = Vec::with_capacity(n);

    for i in 1..=n {
        nodes_by_depth.push((root_info.depth[i], i));
    }

    nodes_by_depth.sort_unstable();

    let mut queries = Vec::with_capacity(n * 2);

    for i in 2..=n {
        let weight = root_info.parent_weight[i];
        let depth1 = root_info.depth[i] + (k - weight);
        let depth2 = root_info.depth[i] + (k - 2 * weight);

        if depth1 >= root_info.depth[i] {
            queries.push(Query::new(depth1, i, QueryType::KSubWeight));
        }

        if depth2 >= root_info.depth[i] {
            queries.push(Query::new(depth2, i, QueryType::KSub2Weight));
        }
    }

    queries.sort_unstable();

    let mut fenwick_tree = FenwickTree::new(n);
    let mut idx = 0;
    let mut cnt_subtree1 = vec![0; n + 1];
    let mut cnt_subtree2 = vec![0; n + 1];

    for query in queries {
        while idx < nodes_by_depth.len() && nodes_by_depth[idx].0 <= query.depth {
            let node = nodes_by_depth[idx].1;
            fenwick_tree.update(root_info.euler_in[node], 1);
            idx += 1;
        }

        let cnt = fenwick_tree.query_range(
            root_info.euler_in[query.node_child],
            root_info.euler_out[query.node_child],
        );

        match query.r#type {
            QueryType::KSubWeight => {
                cnt_subtree1[query.node_child] = cnt;
            }
            QueryType::KSub2Weight => {
                cnt_subtree2[query.node_child] = cnt;
            }
        }
    }

    (cnt_subtree1, cnt_subtree2)
}

fn upper_bound(arr: &[i64], x: i64) -> usize {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = (left + right) / 2;

        if arr[mid] <= x {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

struct CentroidDecomposition {
    graph: Vec<Vec<Edge>>,
    decomposed: Vec<bool>,

    dists_all: Vec<Vec<i64>>,
    dists_child: Vec<Vec<Vec<i64>>>,
    path: Vec<Vec<(usize, i64, usize)>>,

    comp_parent: Vec<usize>,
    comp_size: Vec<usize>,
    comp_nodes: Vec<usize>,

    stack: Vec<(usize, usize, i64)>,
}

impl CentroidDecomposition {
    fn new(n: usize, graph: Vec<Vec<Edge>>) -> Self {
        Self {
            graph,
            decomposed: vec![false; n + 1],

            dists_all: vec![Vec::new(); n + 1],
            dists_child: vec![Vec::new(); n + 1],
            path: vec![Vec::new(); n + 1],

            comp_parent: vec![0; n + 1],
            comp_size: vec![0; n + 1],
            comp_nodes: Vec::with_capacity(n),

            stack: Vec::with_capacity(n),
        }
    }

    fn get_component(&mut self, start: usize) -> usize {
        self.comp_nodes.clear();

        let mut stack = Vec::new();
        stack.push((start, 0, 0));

        while let Some((node, parent, state)) = stack.pop() {
            if state == 0 {
                self.comp_parent[node] = parent;
                self.comp_nodes.push(node);

                stack.push((node, parent, 1));

                for i in 0..self.graph[node].len() {
                    let next = self.graph[node][i].to;

                    if next == parent || self.decomposed[next] {
                        continue;
                    }

                    stack.push((next, node, 0));
                }
            } else {
                let mut size = 1;

                for i in 0..self.graph[node].len() {
                    let next = self.graph[node][i].to;

                    if self.decomposed[next] {
                        continue;
                    }

                    if self.comp_parent[next] == node {
                        size += self.comp_size[next];
                    }
                }

                self.comp_size[node] = size;
            }
        }

        self.comp_size[start]
    }

    fn find_centroid(&self, start: usize, total: usize) -> usize {
        let mut curr = start;

        loop {
            let mut heavy_node = 0;
            let mut heavy_size = 0;

            for i in 0..self.graph[curr].len() {
                let next = self.graph[curr][i].to;

                if self.decomposed[next] {
                    continue;
                }

                if self.comp_parent[next] == curr {
                    let size = self.comp_size[next];

                    if size > heavy_size {
                        heavy_size = size;
                        heavy_node = next;
                    }
                }
            }

            let up = if self.comp_parent[curr] == 0 {
                0
            } else {
                total - self.comp_size[curr]
            };

            if up > heavy_size {
                heavy_size = up;
                heavy_node = self.comp_parent[curr];
            }

            if heavy_size * 2 <= total {
                return curr;
            }

            curr = heavy_node;
        }
    }

    fn collect_lists_for_centroid(&mut self, c: usize) {
        self.dists_all[c].clear();
        self.dists_child[c].clear();

        let mut idx = 0;

        for i in 0..self.graph[c].len() {
            let next = self.graph[c][i].to;

            if self.decomposed[next] {
                continue;
            }

            self.dists_child[c].push(Vec::new());
            self.stack.clear();
            self.stack.push((next, c, self.graph[c][i].weight));

            while let Some((node, parent, dist)) = self.stack.pop() {
                self.dists_all[c].push(dist);
                self.dists_child[c][idx].push(dist);
                self.path[node].push((c, dist, idx));

                for j in 0..self.graph[node].len() {
                    let next = self.graph[node][j].to;

                    if next == parent || self.decomposed[next] {
                        continue;
                    }

                    self.stack
                        .push((next, node, dist + self.graph[node][j].weight));
                }
            }

            idx += 1;
        }

        self.dists_all[c].push(0);
        self.path[c].push((c, 0, usize::MAX));

        self.dists_all[c].sort_unstable();

        for i in 0..self.dists_child[c].len() {
            self.dists_child[c][i].sort_unstable();
        }
    }

    fn decompose_from(&mut self, start: usize) {
        let total = self.get_component(start);
        let c = self.find_centroid(start, total);

        self.decomposed[c] = true;
        self.collect_lists_for_centroid(c);

        for i in 0..self.graph[c].len() {
            let next = self.graph[c][i].to;

            if self.decomposed[next] {
                continue;
            }

            self.decompose_from(next);
        }
    }

    fn build(&mut self) {
        self.decompose_from(1);
    }

    fn query_count(&self, x: usize, r: i64) -> i64 {
        if r < 0 {
            return 0;
        }

        let mut ret = 0;

        for i in 0..self.path[x].len() {
            let (node, dist, idx_child) = self.path[x][i];
            let threshold = r - dist;

            if threshold < 0 {
                continue;
            }

            let add = upper_bound(&self.dists_all[node], threshold) as i64;
            let sub = if idx_child == usize::MAX {
                0
            } else {
                upper_bound(&self.dists_child[node][idx_child], threshold) as i64
            };

            ret += add - sub;
        }

        ret
    }
}

const MOD: i64 = 998_244_353;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut roads = vec![Vec::new(); n + 1];

    for _ in 0..n - 1 {
        let (u, v, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        roads[u].push(Edge::new(v, c));
        roads[v].push(Edge::new(u, c));
    }

    let root_info = precompute_root(&roads, 1, n);
    let (cnt_subtree1, cnt_subtree2) = precompute_subtree_counts(&root_info, n, k);

    let mut centroid_decomposition = CentroidDecomposition::new(n, roads);
    centroid_decomposition.build();

    let mut ret = 0;

    for i in 2..=n {
        let parent = root_info.parent[i];
        let weight = root_info.parent_weight[i];

        let size_a = (n - root_info.size[i]) as i64;
        let size_b = root_info.size[i] as i64;

        let cnt_a_all = if k - weight < 0 {
            0
        } else {
            centroid_decomposition.query_count(parent, k - weight)
        };
        let cnt_a_sub = if k - 2 * weight < 0 {
            0
        } else {
            cnt_subtree2[i]
        };
        let cnt_a = cnt_a_all - cnt_a_sub;
        let cnt_b = if k - weight < 0 { 0 } else { cnt_subtree1[i] };

        let term1 = cnt_a * size_b % MOD;
        let term2 = cnt_b * size_a % MOD;
        let val = weight * (term1 + term2) % MOD;

        ret = (ret + val) % MOD;
    }

    writeln!(out, "{ret}").unwrap();
}

use io::Write;
use std::{collections::VecDeque, io, str};

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

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    rev: usize,
    capacity: i64,
}

impl Edge {
    fn new(to: usize, rev: usize, capacity: i64) -> Self {
        Self { to, rev, capacity }
    }
}

struct Dinic {
    graph: Vec<Vec<Edge>>,
    source: usize,
    sink: usize,
    check: Vec<i64>,
    done: Vec<usize>,
}

impl Dinic {
    fn new(n: usize, source: usize, sink: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n],
            source,
            sink,
            check: vec![-1; n],
            done: vec![0; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, capacity: i64) {
        let orig = self.graph[v].len();
        let dest = self.graph[u].len();

        self.graph[u].push(Edge::new(v, orig, capacity));
        self.graph[v].push(Edge::new(u, dest, 0));
    }

    fn process_bfs(&mut self) -> bool {
        self.check.fill(-1);

        let mut queue = VecDeque::new();

        queue.push_back(self.source);
        self.check[self.source] = 0;

        while let Some(u) = queue.pop_front() {
            for &e in self.graph[u].iter() {
                if e.capacity > 0 && self.check[e.to] < 0 {
                    queue.push_back(e.to);
                    self.check[e.to] = self.check[u] + 1;
                }
            }
        }

        self.check[self.sink] >= 0
    }

    fn process_dfs(&mut self, idx: usize, flow: i64) -> i64 {
        if idx == self.sink {
            return flow;
        }

        let n = self.graph[idx].len();

        while self.done[idx] < n {
            let edge = self.graph[idx][self.done[idx]];

            if edge.capacity > 0 && self.check[edge.to] == self.check[idx] + 1 {
                let flow_current = self.process_dfs(edge.to, flow.min(edge.capacity));

                if flow_current > 0 {
                    self.graph[idx][self.done[idx]].capacity -= flow_current;
                    self.graph[edge.to][edge.rev].capacity += flow_current;

                    return flow_current;
                }
            }

            self.done[idx] += 1;
        }

        0
    }

    fn get_flow(&mut self) -> i64 {
        let mut flow_total = 0;

        while self.process_bfs() {
            self.done.fill(0);

            loop {
                let flow_current = self.process_dfs(self.source, i64::MAX);

                if flow_current == 0 {
                    break;
                }

                flow_total += flow_current;
            }
        }

        flow_total
    }

    fn reachable_from_source(&self) -> Vec<bool> {
        let n = self.graph.len();
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();

        visited[self.source] = true;
        queue.push_back(self.source);

        while let Some(u) = queue.pop_front() {
            for &edge in self.graph[u].iter() {
                if edge.capacity > 0 && !visited[edge.to] {
                    visited[edge.to] = true;
                    queue.push_back(edge.to);
                }
            }
        }

        visited
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![0; n + 1],
            size: vec![1; n + 1],
        }
    }

    fn init(&mut self) {
        for i in 0..self.parent.len() {
            self.parent[i] = i;
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }

        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];

        true
    }
}

struct GomoryHuTree {
    graph: Vec<Vec<(usize, i64)>>,
}

impl GomoryHuTree {
    fn new(n: usize) -> Self {
        GomoryHuTree {
            graph: vec![Vec::new(); n + 1],
        }
    }

    fn from_undirected_graph(n: usize, edges_input: &Vec<(usize, usize, i64)>) -> Self {
        if n <= 1 {
            return GomoryHuTree::new(n);
        }

        let mut parent = vec![1; n + 1];
        let mut weight = vec![0; n + 1];

        parent[1] = 1;

        for source in 2..=n {
            let sink = parent[source];
            let mut maximum_flow = Dinic::new(n + 1, source, sink);

            for &(u, v, w) in edges_input.iter() {
                maximum_flow.add_edge(u, v, w);
                maximum_flow.add_edge(v, u, w);
            }

            let flow = maximum_flow.get_flow();
            let reach = maximum_flow.reachable_from_source();

            weight[source] = flow;

            for v in source + 1..=n {
                if parent[v] == sink && reach[v] {
                    parent[v] = source;
                }
            }

            let sink_parent = parent[sink];

            if reach[sink_parent] {
                parent[source] = sink_parent;
                parent[sink] = source;

                let sink_weight = weight[sink];

                weight[sink] = flow;
                weight[source] = sink_weight;
            }
        }

        let mut tree = GomoryHuTree::new(n);

        for v in 2..=n {
            let p = parent[v];
            let w = weight[v];

            tree.graph[v].push((p, w));
            tree.graph[p].push((v, w));
        }

        tree
    }
}

fn build_graph(block: &Vec<Vec<char>>, n: usize, m: usize) -> (usize, Vec<(usize, usize, i64)>) {
    let get_char = |r: usize, c: usize| -> char {
        if r >= block.len() || c >= block[r].len() {
            ' '
        } else {
            block[r][c]
        }
    };

    let mut map_id = vec![vec![0; m]; n];
    let mut id = 0;

    for i in 0..n {
        for j in 0..m {
            let r = 4 * i + 2 + 2 * (j % 2);
            let c = 6 * j + 4;

            if get_char(r, c) == '*' {
                id += 1;
                map_id[i][j] = id;
            }
        }
    }

    let mut edges = Vec::new();

    for i in 0..n {
        for j in 0..m {
            if map_id[i][j] == 0 {
                continue;
            }

            let r = 4 * i + 2 + 2 * (j % 2);
            let c = 6 * j + 4;

            // Bottom
            if i + 1 < n {
                if map_id[i + 1][j] != 0 {
                    if get_char(r + 2, c - 1) == ' '
                        && get_char(r + 2, c) == ' '
                        && get_char(r + 2, c + 1) == ' '
                    {
                        edges.push((map_id[i][j], map_id[i + 1][j], 1));
                    }
                }
            }

            // Right
            if j + 1 < m {
                if j % 2 == 0 {
                    if i > 0 {
                        if map_id[i - 1][j + 1] != 0 && get_char(r - 1, c + 3) == ' ' {
                            edges.push((map_id[i][j], map_id[i - 1][j + 1], 1));
                        }
                    }

                    if map_id[i][j + 1] != 0 && get_char(r + 1, c + 3) == ' ' {
                        edges.push((map_id[i][j], map_id[i][j + 1], 1));
                    }
                } else {
                    if map_id[i][j + 1] != 0 && get_char(r - 1, c + 3) == ' ' {
                        edges.push((map_id[i][j], map_id[i][j + 1], 1));
                    }

                    if i + 1 < n {
                        if map_id[i + 1][j + 1] != 0 && get_char(r + 1, c + 3) == ' ' {
                            edges.push((map_id[i][j], map_id[i + 1][j + 1], 1));
                        }
                    }
                }
            }
        }
    }

    (id, edges)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let mut block = Vec::with_capacity(4 * n + 3);

        for _ in 0..4 * n + 3 {
            let mut line = scan.line().to_string();

            if line.ends_with('\n') {
                line.pop();
            }

            block.push(line.chars().collect());
        }

        let (id, edges) = build_graph(&block, n, m);

        if id <= 1 {
            writeln!(out, "Case #{i}: 0").unwrap();
            continue;
        }

        let tree = GomoryHuTree::from_undirected_graph(id, &edges);
        let mut edges_tree = Vec::with_capacity(id - 1);

        for u in 1..=id {
            for &(v, w) in tree.graph[u].iter() {
                if u < v {
                    edges_tree.push((u, v, w));
                }
            }
        }

        edges_tree.sort_unstable_by(|a, b| b.2.cmp(&a.2));

        let mut union_find = UnionFind::new(id);
        union_find.init();

        let mut ret = 0;

        for (u, v, w) in edges_tree {
            let root_u = union_find.find(u);
            let root_v = union_find.find(v);

            if root_u == root_v {
                continue;
            }

            let size_u = union_find.size[root_u];
            let size_v = union_find.size[root_v];

            ret += w * (size_u as i64) * (size_v as i64);
            union_find.union(u, v);
        }

        writeln!(out, "Case #{i}: {ret}").unwrap();
    }
}

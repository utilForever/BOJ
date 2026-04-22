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

    fn get_flow_limited(&mut self, limit: i64) -> i64 {
        let mut flow_total = 0;

        while flow_total < limit && self.process_bfs() {
            self.done.fill(0);

            loop {
                let remain = limit - flow_total;

                if remain <= 0 {
                    break;
                }

                let flow_current = self.process_dfs(self.source, remain);

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

const INF: i64 = 1_000_000_000_000;

fn is_bipartite_after_deleting(
    graph: &Vec<Vec<usize>>,
    vertices_deleted: &Vec<usize>,
    limit: usize,
) -> bool {
    let mut deleted = vec![false; limit];

    for &v in vertices_deleted {
        if v < limit {
            deleted[v] = true;
        }
    }

    bipartition_excluding(graph, &deleted, limit).is_some()
}

fn bipartition_excluding(
    graph: &Vec<Vec<usize>>,
    vertices_deleted: &Vec<bool>,
    limit: usize,
) -> Option<Vec<i64>> {
    let mut color = vec![-1; limit];
    let mut deque = VecDeque::new();

    for s in 0..limit {
        if vertices_deleted[s] || color[s] != -1 {
            continue;
        }

        color[s] = 0;
        deque.push_back(s);

        while let Some(u) = deque.pop_front() {
            for &v in graph[u].iter() {
                if v >= limit || vertices_deleted[v] {
                    continue;
                }

                if color[v] == -1 {
                    color[v] = color[u] ^ 1;
                    deque.push_back(v);
                } else if color[v] == color[u] {
                    return None;
                }
            }
        }
    }

    Some(color)
}

fn min_st_vertex_separator(
    edges_active: &Vec<(usize, usize)>,
    blocked: &Vec<bool>,
    side_source: &Vec<bool>,
    side_sink: &Vec<bool>,
    limit: usize,
    max_cut: usize,
) -> Option<Vec<usize>> {
    let mut idxes = vec![usize::MAX; limit];
    let mut vertices = Vec::new();

    for v in 0..limit {
        if blocked[v] {
            continue;
        }

        idxes[v] = vertices.len();
        vertices.push(v);
    }

    let len = vertices.len();
    let source = 2 * len;
    let sink = 2 * len + 1;
    let mut maximum_flow = Dinic::new(2 * len + 2, source, sink);

    for i in 0..len {
        maximum_flow.add_edge(2 * i, 2 * i + 1, 1);
    }

    for &(u, v) in edges_active {
        if u >= limit || v >= limit {
            continue;
        }

        if blocked[u] || blocked[v] {
            continue;
        }

        maximum_flow.add_edge(2 * idxes[u] + 1, 2 * idxes[v], INF);
        maximum_flow.add_edge(2 * idxes[v] + 1, 2 * idxes[u], INF);
    }

    for &v in vertices.iter() {
        if side_source[v] {
            maximum_flow.add_edge(source, 2 * idxes[v], INF);
        }

        if side_sink[v] {
            maximum_flow.add_edge(2 * idxes[v] + 1, sink, INF);
        }
    }

    let flow = maximum_flow.get_flow_limited(max_cut as i64 + 1);

    if flow > max_cut as i64 {
        return None;
    }

    let reachable = maximum_flow.reachable_from_source();
    let mut cut = Vec::new();

    for &v in vertices.iter() {
        let i = idxes[v];

        let vin = 2 * i;
        let vout = 2 * i + 1;

        if reachable[vin] && !reachable[vout] {
            cut.push(v);
        }
    }

    Some(cut)
}

fn compress(
    graph: &Vec<Vec<usize>>,
    edges_active: &Vec<(usize, usize)>,
    s0: &Vec<usize>,
    limit: usize,
    k: usize,
) -> Option<Vec<usize>> {
    let mut exist_in_s0 = vec![false; limit];

    for &v in s0 {
        exist_in_s0[v] = true;
    }

    let color = bipartition_excluding(graph, &exist_in_s0, limit)?;
    let mut total = 1;

    for _ in 0..s0.len() {
        total *= 3;
    }

    for mut kind in 0..total {
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut deleted_from_s0 = Vec::new();

        for &v in s0 {
            match kind % 3 {
                0 => left.push(v),
                1 => right.push(v),
                _ => deleted_from_s0.push(v),
            }

            kind /= 3;
        }

        if deleted_from_s0.len() > k {
            continue;
        }

        let mut side_source = vec![false; limit];
        let mut side_sink = vec![false; limit];

        for &u in left.iter() {
            for &v in graph[u].iter() {
                if v >= limit || exist_in_s0[v] {
                    continue;
                }

                if color[v] == 0 {
                    side_source[v] = true;
                } else {
                    side_sink[v] = true;
                }
            }
        }

        for &u in right.iter() {
            for &v in graph[u].iter() {
                if v >= limit || exist_in_s0[v] {
                    continue;
                }

                if color[v] == 0 {
                    side_sink[v] = true;
                } else {
                    side_source[v] = true;
                }
            }
        }

        let remain = k - deleted_from_s0.len();

        if let Some(mut separator) = min_st_vertex_separator(
            edges_active,
            &exist_in_s0,
            &side_source,
            &side_sink,
            limit,
            remain,
        ) {
            let mut ret = deleted_from_s0.clone();

            ret.append(&mut separator);
            ret.sort_unstable();
            ret.dedup();

            if ret.len() <= k && is_bipartite_after_deleting(graph, &ret, limit) {
                return Some(ret);
            }
        }
    }

    None
}

struct ParityDsu {
    parent: Vec<usize>,
    size: Vec<u8>,
    parity: Vec<u8>,
}

impl ParityDsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![0; n],
            parity: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> (usize, u8) {
        if self.parent[x] == x {
            return (x, 0);
        }

        let p = self.parent[x];
        let (root, par_to_root) = self.find(p);

        self.parent[x] = root;
        self.parity[x] ^= par_to_root;

        (self.parent[x], self.parity[x])
    }

    fn unite(&mut self, u: usize, v: usize, w: u8) -> bool {
        let (mut root_u, mut parity_u) = self.find(u);
        let (mut root_v, mut parity_v) = self.find(v);

        if root_u == root_v {
            return (parity_u ^ parity_v) == w;
        }

        if self.size[root_u] < self.size[root_v] {
            std::mem::swap(&mut root_u, &mut root_v);
            std::mem::swap(&mut parity_u, &mut parity_v);
        }

        self.parent[root_v] = root_u;
        self.parity[root_v] = parity_u ^ parity_v ^ w;

        if self.size[root_u] == self.size[root_v] {
            self.size[root_u] += 1;
        }

        true
    }
}

fn rebuild_dsu(
    edges_active: &Vec<(usize, usize)>,
    cover: &Vec<usize>,
    n: usize,
) -> (ParityDsu, Vec<bool>) {
    let mut in_cover = vec![false; n];

    for &v in cover {
        in_cover[v] = true;
    }

    let mut dsu = ParityDsu::new(n);

    for &(u, v) in edges_active {
        if in_cover[u] || in_cover[v] {
            continue;
        }

        dsu.unite(u, v, 1);
    }

    (dsu, in_cover)
}

fn odd_cycle_transversal(edges: &Vec<(usize, usize)>, n: usize, k: usize) -> Option<Vec<usize>> {
    if k >= n {
        return Some((0..n).collect());
    }

    let mut degree = vec![0; n];

    for &(u, v) in edges {
        if u == v {
            continue;
        }

        degree[u] += 1;
        degree[v] += 1;
    }

    let mut order = (0..n).collect::<Vec<_>>();
    order.sort_unstable_by_key(|&v| (degree[v], v));

    let mut pos = vec![0; n];

    for (i, &v) in order.iter().enumerate() {
        pos[v] = i;
    }

    let mut edges_normalized = Vec::with_capacity(edges.len());

    for &(u, v) in edges {
        if u < v {
            edges_normalized.push((u, v));
        } else {
            edges_normalized.push((v, u));
        }
    }

    edges_normalized.sort_unstable();
    edges_normalized.dedup();

    let mut edges_activated_by_vertex = vec![Vec::new(); n];

    for &(u, v) in edges_normalized.iter() {
        edges_activated_by_vertex[v].push((u, v));
    }

    let mut graph = vec![Vec::new(); n];
    let mut edges_active = Vec::new();
    let mut cover = Vec::new();

    let mut dsu = ParityDsu::new(n);
    let mut in_cover = vec![false; n];

    for i in 0..n {
        let mut check = false;

        for &(u, v) in edges_activated_by_vertex[i].iter() {
            graph[u].push(v);
            graph[v].push(u);
            edges_active.push((u, v));

            if in_cover[u] || in_cover[v] {
                continue;
            }

            if !check && !dsu.unite(u, v, 1) {
                check = true;
            }
        }

        if !check {
            continue;
        }

        let mut s0 = cover.clone();

        s0.push(i);
        s0.sort_unstable();
        s0.dedup();

        if s0.len() <= k {
            cover = s0;
            (dsu, in_cover) = rebuild_dsu(&edges_active, &cover, n);
            continue;
        }

        match compress(&graph, &edges_active, &s0, i + 1, k) {
            Some(cover_next) => {
                cover = cover_next;
                (dsu, in_cover) = rebuild_dsu(&edges_active, &cover, n);
            }
            None => return None,
        }
    }

    Some(cover)
}

// Reference: https://www.sciencedirect.com/science/article/abs/pii/S0167637703001482
// Reference: https://sites.cs.ucsb.edu/~daniello/papers/octIterComp.pdf
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let mut survivors = vec![(0, 0, 0); m];
        let mut cnt = vec![0; n + 1];

        for i in 0..m {
            let (a, b, c) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
            );

            survivors[i] = (a, b, c);
            cnt[a] += 1;
            cnt[b] += 1;
            cnt[c] += 1;
        }

        if m == 0 {
            writeln!(out, "TAK").unwrap();
            continue;
        }

        let mut pivot = 0;

        for i in 1..=n {
            if cnt[i] == m {
                pivot = i;
                break;
            }
        }

        if pivot == 0 {
            writeln!(out, "NIE").unwrap();
            continue;
        }

        let mut idxes = vec![usize::MAX; n + 1];
        let mut idx = 0;

        for i in 1..=n {
            if i != pivot {
                idxes[i] = idx;
                idx += 1;
            }
        }

        let mut edges = Vec::with_capacity(m);
        let mut ret = true;

        for &(a, b, c) in survivors.iter() {
            let mut other = [0; 2];
            let mut len = 0;

            for x in [a, b, c] {
                if x == pivot {
                    continue;
                }

                other[len] = idxes[x];
                len += 1;
            }

            if len != 2 {
                ret = false;
                break;
            }

            edges.push((other[0], other[1]));
        }

        if ret && odd_cycle_transversal(&edges, idx, 2).is_some() {
            writeln!(out, "TAK").unwrap();
        } else {
            writeln!(out, "NIE").unwrap();
        }
    }
}

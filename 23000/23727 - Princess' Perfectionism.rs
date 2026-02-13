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

#[derive(Clone, Copy, PartialEq)]
enum EdgeState {
    Alive,
    Deleted,
}

#[derive(Clone, Copy)]
struct Edge {
    left: usize,
    right: usize,
    state: EdgeState,
}

impl Edge {
    fn new(left: usize, right: usize, state: EdgeState) -> Self {
        Self { left, right, state }
    }
}

const UNREACHABLE: i64 = 0;
const EVEN: i64 = 1;
const ODD: i64 = 2;

struct RankMaximalMatching {
    edges: Vec<Edge>,
    edge_in_matching: Vec<bool>,
    edges_from_left: Vec<Vec<usize>>,
    edges_from_right: Vec<Vec<usize>>,

    matching_size: usize,
    mate_right_of_left: Vec<Option<usize>>,
    mate_left_of_right: Vec<Option<usize>>,
    matching_edge_of_left: Vec<Option<usize>>,
    matching_edge_of_right: Vec<Option<usize>>,

    free_vertices_left: Vec<usize>,
    pos_free_vertices_left: Vec<Option<usize>>,
    free_vertices_right: Vec<usize>,
    pos_free_vertices_right: Vec<Option<usize>>,

    idx_hopcroft_karp: usize,
    queue_hopcroft_karp: Vec<usize>,
    layer_dist: Vec<i64>,
    layer_seen_at: Vec<usize>,
    next_edge_ptr: Vec<usize>,
    next_edge_seen_at: Vec<usize>,
    stack_left: Vec<usize>,
    stack_edge: Vec<i64>,

    idx_decomposition: usize,
    queue_decomposition: Vec<usize>,
    seen_at_left: Vec<usize>,
    seen_at_right: Vec<usize>,
    parity_left: Vec<i64>,
    parity_right: Vec<i64>,
}

impl RankMaximalMatching {
    fn new(n: usize) -> Self {
        Self {
            edges: Vec::new(),
            edge_in_matching: Vec::new(),
            edges_from_left: vec![Vec::new(); n],
            edges_from_right: vec![Vec::new(); n],

            matching_size: 0,
            mate_right_of_left: vec![None; n],
            mate_left_of_right: vec![None; n],
            matching_edge_of_left: vec![None; n],
            matching_edge_of_right: vec![None; n],

            free_vertices_left: (0..n).collect(),
            pos_free_vertices_left: (0..n).map(Some).collect(),
            free_vertices_right: (0..n).collect(),
            pos_free_vertices_right: (0..n).map(Some).collect(),

            idx_hopcroft_karp: 0,
            queue_hopcroft_karp: Vec::new(),
            layer_dist: vec![0; n],
            layer_seen_at: vec![0; n],
            next_edge_ptr: vec![0; n],
            next_edge_seen_at: vec![0; n],
            stack_left: Vec::new(),
            stack_edge: Vec::new(),

            idx_decomposition: 0,
            queue_decomposition: Vec::new(),
            seen_at_left: vec![0; n],
            seen_at_right: vec![0; n],
            parity_left: vec![UNREACHABLE; n],
            parity_right: vec![UNREACHABLE; n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        let idx = self.edges.len();

        self.edges.push(Edge::new(from, to, EdgeState::Alive));
        self.edge_in_matching.push(false);
        self.edges_from_left[from].push(idx);
        self.edges_from_right[to].push(idx);
    }
}

impl RankMaximalMatching {
    fn process_bfs(&mut self, idx: usize) -> bool {
        self.queue_hopcroft_karp.clear();

        for &node in self.free_vertices_left.iter() {
            self.layer_dist[node] = 0;
            self.layer_seen_at[node] = idx;
            self.queue_hopcroft_karp.push(node);
        }

        let mut head = 0;
        let mut check = false;

        while head < self.queue_hopcroft_karp.len() {
            let u = self.queue_hopcroft_karp[head];

            head += 1;

            for &idx_edge in self.edges_from_left[u].iter() {
                if self.edges[idx_edge].state == EdgeState::Deleted {
                    continue;
                }

                if self.edge_in_matching[idx_edge] {
                    continue;
                }

                let v = self.edges[idx_edge].right;
                let u_next = self.mate_left_of_right[v];

                if let Some(u_next) = u_next {
                    if self.layer_seen_at[u_next] != idx {
                        self.layer_dist[u_next] = self.layer_dist[u] + 1;
                        self.layer_seen_at[u_next] = idx;
                        self.queue_hopcroft_karp.push(u_next);
                    }
                } else {
                    check = true;
                }
            }
        }

        check
    }

    fn process_dfs(&mut self, from: usize, idx: usize) -> bool {
        self.stack_left.clear();
        self.stack_edge.clear();

        self.stack_left.push(from);
        self.stack_edge.push(-1);

        while let Some(&u) = self.stack_left.last() {
            if self.layer_seen_at[u] != idx || self.layer_dist[u] == -1 {
                self.stack_left.pop();
                self.stack_edge.pop();
                continue;
            }

            if self.next_edge_seen_at[u] != idx {
                self.next_edge_seen_at[u] = idx;
                self.next_edge_ptr[u] = 0;
            }

            let mut check = false;

            while self.next_edge_ptr[u] < self.edges_from_left[u].len() {
                let idx_edge = self.edges_from_left[u][self.next_edge_ptr[u]];

                self.next_edge_ptr[u] += 1;

                if self.edges[idx_edge].state == EdgeState::Deleted {
                    continue;
                }

                if self.edge_in_matching[idx_edge] {
                    continue;
                }

                let v = self.edges[idx_edge].right;

                match self.mate_left_of_right[v] {
                    Some(u_next) => {
                        if self.layer_seen_at[u_next] == idx
                            && self.layer_dist[u_next] == self.layer_dist[u] + 1
                        {
                            let last = self.stack_edge.len() - 1;

                            self.stack_edge[last] = idx_edge as i64;
                            self.stack_left.push(u_next);
                            self.stack_edge.push(-1);

                            check = true;
                            break;
                        }
                    }
                    None => {
                        let last = self.stack_edge.len() - 1;

                        self.stack_edge[last] = idx_edge as i64;
                        self.apply_augment_from_right(Some(v));
                        return true;
                    }
                }
            }

            if check {
                continue;
            }

            self.layer_dist[u] = -1;
            self.stack_left.pop();
            self.stack_edge.pop();
        }

        false
    }

    fn remove_from_free_vertices_left(&mut self, node: usize) {
        let idx = self.pos_free_vertices_left[node].unwrap();
        let last = self.free_vertices_left.pop().unwrap();

        if idx < self.free_vertices_left.len() {
            self.free_vertices_left[idx] = last;
            self.pos_free_vertices_left[last] = Some(idx);
        }

        self.pos_free_vertices_left[node] = None;
    }

    fn remove_from_free_vertices_right(&mut self, node: usize) {
        let idx = self.pos_free_vertices_right[node].unwrap();
        let last = self.free_vertices_right.pop().unwrap();

        if idx < self.free_vertices_right.len() {
            self.free_vertices_right[idx] = last;
            self.pos_free_vertices_right[last] = Some(idx);
        }

        self.pos_free_vertices_right[node] = None;
    }

    fn apply_augment_from_right(&mut self, mut idx_right: Option<usize>) {
        for i in (0..self.stack_edge.len()).rev() {
            let u = self.stack_left[i];
            let idx_edge = self.stack_edge[i] as usize;
            let v = self.edges[idx_edge].right;

            assert_eq!(idx_right, Some(v));

            let old_v = self.mate_right_of_left[u];
            let old_edge = self.matching_edge_of_left[u];

            if let Some(old_edge) = old_edge {
                self.edge_in_matching[old_edge] = false;

                if let Some(old_v) = old_v {
                    self.mate_left_of_right[old_v] = None;
                    self.matching_edge_of_right[old_v] = None;
                }
            }

            self.mate_right_of_left[u] = Some(v);
            self.matching_edge_of_left[u] = Some(idx_edge);
            self.mate_left_of_right[v] = Some(u);
            self.matching_edge_of_right[v] = Some(idx_edge);
            self.edge_in_matching[idx_edge] = true;

            if self.pos_free_vertices_left[u].is_some() {
                self.remove_from_free_vertices_left(u);
            }

            if self.pos_free_vertices_right[v].is_some() {
                self.remove_from_free_vertices_right(v);
            }

            idx_right = old_v;

            if idx_right.is_none() {
                break;
            }
        }
    }

    fn augment_to_maximum(&mut self) {
        loop {
            self.idx_hopcroft_karp += 1;

            if !self.process_bfs(self.idx_hopcroft_karp) {
                break;
            }

            let mut idx = 0;

            while idx < self.free_vertices_left.len() {
                let node = self.free_vertices_left[idx];

                if self.layer_seen_at[node] != self.idx_hopcroft_karp {
                    idx += 1;
                    continue;
                }

                if self.process_dfs(node, self.idx_hopcroft_karp) {
                    self.matching_size += 1;
                } else {
                    idx += 1;
                }
            }
        }
    }
}

impl RankMaximalMatching {
    fn decompose_even_odd_unreachable(&mut self) {
        self.idx_decomposition += 1;
        self.queue_decomposition.clear();

        for &node in self.free_vertices_left.iter() {
            self.seen_at_left[node] = self.idx_decomposition;
            self.parity_left[node] = EVEN;
            self.queue_decomposition.push(node);
        }

        for &node in self.free_vertices_right.iter() {
            self.seen_at_right[node] = self.idx_decomposition;
            self.parity_right[node] = EVEN;
            self.queue_decomposition.push(node | (1 << 31));
        }

        let mut head = 0;

        while head < self.queue_decomposition.len() {
            let u = self.queue_decomposition[head];

            head += 1;

            if u & (1 << 31) == 0 {
                let parity = self.parity_left[u];

                if parity == EVEN {
                    for &idx_edge in self.edges_from_left[u].iter() {
                        if self.edges[idx_edge].state == EdgeState::Deleted {
                            continue;
                        }

                        if self.edge_in_matching[idx_edge] {
                            continue;
                        }

                        let v = self.edges[idx_edge].right;

                        if self.seen_at_right[v] != self.idx_decomposition {
                            self.seen_at_right[v] = self.idx_decomposition;
                            self.parity_right[v] = ODD;
                            self.queue_decomposition.push(v | (1 << 31));
                        }
                    }
                } else {
                    if let Some(idx_edge) = self.matching_edge_of_left[u] {
                        let v = self.edges[idx_edge].right;

                        if self.seen_at_right[v] != self.idx_decomposition {
                            self.seen_at_right[v] = self.idx_decomposition;
                            self.parity_right[v] = EVEN;
                            self.queue_decomposition.push(v | (1 << 31));
                        }
                    }
                }
            } else {
                let v = u & (!(1 << 31));
                let parity = self.parity_right[v];

                if parity == EVEN {
                    for &idx_edge in self.edges_from_right[v].iter() {
                        if self.edges[idx_edge].state == EdgeState::Deleted {
                            continue;
                        }

                        if self.edge_in_matching[idx_edge] {
                            continue;
                        }

                        let u = self.edges[idx_edge].left;

                        if self.seen_at_left[u] != self.idx_decomposition {
                            self.seen_at_left[u] = self.idx_decomposition;
                            self.parity_left[u] = ODD;
                            self.queue_decomposition.push(u);
                        }
                    }
                } else {
                    if let Some(idx_edge) = self.matching_edge_of_right[v] {
                        let u = self.edges[idx_edge].left;

                        if self.seen_at_left[u] != self.idx_decomposition {
                            self.seen_at_left[u] = self.idx_decomposition;
                            self.parity_left[u] = EVEN;
                            self.queue_decomposition.push(u);
                        }
                    }
                }
            }
        }
    }
}

fn scc_kosaraju(
    graph: &Vec<Vec<usize>>,
    graph_rev: &Vec<Vec<usize>>,
    n: usize,
) -> (usize, Vec<usize>) {
    let mut visited = vec![false; n];
    let mut order = Vec::with_capacity(n);

    for i in 0..n {
        if visited[i] {
            continue;
        }

        visited[i] = true;

        let mut stack = Vec::new();
        stack.push((i, 0));

        while let Some((u, idx)) = stack.pop() {
            if idx < graph[u].len() {
                stack.push((u, idx + 1));

                let v = graph[u][idx];

                if !visited[v] {
                    visited[v] = true;
                    stack.push((v, 0));
                }
            } else {
                order.push(u);
            }
        }
    }

    let mut idx_component = 0;
    let mut component = vec![usize::MAX; n];

    for &i in order.iter().rev() {
        if component[i] != usize::MAX {
            continue;
        }

        component[i] = idx_component;

        let mut stack = Vec::new();
        stack.push(i);

        while let Some(u) = stack.pop() {
            for &v in graph_rev[u].iter() {
                if component[v] == usize::MAX {
                    component[v] = idx_component;
                    stack.push(v);
                }
            }
        }

        idx_component += 1;
    }

    (idx_component, component)
}

fn dfs_find_sink(
    graph: &Vec<Vec<usize>>,
    degree_out: &Vec<usize>,
    used: &mut Vec<bool>,
    start: usize,
) -> Option<usize> {
    let mut stack = Vec::new();

    used[start] = true;
    stack.push((start, 0));

    while let Some((u, idx)) = stack.pop() {
        if degree_out[u] == 0 {
            return Some(u);
        }

        if idx < graph[u].len() {
            stack.push((u, idx + 1));

            let v = graph[u][idx];

            if !used[v] {
                used[v] = true;
                stack.push((v, 0));
            }
        }
    }

    None
}

// Reference: https://d-michail.github.io/assets/papers/RankMaximalMatchings-journal.pdf
// Reference: https://www.cse.iitm.ac.in/~meghana/papers/ISAAC14-RMM.pdf
// Reference: https://en.wikipedia.org/wiki/Dulmage%E2%80%93Mendelsohn_decomposition
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

        if n == 0 && m == 0 {
            break;
        }

        let mut rank_maximal_matching = RankMaximalMatching::new(n);

        for _ in 0..m {
            let (s, t) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
            rank_maximal_matching.add_edge(s, t);
        }

        rank_maximal_matching.augment_to_maximum();
        rank_maximal_matching.decompose_even_odd_unreachable();

        let idx_decomposition = rank_maximal_matching.idx_decomposition;
        let mut core_left = vec![false; n];
        let mut core_right = vec![false; n];

        for i in 0..n {
            core_left[i] = rank_maximal_matching.seen_at_left[i] != idx_decomposition;
            core_right[i] = rank_maximal_matching.seen_at_right[i] != idx_decomposition;
        }

        let mut graph = vec![Vec::<usize>::new(); 2 * n];
        let mut graph_rev = vec![Vec::<usize>::new(); 2 * n];

        for edge in rank_maximal_matching.edges.iter() {
            if edge.state == EdgeState::Deleted {
                continue;
            }

            let u = edge.left;
            let v = edge.right;

            if core_left[u] && core_right[v] {
                let a = u;
                let b = n + v;

                graph[a].push(b);
                graph_rev[b].push(a);
            }
        }

        for u in 0..n {
            if let Some(idx_edge) = rank_maximal_matching.matching_edge_of_left[u] {
                let v = rank_maximal_matching.edges[idx_edge].right;

                if core_left[u] && core_right[v] {
                    let a = n + v;
                    let b = u;

                    graph[a].push(b);
                    graph_rev[b].push(a);
                }
            }
        }

        let (cnt_component, component) = scc_kosaraju(&graph, &graph_rev, 2 * n);
        let mut pos_left = vec![0usize; n];
        let mut pos_right = vec![0usize; n];

        for u in 0..n {
            pos_left[u] = component[u];
        }

        for v in 0..n {
            pos_right[v] = component[n + v];
        }

        let mut rep_left = vec![None::<usize>; cnt_component];
        let mut rep_right = vec![None::<usize>; cnt_component];

        for u in 0..n {
            let c = pos_left[u];

            if rep_left[c].is_none() {
                rep_left[c] = Some(u);
            }
        }

        for v in 0..n {
            let c = pos_right[v];

            if rep_right[c].is_none() {
                rep_right[c] = Some(v);
            }
        }

        let mut graph_condensation = vec![Vec::new(); cnt_component];
        let mut degree_in = vec![0; cnt_component];
        let mut degree_out = vec![0; cnt_component];

        for e in rank_maximal_matching.edges.iter() {
            if e.state == EdgeState::Deleted {
                continue;
            }

            let a = pos_left[e.left];
            let b = pos_right[e.right];

            if a == b {
                continue;
            }

            graph_condensation[a].push(b);
            degree_out[a] += 1;
            degree_in[b] += 1;
        }

        let mut has_active = false;

        for i in 0..cnt_component {
            if degree_in[i] > 0 || degree_out[i] > 0 {
                has_active = true;
                break;
            }
        }

        if !has_active {
            writeln!(out, "0").unwrap();
            continue;
        }

        let mut used = vec![false; cnt_component];
        let mut path = Vec::new();
        let mut s_remain = Vec::new();
        let mut t_remain = Vec::new();

        for i in 0..cnt_component {
            if degree_in[i] == 0 && degree_out[i] != 0 && !used[i] {
                if let Some(sink) = dfs_find_sink(&graph_condensation, &degree_out, &mut used, i) {
                    path.push((i, sink));
                } else {
                    s_remain.push(i);
                }
            }
        }

        if path.is_empty() {
            writeln!(out, "0").unwrap();
            continue;
        }

        for i in 0..cnt_component {
            if degree_out[i] == 0 && degree_in[i] != 0 && !used[i] {
                t_remain.push(i);
            }
        }

        let mut ret = Vec::new();

        for i in 0..path.len() {
            let from = path[i].1;
            let to = path[(i + 1) % path.len()].0;
            ret.push((from, to));
        }

        let extra = s_remain.len().max(t_remain.len());

        for i in 0..extra {
            let from = if i < t_remain.len() {
                t_remain[i]
            } else {
                path[0].1
            };

            let to = if i < s_remain.len() {
                s_remain[i]
            } else {
                path[0].0
            };

            ret.push((from, to));
        }

        writeln!(out, "{}", ret.len()).unwrap();

        for (a, b) in ret {
            writeln!(
                out,
                "{} {}",
                rep_left[a].unwrap() + 1,
                rep_right[b].unwrap() + 1
            )
            .unwrap();
        }
    }
}

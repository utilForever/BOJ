use io::Write;
use std::{
    collections::{HashMap, HashSet},
    io, str,
};

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

pub struct DisjointSets {
    parent: Vec<usize>,
}

impl DisjointSets {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }

    pub fn find(&mut self, u: usize) -> usize {
        let pu = self.parent[u];

        if pu != u {
            self.parent[u] = self.find(pu);
        }

        self.parent[u]
    }

    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        let (pu, pv) = (self.find(u), self.find(v));

        self.parent[pu] = pv;

        pu != pv
    }
}

pub struct Graph {
    first: Vec<Option<usize>>,
    next: Vec<Option<usize>>,
    point_start: Vec<usize>,
    point_end: Vec<usize>,
    cost: Vec<i64>,
}

impl Graph {
    pub fn new(vmax: usize, emax_hint: usize) -> Self {
        Self {
            first: vec![None; vmax],
            next: Vec::with_capacity(emax_hint),
            point_start: Vec::with_capacity(emax_hint),
            point_end: Vec::with_capacity(emax_hint),
            cost: Vec::with_capacity(emax_hint),
        }
    }

    pub fn num_v(&self) -> usize {
        self.first.len()
    }

    pub fn num_e(&self) -> usize {
        self.point_end.len()
    }

    pub fn add_edge(&mut self, u: usize, v: usize, c: i64) {
        self.next.push(self.first[u]);
        self.first[u] = Some(self.num_e());
        self.point_start.push(u);
        self.point_end.push(v);
        self.cost.push(c);
    }

    pub fn add_undirected_edge(&mut self, u: usize, v: usize, c: i64) {
        self.add_edge(u, v, c);
        self.add_edge(v, u, c);
    }

    pub fn adj_list(&self, u: usize) -> AdjListIterator {
        AdjListIterator {
            graph: self,
            next_e: self.first[u],
        }
    }
}

pub struct AdjListIterator<'a> {
    graph: &'a Graph,
    next_e: Option<usize>,
}

impl<'a> Iterator for AdjListIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_e.map(|e| {
            let v = self.graph.point_end[e];
            self.next_e = self.graph.next[e];

            (e, v)
        })
    }
}

struct ConnectivityData {
    time: usize,
    vis: Box<[usize]>,
    low: Box<[usize]>,
    v_stack: Vec<usize>,
    e_stack: Vec<usize>,
}

impl ConnectivityData {
    fn new(num_v: usize) -> Self {
        Self {
            time: 0,
            vis: vec![0; num_v].into_boxed_slice(),
            low: vec![0; num_v].into_boxed_slice(),
            v_stack: vec![],
            e_stack: vec![],
        }
    }

    fn visit(&mut self, u: usize) {
        self.time += 1;
        self.vis[u] = self.time;
        self.low[u] = self.time;
        self.v_stack.push(u);
    }

    fn lower(&mut self, u: usize, val: usize) {
        if self.low[u] > val {
            self.low[u] = val
        }
    }
}

pub struct ConnectivityGraph<'a> {
    pub graph: &'a Graph,
    pub cc: Vec<usize>,
    pub vcc: Vec<usize>,
    pub num_cc: usize,
    pub num_vcc: usize,
}

impl<'a> ConnectivityGraph<'a> {
    pub fn new(graph: &'a Graph, is_directed: bool) -> Self {
        let mut connect = Self {
            graph,
            cc: vec![0; graph.num_v()],
            vcc: vec![0; graph.num_e()],
            num_cc: 0,
            num_vcc: 0,
        };
        let mut data = ConnectivityData::new(graph.num_v());

        for u in 0..graph.num_v() {
            if data.vis[u] == 0 {
                if is_directed {
                    connect.scc(&mut data, u);
                } else {
                    connect.bcc(&mut data, u, graph.num_e() + 1);
                }
            }
        }

        connect
    }

    fn scc(&mut self, data: &mut ConnectivityData, u: usize) {
        data.visit(u);

        for (_, v) in self.graph.adj_list(u) {
            if data.vis[v] == 0 {
                self.scc(data, v);
            }

            if self.cc[v] == 0 {
                data.lower(u, data.low[v]);
            }
        }

        if data.vis[u] == data.low[u] {
            self.num_cc += 1;

            while let Some(v) = data.v_stack.pop() {
                self.cc[v] = self.num_cc;

                if v == u {
                    break;
                }
            }
        }
    }

    pub fn two_sat_assign(&self) -> Option<Vec<bool>> {
        (0..self.graph.num_v() / 2)
            .map(|i| {
                let scc_true = self.cc[2 * i];
                let scc_false = self.cc[2 * i + 1];

                if scc_true == scc_false {
                    None
                } else {
                    Some(scc_true < scc_false)
                }
            })
            .collect()
    }

    pub fn topological_sort(&self) -> Vec<usize> {
        let mut vertices = (0..self.graph.num_v()).collect::<Vec<_>>();

        vertices.sort_unstable_by_key(|&u| self.num_cc - self.cc[u]);

        vertices
    }

    fn bcc(&mut self, data: &mut ConnectivityData, u: usize, par: usize) {
        data.visit(u);

        for (e, v) in self.graph.adj_list(u) {
            if data.vis[v] == 0 {
                data.e_stack.push(e);
                self.bcc(data, v, e);
                data.lower(u, data.low[v]);

                if data.vis[u] <= data.low[v] {
                    self.num_vcc += 1;

                    while let Some(top_e) = data.e_stack.pop() {
                        self.vcc[top_e] = self.num_vcc;
                        self.vcc[top_e ^ 1] = self.num_vcc;

                        if e ^ top_e <= 1 {
                            break;
                        }
                    }
                }
            } else if data.vis[v] < data.vis[u] && e ^ par != 1 {
                data.lower(u, data.vis[v]);
                data.e_stack.push(e);
            } else if v == u {
                self.num_vcc += 1;
                self.vcc[e] = self.num_vcc;
                self.vcc[e ^ 1] = self.num_vcc;
            }
        }

        if data.vis[u] == data.low[u] {
            self.num_cc += 1;

            while let Some(v) = data.v_stack.pop() {
                self.cc[v] = self.num_cc;

                if v == u {
                    break;
                }
            }
        }
    }

    pub fn is_cut_vertex(&self, u: usize) -> bool {
        if let Some(first_e) = self.graph.first[u] {
            self.graph
                .adj_list(u)
                .any(|(e, _)| self.vcc[first_e] != self.vcc[e])
        } else {
            false
        }
    }

    pub fn is_cut_edge(&self, e: usize) -> bool {
        let u = self.graph.point_end[e ^ 1];
        let v = self.graph.point_end[e];
        self.cc[u] != self.cc[v]
    }
}

fn process_dfs(
    connectivity: &ConnectivityGraph,
    adjacent: &Vec<HashMap<usize, Vec<(usize, usize)>>>,
    set: &mut HashSet<usize>,
    visited: &mut Vec<bool>,
    ret: &mut Vec<Vec<i64>>,
    point_end: usize,
    vcc: usize,
) {
    set.insert(point_end);

    if let Some(adj) = adjacent[point_end].get(&vcc) {
        for &(e, v) in adj {
            if connectivity.vcc[e] == vcc && !set.contains(&v) && !visited[e] && !visited[e ^ 1] {
                visited[e] = true;
                visited[e ^ 1] = true;

                process_dfs(connectivity, adjacent, set, visited, ret, v, vcc);

                ret[vcc].push(connectivity.graph.cost[e]);
                break;
            } else if connectivity.vcc[e] == vcc && !visited[e] && !visited[e ^ 1] {
                visited[e] = true;
                visited[e ^ 1] = true;
                ret[connectivity.vcc[e]].push(connectivity.graph.cost[e]);
            }
        }
    }
}

fn subset_sum(nums: &Vec<i64>, target: i64) -> Vec<bool> {
    let n = nums.len();
    let nums_max = *nums.iter().max().unwrap();

    let mut brk = 0;
    let mut sbrk = 0;

    while brk < n && sbrk + nums[brk] <= target {
        sbrk += nums[brk];
        brk += 1;
    }

    if brk == n && sbrk != target {
        return Vec::new();
    }

    let mut dp = vec![-1; nums_max as usize * 2];
    let mut prev = vec![vec![-1; nums_max as usize * 2]; n];
    let offset = target - nums_max + 1;

    dp[(sbrk - offset) as usize] = brk as i64;

    for i in brk..n {
        let mut dp2 = dp.clone();
        let prev_curr = &mut prev[i];

        for j in (0..nums_max).rev() {
            if dp2[(j + nums[i]) as usize] < dp2[j as usize] {
                prev_curr[(j + nums[i]) as usize] = -2;
                dp2[(j + nums[i]) as usize] = dp2[j as usize];
            }
        }

        for j in (nums_max..nums_max * 2).rev() {
            for k in dp[j as usize].max(0)..dp2[j as usize] {
                if dp2[(j - nums[k as usize]) as usize] < k {
                    prev_curr[(j - nums[k as usize]) as usize] = k;
                    dp2[(j - nums[k as usize]) as usize] = k;
                }
            }
        }

        std::mem::swap(&mut dp, &mut dp2);
    }

    if dp[(nums_max - 1) as usize] == -1 {
        return Vec::new();
    }

    let mut i = (n - 1) as i64;
    let mut j = nums_max - 1;
    let mut ret = vec![false; n];

    while i >= brk as i64 {
        let p = prev[i as usize][j as usize];

        if p == -2 {
            ret[i as usize] = !ret[i as usize];
            j -= nums[i as usize];
            i -= 1;
        } else if p == -1 {
            i -= 1;
        } else {
            ret[p as usize] = !ret[p as usize];
            j += nums[p as usize];
        }
    }

    while i >= 0 {
        ret[i as usize] = !ret[i as usize];
        i -= 1;
    }

    ret
}

// Reference: https://github.com/EbTech/rust-algorithms
// Reference: https://atcoder.jp/contests/abc221/submissions/26323758
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = Graph::new(n, m * 2);

    for _ in 0..m {
        let (u, v, l) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        graph.add_undirected_edge(u - 1, v - 1, l);
    }

    let connectivity = ConnectivityGraph::new(&graph, false);
    let mut visited = vec![false; connectivity.graph.num_e()];
    let mut adjacent: Vec<HashMap<usize, Vec<(usize, usize)>>> =
        vec![HashMap::new(); connectivity.graph.num_v()];
    let mut ret = vec![Vec::new(); connectivity.num_vcc + 1];

    for i in 0..connectivity.graph.num_v() {
        for (e, v) in connectivity.graph.adj_list(i) {
            match adjacent[i].get_mut(&connectivity.vcc[e]) {
                Some(x) => {
                    x.push((e, v));
                }
                None => {
                    adjacent[i].insert(connectivity.vcc[e], vec![(e, v)]);
                }
            }
        }
    }

    for i in 0..connectivity.graph.num_e() {
        if visited[i] || visited[i ^ 1] {
            continue;
        }

        visited[i] = true;
        visited[i ^ 1] = true;

        let mut set = HashSet::new();

        ret[connectivity.vcc[i]].push(connectivity.graph.cost[i]);
        set.insert(connectivity.graph.point_start[i]);

        process_dfs(
            &connectivity,
            &adjacent,
            &mut set,
            &mut visited,
            &mut ret,
            connectivity.graph.point_end[i],
            connectivity.vcc[i],
        );
    }

    let mut is_sat = true;

    for i in 1..=connectivity.num_vcc {
        if ret[i].len() <= 1 {
            continue;
        }

        if ret[i].iter().sum::<i64>() % 2 == 1 {
            is_sat = false;
            break;
        }

        let subset_sum = subset_sum(
            &ret[i].iter().map(|&x| x as i64).collect::<Vec<_>>(),
            ret[i].iter().sum::<i64>() / 2,
        );

        if subset_sum.is_empty() {
            is_sat = false;
            break;
        }
    }

    writeln!(out, "{}", if is_sat { "YES" } else { "NO" }).unwrap();
}

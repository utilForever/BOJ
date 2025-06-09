use crate::{
    dual_graph::DualGraph,
    maximum_weight_matching::{Matching, SENTINEL},
};
use io::Write;
use std::{
    cmp::{max, Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    io, str,
};
use Ordering::{Greater, Less};

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

// Maximum Weight Maching in General Graph
// Reference: https://github.com/ahenshaw/mwmatching
mod maximum_weight_matching {
    use std::cmp::max;

    type Vertex = usize;
    type Vertices = Vec<Vertex>;
    type Weight = i64;
    type Weights = Vec<Weight>;
    type Edge = (Vertex, Vertex, Weight);
    type Edges = Vec<Edge>;

    pub const SENTINEL: Vertex = <Vertex>::MAX;
    const CHECK_DELTA: bool = false;
    const CHECK_OPTIMUM: bool = true;

    #[derive(Debug, Default)]
    pub struct Matching {
        cnt_vertex: usize,
        cnt_edge: usize,
        weight_max: Weight,
        edges: Edges,
        dual_var: Weights,
        endpoint: Vertices,
        label: Vertices,
        label_end: Vertices,
        blossom_in: Vertices,
        blossom_base: Vertices,
        blossom_parent: Vertices,
        edge_best: Vertices,
        blossoms_unused: Vertices,
        mate: Vertices,
        queue: Vertices,
        blossom_best_edges: Vec<Vertices>,
        blossom_childs: Vec<Vertices>,
        blossom_endpoints: Vec<Vertices>,
        neighbors: Vec<Vertices>,
        allow_edge: Vec<bool>,
        cardinality_max: bool,
    }

    impl Matching {
        pub fn new(edges: Edges) -> Matching {
            let mut matching = Matching {
                edges,
                ..Default::default()
            };

            if !matching.edges.is_empty() {
                matching.initialize();
            }

            matching
        }

        fn initialize(&mut self) {
            self.cnt_edge = self.edges.len();
            self.cnt_vertex = 0;
            self.cardinality_max = false;

            for &(i, j, _) in self.edges.iter() {
                if i >= self.cnt_vertex {
                    self.cnt_vertex = i + 1;
                }

                if j >= self.cnt_vertex {
                    self.cnt_vertex = j + 1;
                }
            }

            self.weight_max = self.edges.iter().max_by_key(|x| x.2).unwrap().2;

            self.endpoint = (0..2 * self.cnt_edge)
                .map(|x| {
                    if x % 2 == 0 {
                        self.edges[x / 2].0
                    } else {
                        self.edges[x / 2].1
                    }
                })
                .collect();

            self.neighbors = vec![Vec::new(); self.cnt_vertex];

            for (k, &(i, j, _)) in self.edges.iter().enumerate() {
                self.neighbors[i].push(2 * k + 1);
                self.neighbors[j].push(2 * k);
            }

            self.mate = vec![SENTINEL; self.cnt_vertex];
            self.label = vec![0; 2 * self.cnt_vertex];
            self.label_end = vec![SENTINEL; 2 * self.cnt_vertex];
            self.blossom_in = (0..self.cnt_vertex).collect();
            self.blossom_parent = vec![SENTINEL; 2 * self.cnt_vertex];
            self.blossom_childs = vec![Vec::new(); 2 * self.cnt_vertex];
            self.blossom_base = (0..(self.cnt_vertex)).collect();
            self.blossom_base.extend(vec![SENTINEL; self.cnt_vertex]);
            self.blossom_endpoints = vec![Vec::new(); 2 * self.cnt_vertex];
            self.edge_best = vec![SENTINEL; 2 * self.cnt_vertex];
            self.blossom_best_edges = vec![Vec::new(); 2 * self.cnt_vertex];
            self.blossoms_unused = (self.cnt_vertex..2 * self.cnt_vertex).collect();
            self.dual_var = vec![self.weight_max; self.cnt_vertex];
            self.dual_var.extend(vec![0; self.cnt_vertex]);
            self.allow_edge = vec![false; self.cnt_edge];
            self.queue = Vec::new();
        }

        #[inline]
        fn slack(&self, k: Vertex) -> Weight {
            let (i, j, wt) = self.edges[k];
            self.dual_var[i] + self.dual_var[j] - 2 * wt
        }

        fn blossom_leaves(&self, b: Vertex) -> Vertices {
            let mut leaves: Vertices = Vec::new();

            if b < self.cnt_vertex {
                leaves.push(b);
            } else {
                for &t in self.blossom_childs[b].iter() {
                    if t < self.cnt_vertex {
                        leaves.push(t);
                    } else {
                        leaves.extend(self.blossom_leaves(t));
                    }
                }
            }

            leaves
        }

        fn assign_label(&mut self, w: Vertex, t: Vertex, p: Vertex) {
            let b = self.blossom_in[w];

            self.label[w] = t;
            self.label[b] = t;
            self.label_end[w] = p;
            self.label_end[b] = p;
            self.edge_best[w] = SENTINEL;
            self.edge_best[b] = SENTINEL;

            if t == 1 {
                let leaves = self.blossom_leaves(b);
                self.queue.extend(leaves);
            } else if t == 2 {
                let base = self.blossom_base[b];
                let base_mate = self.mate[base];
                let endpoint = self.endpoint[base_mate];

                self.assign_label(endpoint, 1, base_mate ^ 1);
            }
        }

        fn scan_blossom(&mut self, v: Vertex, w: Vertex) -> Vertex {
            let mut path = Vec::new();
            let mut base = SENTINEL;
            let mut v = v;
            let mut w = w;

            while v != SENTINEL || w != SENTINEL {
                let mut b = self.blossom_in[v];

                if (self.label[b] & 4) != 0 {
                    base = self.blossom_base[b];
                    break;
                }

                path.push(b);
                self.label[b] = 5;

                if self.label_end[b] == SENTINEL {
                    v = SENTINEL;
                } else {
                    v = self.endpoint[self.label_end[b]];
                    b = self.blossom_in[v];
                    v = self.endpoint[self.label_end[b]];
                }

                if w != SENTINEL {
                    std::mem::swap(&mut v, &mut w);
                }
            }

            for b in path {
                self.label[b] = 1;
            }

            base
        }

        fn add_blossom(&mut self, base: Vertex, k: usize) {
            let (mut v, mut w, _) = self.edges[k];
            let bb = self.blossom_in[base];
            let mut bv = self.blossom_in[v];
            let mut bw = self.blossom_in[w];

            let b = self.blossoms_unused.pop().unwrap();
            self.blossom_base[b] = base;
            self.blossom_parent[b] = SENTINEL;
            self.blossom_parent[bb] = b;

            self.blossom_childs[b] = Vec::new();
            self.blossom_endpoints[b] = Vec::new();

            while bv != bb {
                self.blossom_parent[bv] = b;
                self.blossom_childs[b].push(bv);
                self.blossom_endpoints[b].push(self.label_end[bv]);

                v = self.endpoint[self.label_end[bv]];
                bv = self.blossom_in[v];
            }

            self.blossom_childs[b].push(bb);
            self.blossom_childs[b].reverse();
            self.blossom_endpoints[b].reverse();
            self.blossom_endpoints[b].push(2 * k);

            while bw != bb {
                self.blossom_parent[bw] = b;
                self.blossom_childs[b].push(bw);
                self.blossom_endpoints[b].push(self.label_end[bw] ^ 1);

                w = self.endpoint[self.label_end[bw]];
                bw = self.blossom_in[w];
            }

            self.label[b] = 1;
            self.label_end[b] = self.label_end[bb];
            self.dual_var[b] = 0;

            for v in self.blossom_leaves(b) {
                if self.label[self.blossom_in[v]] == 2 {
                    self.queue.push(v);
                }

                self.blossom_in[v] = b;
            }

            let mut edge_best_to = vec![SENTINEL; 2 * self.cnt_vertex];

            for &bv in self.blossom_childs[b].iter() {
                let mut num_lists = Vec::new();

                if self.blossom_best_edges[bv].is_empty() {
                    for v in self.blossom_leaves(bv) {
                        num_lists.push(self.neighbors[v].iter().map(|p| p / 2).collect());
                    }
                } else {
                    let bbe = self.blossom_best_edges[bv].clone();
                    num_lists.push(bbe);
                }

                for num_list in num_lists {
                    for k in num_list {
                        let (mut i, mut j, _wt) = self.edges[k];

                        if self.blossom_in[j] == b {
                            std::mem::swap(&mut i, &mut j);
                        }

                        let bj = self.blossom_in[j];
                        let best_to = edge_best_to[bj];

                        if (bj != b)
                            && (self.label[bj] == 1)
                            && (best_to == SENTINEL || (self.slack(k) < self.slack(best_to)))
                        {
                            edge_best_to[bj] = k;
                        }
                    }
                }

                self.blossom_best_edges[bv] = Vec::new();
                self.edge_best[bv] = SENTINEL;
            }

            self.blossom_best_edges[b] = edge_best_to
                .iter()
                .filter(|k| **k != SENTINEL)
                .copied()
                .collect();

            self.edge_best[b] = SENTINEL;

            for &k in self.blossom_best_edges[b].iter() {
                let be = self.edge_best[b];

                if (be == SENTINEL) || (self.slack(k) < self.slack(be)) {
                    self.edge_best[b] = k;
                }
            }
        }

        fn expand_blossom(&mut self, b: Vertex, stage_end: bool) {
            for s in self.blossom_childs[b].clone() {
                self.blossom_parent[s] = SENTINEL;

                if s < self.cnt_vertex {
                    self.blossom_in[s] = s;
                } else if stage_end && (self.dual_var[s] == 0) {
                    self.expand_blossom(s, stage_end);
                } else {
                    for &v in self.blossom_leaves(s).iter() {
                        self.blossom_in[v] = s;
                    }
                }
            }

            if !stage_end && (self.label[b] == 2) {
                let entry_child = self.blossom_in[self.endpoint[self.label_end[b] ^ 1]];
                let mut j = self.blossom_childs[b]
                    .iter()
                    .position(|r| *r == entry_child)
                    .unwrap() as i32;
                let (j_step, endptrick) = if (j & 1) != 0 {
                    j -= self.blossom_childs[b].len() as i32;
                    (1, 0)
                } else {
                    (-1, 1)
                };

                let mut p = self.label_end[b];

                while j != 0 {
                    self.label[self.endpoint[p ^ 1]] = 0;
                    self.label[self.endpoint[pos_neg_index(
                        &self.blossom_endpoints[b],
                        j - endptrick as i32,
                    ) ^ endptrick
                        ^ 1]] = 0;

                    let ep = self.endpoint[p ^ 1];

                    self.assign_label(ep, 2, p);
                    self.allow_edge
                        [pos_neg_index(&self.blossom_endpoints[b], j - endptrick as i32) / 2] =
                        true;

                    j += j_step;

                    p = pos_neg_index(&self.blossom_endpoints[b], j - endptrick as i32) ^ endptrick;
                    self.allow_edge[p / 2] = true;

                    j += j_step;
                }

                let bv = pos_neg_index(&self.blossom_childs[b], j);

                self.label[self.endpoint[p ^ 1]] = 2;
                self.label[bv] = 2;

                self.label_end[self.endpoint[p ^ 1]] = p;
                self.label_end[bv] = p;

                self.edge_best[bv] = SENTINEL;

                j += j_step;

                while pos_neg_index(&self.blossom_childs[b], j) != entry_child {
                    let bv = pos_neg_index(&self.blossom_childs[b], j);

                    if self.label[bv] == 1 {
                        j += j_step;
                        continue;
                    }

                    let mut v: Vertex = SENTINEL;

                    for &temp in self.blossom_leaves(bv).iter() {
                        v = temp;

                        if self.label[v] != 0 {
                            break;
                        }
                    }

                    if self.label[v] != 0 {
                        self.label[v] = 0;
                        self.label[self.endpoint[self.mate[self.blossom_base[bv]]]] = 0;

                        let lblend = self.label_end[v];
                        self.assign_label(v, 2, lblend);
                    }

                    j += j_step;
                }
            }

            self.label[b] = SENTINEL;
            self.label_end[b] = SENTINEL;
            self.blossom_base[b] = SENTINEL;
            self.edge_best[b] = SENTINEL;

            self.blossom_childs[b] = Vec::new();
            self.blossom_endpoints[b] = Vec::new();
            self.blossom_best_edges[b] = Vec::new();

            self.blossoms_unused.push(b);
        }

        fn augment_blossom(&mut self, b: Vertex, v: Vertex) {
            let mut t = v;

            while self.blossom_parent[t] != b {
                t = self.blossom_parent[t];
            }

            if (t != SENTINEL) && (t >= self.cnt_vertex) {
                self.augment_blossom(t, v);
            }

            let i = self.blossom_childs[b].iter().position(|r| *r == t).unwrap();
            let mut j = i as i32;

            let (j_step, endptrick) = if (i & 1) != 0 {
                j -= self.blossom_childs[b].len() as i32;
                (1, 0)
            } else {
                (-1, 1)
            };

            while j != 0 {
                j += j_step;

                let mut t = pos_neg_index(&self.blossom_childs[b], j);
                let p = pos_neg_index(&self.blossom_endpoints[b], j - endptrick as i32) ^ endptrick;

                if (t != SENTINEL) && (t >= self.cnt_vertex) {
                    let endpoint = self.endpoint[p];
                    self.augment_blossom(t, endpoint);
                }

                if j_step == 1 {
                    j += 1;
                } else {
                    j -= 1;
                }

                t = pos_neg_index(&self.blossom_childs[b], j);

                if (t != SENTINEL) && (t >= self.cnt_vertex) {
                    let endpoint = self.endpoint[p ^ 1];
                    self.augment_blossom(t, endpoint);
                }

                self.mate[self.endpoint[p]] = p ^ 1;
                self.mate[self.endpoint[p ^ 1]] = p;
            }

            rotate(&mut self.blossom_childs[b], i);
            rotate(&mut self.blossom_endpoints[b], i);

            self.blossom_base[b] = self.blossom_base[self.blossom_childs[b][0]];
        }

        fn augment_matching(&mut self, k: Vertex) {
            let (v, w, _) = self.edges[k];

            for (mut s, mut p) in [(v, 2 * k + 1), (w, 2 * k)].iter() {
                loop {
                    let bs = self.blossom_in[s];

                    if (bs != SENTINEL) && (bs >= self.cnt_vertex) {
                        self.augment_blossom(bs, s);
                    }

                    self.mate[s] = p;

                    if self.label_end[bs] == SENTINEL {
                        break;
                    }

                    let t = self.endpoint[self.label_end[bs]];
                    let bt = self.blossom_in[t];

                    s = self.endpoint[self.label_end[bt]];

                    let j = self.endpoint[self.label_end[bt] ^ 1];

                    if (bt != SENTINEL) && (bt >= self.cnt_vertex) {
                        self.augment_blossom(bt, j);
                    }

                    self.mate[j] = self.label_end[bt];
                    p = self.label_end[bt] ^ 1;
                }
            }
        }

        fn verify_optimum(&self) {
            for k in 0..self.cnt_edge {
                let (i, j, _) = self.edges[k];
                let mut blossoms_i = vec![i];
                let mut blossoms_j = vec![j];

                while self.blossom_parent[*(blossoms_i.last().unwrap())] != SENTINEL {
                    let idx_x = blossoms_i.last().unwrap();
                    blossoms_i.push(self.blossom_parent[*idx_x]);
                }

                while self.blossom_parent[*(blossoms_j.last().unwrap())] != SENTINEL {
                    let idx_j = blossoms_j.last().unwrap();
                    blossoms_j.push(self.blossom_parent[*idx_j]);
                }

                blossoms_i.reverse();
                blossoms_j.reverse();
            }
        }

        fn check_delta2(&self) {
            for v in 0..self.cnt_vertex {
                if self.label[self.blossom_in[v]] == 0 {
                    let mut bd = 0;
                    let mut bk = SENTINEL;

                    for &p in self.neighbors[v].iter() {
                        let k = p / 2;
                        let w = self.endpoint[p];

                        if self.label[self.blossom_in[w]] == 1 {
                            let d = self.slack(k);

                            if (bk == SENTINEL) || d < bd {
                                bk = k;
                                bd = d;
                            }
                        }
                    }
                }
            }
        }

        fn check_delta3(&self) {
            let mut bk = SENTINEL;
            let mut bd = 0;
            let mut tbk = SENTINEL;
            let mut tbd = 0;

            for b in 0..2 * self.cnt_vertex {
                if (self.blossom_parent[b] == SENTINEL) && (self.label[b] == 1) {
                    for v in self.blossom_leaves(b) {
                        for &p in self.neighbors[v].iter() {
                            let k = p / 2;
                            let w = self.endpoint[p];

                            if (self.blossom_in[w] != b) && (self.label[self.blossom_in[w]] == 1) {
                                let d = self.slack(k);

                                if (bk == SENTINEL) || (d < bd) {
                                    bk = k;
                                    bd = d;
                                }
                            }
                        }
                    }

                    if self.edge_best[b] != SENTINEL {
                        if (tbk == SENTINEL) || (self.slack(self.edge_best[b]) < tbd) {
                            tbk = self.edge_best[b];
                            tbd = self.slack(self.edge_best[b]);
                        }
                    }
                }
            }
        }

        pub fn solve(&mut self) -> Vertices {
            if self.edges.is_empty() {
                return Vec::new();
            }

            let mut k_slack = 0;

            for _ in 0..self.cnt_vertex {
                self.label = vec![0; 2 * self.cnt_vertex];
                self.edge_best = vec![SENTINEL; 2 * self.cnt_vertex];

                for ll in self.cnt_vertex..2 * self.cnt_vertex {
                    self.blossom_best_edges[ll] = Vec::new();
                }

                self.allow_edge = vec![false; self.cnt_edge];
                self.queue = Vec::new();

                for v in 0..self.cnt_vertex {
                    if self.mate[v] == SENTINEL && self.label[self.blossom_in[v]] == 0 {
                        self.assign_label(v, 1, SENTINEL);
                    }
                }

                let mut augmented = false;

                loop {
                    while !self.queue.is_empty() && (!augmented) {
                        let v = self.queue.pop().unwrap();

                        for p in self.neighbors[v].clone() {
                            let k = p / 2;
                            let w = self.endpoint[p];

                            if self.blossom_in[v] == self.blossom_in[w] {
                                continue;
                            }

                            if !self.allow_edge[k] {
                                k_slack = self.slack(k);

                                if k_slack <= 0 {
                                    self.allow_edge[k] = true
                                }
                            }

                            if self.allow_edge[k] {
                                if self.label[self.blossom_in[w]] == 0 {
                                    self.assign_label(w, 2, p ^ 1);
                                } else if self.label[self.blossom_in[w]] == 1 {
                                    let base = self.scan_blossom(v, w);

                                    if base != SENTINEL {
                                        self.add_blossom(base, k);
                                    } else {
                                        self.augment_matching(k);
                                        augmented = true;
                                        break;
                                    }
                                } else if self.label[w] == 0 {
                                    self.label[w] = 2;
                                    self.label_end[w] = p ^ 1;
                                }
                            } else if self.label[self.blossom_in[w]] == 1 {
                                let b = self.blossom_in[v];

                                if self.edge_best[b] == SENTINEL
                                    || k_slack < self.slack(self.edge_best[b])
                                {
                                    self.edge_best[b] = k;
                                }
                            } else if self.label[w] == 0 {
                                if self.edge_best[w] == SENTINEL
                                    || k_slack < self.slack(self.edge_best[w])
                                {
                                    self.edge_best[w] = k;
                                }
                            }
                        }
                    }

                    if augmented {
                        break;
                    }

                    let mut delta_type = -1;
                    let mut delta = 0;
                    let mut delta_edge = 0;
                    let mut delta_blossom = 0;

                    if CHECK_DELTA {
                        self.check_delta2();
                        self.check_delta3();
                    }

                    if !self.cardinality_max {
                        delta_type = 1;
                        delta = *self.dual_var[0..self.cnt_vertex].iter().min().unwrap();
                    }

                    for v in 0..self.cnt_vertex {
                        if (self.label[self.blossom_in[v]] == 0) && (self.edge_best[v] != SENTINEL)
                        {
                            let d = self.slack(self.edge_best[v]);

                            if (delta_type == -1) || (d < delta) {
                                delta = d;
                                delta_type = 2;
                                delta_edge = self.edge_best[v];
                            }
                        }
                    }

                    for b in 0..2 * self.cnt_vertex {
                        if (self.blossom_parent[b] == SENTINEL)
                            && (self.label[b] == 1)
                            && (self.edge_best[b] != SENTINEL)
                        {
                            let k_slack = self.slack(self.edge_best[b]);
                            let d = k_slack / 2;

                            if (delta_type == -1) || (d < delta) {
                                delta = d;
                                delta_type = 3;
                                delta_edge = self.edge_best[b];
                            }
                        }
                    }

                    for b in self.cnt_vertex..2 * self.cnt_vertex {
                        if self.blossom_base[b] != SENTINEL
                            && self.blossom_parent[b] == SENTINEL
                            && self.label[b] == 2
                            && (delta_type == -1 || self.dual_var[b] < delta)
                        {
                            delta = self.dual_var[b];
                            delta_type = 4;
                            delta_blossom = b;
                        }
                    }

                    if delta_type == -1 {
                        delta_type = 1;
                        delta = max(0, *(self.dual_var[..self.cnt_vertex]).iter().min().unwrap());
                    }

                    for v in 0..self.cnt_vertex {
                        match self.label[self.blossom_in[v]] {
                            0 => {}
                            1 => {
                                self.dual_var[v] -= delta;
                            }
                            2 => {
                                self.dual_var[v] += delta;
                            }
                            _ => {
                                unreachable!()
                            }
                        }
                    }

                    for b in self.cnt_vertex..2 * self.cnt_vertex {
                        if (self.blossom_base[b] != SENTINEL)
                            && (self.blossom_parent[b] == SENTINEL)
                        {
                            match self.label[b] {
                                0 => {}
                                1 => {
                                    self.dual_var[b] += delta;
                                }
                                2 => {
                                    self.dual_var[b] -= delta;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                    }

                    match delta_type {
                        1 => {
                            break;
                        }
                        2 => {
                            let (mut i, j, _) = self.edges[delta_edge];

                            self.allow_edge[delta_edge] = true;

                            if self.label[self.blossom_in[i]] == 0 {
                                i = j;
                            }

                            self.queue.push(i);
                        }
                        3 => {
                            self.allow_edge[delta_edge] = true;

                            let (i, _, _) = self.edges[delta_edge];

                            self.queue.push(i);
                        }
                        4 => {
                            self.expand_blossom(delta_blossom, false);
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }

                if !augmented {
                    break;
                }

                for b in self.cnt_vertex..2 * self.cnt_vertex {
                    if self.blossom_parent[b] == SENTINEL
                        && self.blossom_base[b] != SENTINEL
                        && self.label[b] == 1
                        && self.dual_var[b] == 0
                    {
                        self.expand_blossom(b, true);
                    }
                }
            }

            if CHECK_OPTIMUM {
                self.verify_optimum();
            }

            for v in 0..self.cnt_vertex {
                if self.mate[v] != SENTINEL {
                    self.mate[v] = self.endpoint[self.mate[v]];
                }
            }

            self.mate.clone()
        }

        pub fn _max_cardinality(&mut self) -> &mut Self {
            self.cardinality_max = true;
            self
        }
    }

    fn rotate(v: &mut Vertices, split: usize) {
        let v2 = v.clone();
        let (a, b) = v2.split_at(split);

        v[..b.len()].copy_from_slice(b);
        v[b.len()..].copy_from_slice(a);
    }

    fn pos_neg_index(v: &Vertices, index: i32) -> Vertex {
        let actual_index = if index >= 0 {
            index as usize
        } else {
            v.len() - (-index) as usize
        };

        v[actual_index]
    }
}

pub trait Ext {
    type Item;

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    fn lower_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;

    fn upper_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    fn upper_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;
}

impl<T> Ext for [T] {
    type Item = T;

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.lower_bound_by(|y| y.cmp(x))
    }

    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0;
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Less { mid } else { base };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp == Less) as usize
    }

    fn upper_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.upper_bound_by(|y| y.cmp(x))
    }

    fn upper_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0;
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Greater { base } else { mid };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp != Greater) as usize
    }
}

// Constructing Dual Graph
// Reference: https://kopricky.github.io/code/Graph/dual_graph.html
mod dual_graph {
    use super::Ext;
    use std::cmp::Ordering;

    #[derive(Debug, Clone, Copy, Eq)]
    struct Info {
        x: i64,
        y: i64,
        z: i64,
    }

    impl Info {
        fn new(x: i64, y: i64, z: i64) -> Self {
            Self { x, y, z }
        }
    }

    impl PartialEq for Info {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    impl Ord for Info {
        fn cmp(&self, other: &Self) -> Ordering {
            use Ordering::*;

            if self.y * other.y <= 0 {
                if self.y == 0 && other.y == 0 {
                    return if self.x >= 0 && other.x < 0 {
                        Less
                    } else {
                        Greater
                    };
                }

                if self.y == 0 && other.y > 0 {
                    return if self.x >= 0 { Less } else { Greater };
                }

                if self.y > 0 && other.y == 0 {
                    return if other.x < 0 { Less } else { Greater };
                }

                return self.y.cmp(&other.y);
            }

            if self.x * other.x <= 0 {
                if self.x == other.x {
                    return Equal;
                }

                if self.y > 0 {
                    return if self.x > other.x { Less } else { Greater };
                } else {
                    return if self.x < other.x { Less } else { Greater };
                }
            }

            let lhs = self.y * other.x;
            let rhs = self.x * other.y;

            if lhs < rhs {
                Less
            } else if lhs > rhs {
                Greater
            } else {
                Equal
            }
        }
    }

    impl PartialOrd for Info {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug, Clone)]
    struct HalfEdge {
        to: usize,
        rev: usize,
        face: i64,
    }

    impl HalfEdge {
        fn new(to: usize, rev: usize) -> Self {
            Self { to, rev, face: -1 }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Edge {
        pub to: usize,
        pub e: (usize, usize),
    }

    impl Edge {
        pub fn new(to: usize, e: (usize, usize)) -> Self {
            Self { to, e }
        }
    }

    #[derive(Debug, Clone)]
    pub struct DualGraph {
        pub cnt_vertices: usize,
        position: Vec<Vec<Info>>,
        graph: Vec<Vec<HalfEdge>>,
        pub points: Vec<(i64, i64)>,
        pub dual_graph: Vec<Vec<Edge>>,
        pub faces: Vec<Vec<i64>>,
    }

    impl DualGraph {
        pub fn new(points: Vec<(i64, i64)>) -> Self {
            Self {
                cnt_vertices: points.len(),
                position: vec![Vec::new(); points.len()],
                graph: vec![Vec::new(); points.len()],
                points,
                dual_graph: Vec::new(),
                faces: Vec::new(),
            }
        }

        pub fn sort_edges(&mut self) {
            for i in 0..self.cnt_vertices {
                self.position[i].sort_unstable();
            }

            for i in 0..self.cnt_vertices {
                for j in 0..self.position[i].len() {
                    let e = &self.position[i][j];
                    let arg = Info::new(
                        self.points[i].0 - self.points[e.z as usize].0,
                        self.points[i].1 - self.points[e.z as usize].1,
                        -1,
                    );
                    let rev = self.position[e.z as usize].lower_bound(&arg);

                    self.graph[i].push(HalfEdge::new(e.z as usize, rev));
                }
            }
        }

        fn search_outside_face(&mut self) {
            let mut candidate: Option<(i64, i64, usize)> = None;

            for i in 0..self.cnt_vertices {
                if self.graph[i].is_empty() {
                    continue;
                }

                match candidate {
                    None => candidate = Some((self.points[i].1, self.points[i].0, i)),
                    Some((best_y, best_x, _)) => {
                        let cur_y = self.points[i].1;
                        let cur_x = self.points[i].0;
                        if cur_y > best_y || (cur_y == best_y && cur_x < best_x) {
                            candidate = Some((cur_y, cur_x, i));
                        }
                    }
                }
            }

            let (_, _, idx) = candidate.expect("graph has at least one edge");
            self.search(self.graph[idx][0].to, self.graph[idx][0].rev, 0, (idx, 0));
        }

        pub fn search(
            &mut self,
            mut curr: usize,
            mut idx: usize,
            face: usize,
            goal: (usize, usize),
        ) {
            loop {
                idx = if idx < self.graph[curr].len() - 1 {
                    idx + 1
                } else {
                    0
                };

                self.graph[curr][idx].face = face as i64;
                self.faces[face].push(curr as i64);

                if curr == goal.0 && idx == goal.1 {
                    break;
                }

                (curr, idx) = (self.graph[curr][idx].to, self.graph[curr][idx].rev);
            }
        }

        pub fn construct_dual_graph(&mut self, node_size: usize) {
            self.dual_graph = vec![Vec::new(); node_size];

            for i in 0..self.cnt_vertices {
                for e in self.graph[i].iter() {
                    self.dual_graph[e.face as usize]
                        .push(Edge::new(self.graph[e.to][e.rev].face as usize, (i, e.to)));
                }
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.position[u].push(Info::new(
                self.points[v].0 - self.points[u].0,
                self.points[v].1 - self.points[u].1,
                v as i64,
            ));
            self.position[v].push(Info::new(
                self.points[u].0 - self.points[v].0,
                self.points[u].1 - self.points[v].1,
                u as i64,
            ));
        }

        pub fn build(&mut self) -> usize {
            self.sort_edges();
            self.faces.push(Vec::new());
            self.search_outside_face();

            let mut cnt = 1;

            for i in 0..self.cnt_vertices {
                for j in 0..self.graph[i].len() {
                    let e = &self.graph[i][j];

                    if e.face < 0 {
                        self.faces.push(Vec::new());
                        self.search(e.to, e.rev, cnt, (i, j));
                        cnt += 1;
                    }
                }
            }

            self.construct_dual_graph(cnt);

            cnt
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut points = vec![(0, 0); n];

    for i in 0..n {
        points[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut edges = vec![(0, 0, 0); m];
    let mut edge_id_of = HashMap::with_capacity(m);
    let mut sum_cost = 0;

    for i in 0..m {
        let (mut a, mut b, c) = (
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<i64>(),
        );

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        edges[i] = (a, b, c);
        edge_id_of.insert((a, b), i);
        sum_cost += c;
    }

    if m == 0 {
        writeln!(out, "0").unwrap();

        for _ in 0..n {
            write!(out, "0 ").unwrap()
        }

        writeln!(out).unwrap();
        return;
    }

    // Step 1: Convert planar graph to dual graph
    let mut dual = DualGraph::new(points);

    for &(u, v, _) in edges.iter() {
        dual.add_edge(u, v);
    }

    let cnt_face = dual.build();
    let faces_odd = dual
        .faces
        .iter()
        .enumerate()
        .filter(|(_, c)| c.len() & 1 == 1)
        .map(|(id, _)| id)
        .collect::<Vec<_>>();

    // If no odd faces, the primal graph is already bipartite
    if faces_odd.is_empty() {
        writeln!(out, "{sum_cost}").unwrap();

        let mut graph = vec![Vec::new(); n];

        for &(u, v, _) in edges.iter() {
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut color = vec![None; n];
        let mut stack = Vec::new();

        for i in 0..n {
            if color[i].is_some() {
                continue;
            }

            color[i] = Some(0);
            stack.push(i);

            while let Some(u) = stack.pop() {
                let color_u = color[u].unwrap();

                for &v in graph[u].iter() {
                    match color[v] {
                        Some(color_v) => {
                            debug_assert_ne!(color_u, color_v);
                        }
                        None => {
                            color[v] = Some(color_u ^ 1);
                            stack.push(v);
                        }
                    }
                }
            }
        }

        for i in 0..n {
            write!(out, "{} ", color[i].unwrap()).unwrap();
        }

        writeln!(out).unwrap();
        return;
    }

    let mut graph_dual = vec![Vec::new(); cnt_face];

    for (face_id, edge_list) in dual.dual_graph.iter().enumerate() {
        for edge in edge_list.iter() {
            let (a, b) = edge.e;
            let key = if a < b { (a, b) } else { (b, a) };
            let edge_id = edge_id_of[&key];
            let cost = edges[edge_id].2;

            graph_dual[face_id].push((edge.to, edge_id, cost));
        }
    }

    // Step 2: Find the shortest path in the dual graph
    let mut dists = vec![vec![i64::MAX / 4; cnt_face]; faces_odd.len()];

    for (src_idx, &src_face) in faces_odd.iter().enumerate() {
        let mut dist = vec![i64::MAX / 4; cnt_face];
        let mut priority_queue = BinaryHeap::new();

        dist[src_face] = 0;
        priority_queue.push((Reverse(0), src_face));

        while let Some((Reverse(cost_curr), node_curr)) = priority_queue.pop() {
            if dist[node_curr] != cost_curr {
                continue;
            }

            for &(node_next, _, mut cost_next) in graph_dual[node_curr].iter() {
                cost_next += cost_curr;

                if dist[node_next] > cost_next {
                    dist[node_next] = cost_next;
                    priority_queue.push((Reverse(dist[node_next]), node_next));
                }
            }
        }

        dists[src_idx] = dist;
    }

    // Step 3: Perform maximum weight perfect matching
    let mut dist_max = 0;

    for i in 0..faces_odd.len() {
        for j in i + 1..faces_odd.len() {
            dist_max = max(dist_max, dists[i][faces_odd[j]]);
        }
    }

    // To make all weights non-negative
    let mut edges_matching = Vec::new();

    for i in 0..faces_odd.len() {
        for j in i + 1..faces_odd.len() {
            let d = dists[i][faces_odd[j]];
            edges_matching.push((i, j, dist_max + 1 - d));
        }
    }

    let mut matching = Matching::new(edges_matching);
    let mate = matching.solve();

    let mut edges_odd_circuit = HashSet::<usize>::new();

    // Build odd‑circuit cover by unpacking matched paths in the dual graph
    for i in 0..faces_odd.len() {
        let j = mate[i];

        if j == SENTINEL || i > j {
            continue;
        }

        let s = faces_odd[i];
        let t = faces_odd[j];

        let mut dist = vec![i64::MAX / 4; cnt_face];
        let mut priority_queue = BinaryHeap::new();
        let mut prev = vec![(usize::MAX, usize::MAX); cnt_face];

        dist[s] = 0;
        priority_queue.push((Reverse(0), s));

        while let Some((Reverse(cost_curr), node_curr)) = priority_queue.pop() {
            if node_curr == t {
                break;
            }

            if dist[node_curr] != cost_curr {
                continue;
            }

            for &(node_next, edge_id, mut cost_next) in graph_dual[node_curr].iter() {
                cost_next += cost_curr;

                if dist[node_next] > cost_next {
                    dist[node_next] = cost_next;
                    prev[node_next] = (node_curr, edge_id);
                    priority_queue.push((Reverse(dist[node_next]), node_next));
                }
            }
        }

        let mut curr = t;

        while curr != s {
            let (prev, edge_id) = prev[curr];

            edges_odd_circuit.insert(edge_id);
            curr = prev;
        }
    }

    let sum_weight = edges_odd_circuit.iter().map(|&id| edges[id].2).sum::<i64>();
    let mut is_cut = vec![true; m];

    for &edge_id in edges_odd_circuit.iter() {
        is_cut[edge_id] = false;
    }

    let mut color = vec![None; n];
    let mut stack = Vec::<usize>::new();
    let mut graph = vec![Vec::new(); n];

    for (edge_id, &(u, v, _)) in edges.iter().enumerate() {
        graph[u].push((v, edge_id));
        graph[v].push((u, edge_id));
    }

    for i in 0..n {
        if color[i].is_some() {
            continue;
        }

        color[i] = Some(0);
        stack.push(i);

        while let Some(u) = stack.pop() {
            let color_u = color[u].unwrap();

            for &(v, edge_id) in graph[u].iter() {
                let color_need = if is_cut[edge_id] {
                    color_u ^ 1
                } else {
                    color_u
                };

                match color[v] {
                    Some(color_v) => debug_assert_eq!(color_v, color_need),
                    None => {
                        color[v] = Some(color_need);
                        stack.push(v);
                    }
                }
            }
        }
    }

    writeln!(out, "{}", sum_cost - sum_weight).unwrap();

    for i in 0..n {
        write!(out, "{} ", color[i].unwrap()).unwrap();
    }

    writeln!(out).unwrap();
}

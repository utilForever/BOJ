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
            buf_str: Vec::new(),
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

// Reference: https://koosaga.com/258
fn main() {
    use maximum_weight_matching::*;

    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (_, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut edges = vec![(0, 0, 0); m];

    for i in 0..m {
        let (u, v) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        edges[i] = (u, v, 1);
    }

    let matching = Matching::new(edges).solve();
    let mut ret = 0;

    for (_, &v2) in matching.iter().enumerate() {
        if v2 == SENTINEL {
            continue;
        }

        ret += 1;
    }

    writeln!(out, "{}", ret / 2).unwrap();
}

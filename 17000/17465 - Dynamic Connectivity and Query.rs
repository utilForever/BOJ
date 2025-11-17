use io::Write;
use std::{
    collections::{HashMap, HashSet, hash_map::RandomState},
    hash::{BuildHasher, Hasher},
    iter::repeat_with,
};
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

#[derive(Clone, Debug)]
pub struct Xorshift {
    y: u64,
}

impl Xorshift {
    pub fn new_with_seed(seed: u64) -> Self {
        Xorshift { y: seed }
    }

    pub fn new() -> Self {
        Xorshift::new_with_seed(RandomState::new().build_hasher().finish())
    }

    pub fn rand64(&mut self) -> u64 {
        self.y ^= self.y << 5;
        self.y ^= self.y >> 17;
        self.y ^= self.y << 11;
        self.y
    }

    pub fn rand(&mut self, k: u64) -> u64 {
        self.rand64() % k
    }

    pub fn rands(&mut self, k: u64, n: usize) -> Vec<u64> {
        repeat_with(|| self.rand(k)).take(n).collect()
    }

    pub fn randf(&mut self) -> f64 {
        const UPPER_MASK: u64 = 0x3FF0_0000_0000_0000;
        const LOWER_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;
        let x = self.rand64();
        let tmp = UPPER_MASK | (x & LOWER_MASK);
        let result: f64 = f64::from_bits(tmp);
        f64::from_bits(f64::to_bits(result - 1.0) ^ (x >> 63))
    }

    pub fn gen_bool(&mut self, p: f64) -> bool {
        self.randf() < p
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let mut n = slice.len();
        while n > 1 {
            let i = self.rand(n as _) as usize;
            n -= 1;
            slice.swap(i, n);
        }
    }
}

impl Default for Xorshift {
    fn default() -> Self {
        Xorshift::new_with_seed(0x2b99_2ddf_a232_49d6)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Token {
    Anchor(usize),
    Edge {
        from: usize,
        to: usize,
        id: usize,
        dir: u8,
    },
}

#[derive(Debug, Clone)]
struct Node {
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
    priority: u64,
    size: usize,
    cnt_anchor: usize,
    id_token: usize,
}

impl Node {
    fn new(priority: u64, id_token: usize, is_anchor: bool) -> Self {
        Self {
            left: None,
            right: None,
            parent: None,
            priority,
            size: 1,
            cnt_anchor: if is_anchor { 1 } else { 0 },
            id_token,
        }
    }
}

struct EulerTourTree {
    nodes: Vec<Node>,
    tokens: Vec<Token>,
    token_node_map: Vec<Option<usize>>,
    anchors: Vec<usize>,
    rng: Xorshift,
}

impl EulerTourTree {
    fn new(n: usize, seed: u64) -> Self {
        let mut ett = Self {
            nodes: Vec::new(),
            tokens: Vec::new(),
            token_node_map: Vec::new(),
            anchors: vec![0; n],
            rng: Xorshift::new_with_seed(seed),
        };

        for i in 0..n {
            let id_token = ett.new_token(Token::Anchor(i));
            let id_node = ett.new_node(id_token);

            ett.token_node_map[id_token] = Some(id_node);
        }

        ett
    }

    fn new_token(&mut self, token: Token) -> usize {
        let id_token = self.tokens.len();

        self.tokens.push(token);
        self.token_node_map.push(None);

        if let Token::Anchor(v) = self.tokens[id_token] {
            self.anchors[v] = id_token;
        }

        id_token
    }

    fn new_node(&mut self, id_token: usize) -> usize {
        let is_anchor = matches!(self.tokens[id_token], Token::Anchor(_));
        let rng = self.rng.rand64();
        let id_node = self.nodes.len();

        self.nodes.push(Node::new(rng, id_token, is_anchor));

        id_node
    }
}

impl EulerTourTree {
    fn root(&self, mut x: usize) -> usize {
        while let Some(p) = self.nodes[x].parent {
            x = p;
        }

        x
    }

    fn _size(&self, r: usize) -> usize {
        self.nodes[r].size
    }

    fn cnt_anchor(&self, r: usize) -> usize {
        self.nodes[r].cnt_anchor
    }

    fn rank(&self, mut x: usize) -> usize {
        let mut rank = self.nodes[x].left.map(|i| self.nodes[i].size).unwrap_or(0) + 1;

        while let Some(p) = self.nodes[x].parent {
            if self.nodes[p].right == Some(x) {
                rank += self.nodes[p].left.map(|i| self.nodes[i].size).unwrap_or(0) + 1;
            }

            x = p;
        }

        rank
    }

    fn anchor(&self, v: usize) -> usize {
        self.anchors[v]
    }

    fn node_of_token(&self, tid: usize) -> Option<usize> {
        self.token_node_map[tid]
    }
}

impl EulerTourTree {
    fn update(&mut self, x: usize) {
        let (left, right) = (self.nodes[x].left, self.nodes[x].right);

        self.nodes[x].size = 1
            + left.map(|i| self.nodes[i].size).unwrap_or(0)
            + right.map(|i| self.nodes[i].size).unwrap_or(0);

        let cond = matches!(self.tokens[self.nodes[x].id_token], Token::Anchor(_));
        let is_anchor = if cond { 1 } else { 0 };

        self.nodes[x].cnt_anchor = is_anchor
            + left.map(|i| self.nodes[i].cnt_anchor).unwrap_or(0)
            + right.map(|i| self.nodes[i].cnt_anchor).unwrap_or(0);

        if let Some(left) = left {
            self.nodes[left].parent = Some(x);
        }

        if let Some(right) = right {
            self.nodes[right].parent = Some(x);
        }
    }

    fn set_left(&mut self, x: usize, node: Option<usize>) {
        self.nodes[x].left = node;

        if let Some(left) = node {
            self.nodes[left].parent = Some(x);
        }
    }

    fn set_right(&mut self, x: usize, node: Option<usize>) {
        self.nodes[x].right = node;

        if let Some(right) = node {
            self.nodes[right].parent = Some(x);
        }
    }

    fn merge(&mut self, a: Option<usize>, b: Option<usize>) -> Option<usize> {
        match (a, b) {
            (None, x) => x,
            (x, None) => x,
            (Some(x), Some(y)) => {
                if self.nodes[x].priority < self.nodes[y].priority {
                    let x_right = self.nodes[x].right;
                    let merged = self.merge(x_right, Some(y));

                    self.set_right(x, merged);
                    self.update(x);
                    self.nodes[x].parent = None;

                    Some(x)
                } else {
                    let y_left = self.nodes[y].left;
                    let merged = self.merge(Some(x), y_left);

                    self.set_left(y, merged);
                    self.update(y);
                    self.nodes[y].parent = None;

                    Some(y)
                }
            }
        }
    }

    fn split_by_rank(&mut self, root: Option<usize>, k: usize) -> (Option<usize>, Option<usize>) {
        if root.is_none() {
            return (None, None);
        }

        let x = root.unwrap();
        let size_left = self.nodes[x].left.map(|i| self.nodes[i].size).unwrap_or(0);

        if k <= size_left {
            let (a, b) = self.split_by_rank(self.nodes[x].left, k);

            self.set_left(x, b);
            self.update(x);

            if let Some(a) = a {
                self.nodes[a].parent = None;
            }

            (a, Some(x))
        } else {
            let (a, b) = self.split_by_rank(self.nodes[x].right, k - size_left - 1);

            self.set_right(x, a);
            self.update(x);

            if let Some(b) = b {
                self.nodes[b].parent = None;
            }

            (Some(x), b)
        }
    }

    fn _kth(&self, mut x: usize, mut k: usize) -> usize {
        loop {
            let size_left = self.nodes[x].left.map(|i| self.nodes[i].size).unwrap_or(0);

            if k <= size_left {
                x = self.nodes[x].left.unwrap();
            } else if k == size_left + 1 {
                return x;
            } else {
                k -= size_left + 1;
                x = self.nodes[x].right.unwrap();
            }
        }
    }

    fn make_root(&mut self, v: usize) {
        let id_token = self.anchor(v);
        let id_node = self.node_of_token(id_token).unwrap();
        let root = self.root(id_node);
        let rank = self.rank(id_node);
        let (a, b) = self.split_by_rank(Some(root), rank - 1);
        let merged = self.merge(b, a);

        if let Some(root) = merged {
            self.nodes[root].parent = None;
        }
    }

    fn connected(&self, u: usize, v: usize) -> bool {
        let token_u = self.node_of_token(self.anchor(u)).unwrap();
        let token_v = self.node_of_token(self.anchor(v)).unwrap();

        self.root(token_u) == self.root(token_v)
    }

    fn component_size(&self, v: usize) -> usize {
        let id_token = self.anchor(v);
        let id_node = self.node_of_token(id_token).unwrap();
        let root = self.root(id_node);

        self.cnt_anchor(root)
    }

    fn collect_vertices(&self, v: usize) -> Vec<usize> {
        let id_node = self.node_of_token(self.anchor(v)).unwrap();
        let root = self.root(id_node);
        let mut stack = Vec::new();
        let mut ret = Vec::new();

        stack.push(root);

        while let Some(x) = stack.pop() {
            if let Some(left) = self.nodes[x].left {
                stack.push(left);
            }

            if let Some(right) = self.nodes[x].right {
                stack.push(right);
            }

            let token = &self.tokens[self.nodes[x].id_token];

            if let Token::Anchor(v) = token {
                ret.push(*v);
            }
        }

        ret
    }

    fn link(&mut self, u: usize, v: usize, id_edge: usize) -> (usize, usize) {
        self.make_root(u);
        self.make_root(v);

        let token_uv = self.new_token(Token::Edge {
            from: u,
            to: v,
            dir: 0,
            id: id_edge,
        });
        let token_vu = self.new_token(Token::Edge {
            from: v,
            to: u,
            dir: 1,
            id: id_edge,
        });

        let node_uv = self.new_node(token_uv);
        let node_vu = self.new_node(token_vu);

        self.token_node_map[token_uv] = Some(node_uv);
        self.token_node_map[token_vu] = Some(node_vu);

        let root_u = {
            let id_node = self.node_of_token(self.anchor(u)).unwrap();
            self.root(id_node)
        };
        let root_v = {
            let id_node = self.node_of_token(self.anchor(v)).unwrap();
            self.root(id_node)
        };

        let merged1 = self.merge(Some(root_u), Some(node_uv));
        let merged2 = self.merge(merged1, Some(root_v));
        let merged3 = self.merge(merged2, Some(node_vu));

        if let Some(root) = merged3 {
            self.nodes[root].parent = None;
        }

        (token_uv, token_vu)
    }

    fn cut(&mut self, token_uv: usize, token_vu: usize) {
        let node_uv = self.node_of_token(token_uv).unwrap();
        let node_vu = self.node_of_token(token_vu).unwrap();
        let root = self.root(node_uv);

        let rank_uv = self.rank(node_uv);
        let rank_vu = self.rank(node_vu);
        let (root, rank_uv, rank_vu) = if rank_uv < rank_vu {
            (root, rank_uv, rank_vu)
        } else {
            let (a, b) = self.split_by_rank(Some(root), rank_uv - 1);
            let merged = self.merge(b, a).unwrap();

            let node_uv2 = self.node_of_token(token_uv).unwrap();
            let node_vu2 = self.node_of_token(token_vu).unwrap();
            (merged, self.rank(node_uv2), self.rank(node_vu2))
        };

        let (a, rest1) = self.split_by_rank(Some(root), rank_uv - 1);
        let (_uv, rest2) = self.split_by_rank(rest1, 1);
        let mid_len = rank_vu - rank_uv - 1;
        let (b, rest3) = self.split_by_rank(rest2, mid_len);
        let (_vu, c) = self.split_by_rank(rest3, 1);

        self.token_node_map[token_uv] = None;
        self.token_node_map[token_vu] = None;

        let merged = self.merge(a, c);

        if let Some(root) = merged {
            self.nodes[root].parent = None;
        }

        if let Some(root) = b {
            self.nodes[root].parent = None;
        }
    }
}

struct Edge {
    u: usize,
    v: usize,
    level: usize,
    is_tree: bool,
    tree_level_base: Option<usize>,
    tree_tokens: Vec<Option<(usize, usize)>>,
}

struct DynamicGraph {
    level: usize,
    edges: Vec<Edge>,
    forests: Vec<EulerTourTree>,
    non_tree_adj: Vec<Vec<HashSet<usize>>>,
}

impl DynamicGraph {
    fn new(n: usize) -> Self {
        let level = ((n - 1).next_power_of_two().trailing_zeros()) as usize;
        let mut forests = Vec::with_capacity(level + 1);

        for i in 0..=level {
            forests.push(EulerTourTree::new(n, (i as u64 + 123456789) * 987654321));
        }

        let mut non_tree_adj = Vec::with_capacity(level + 1);

        for _ in 0..=level {
            non_tree_adj.push((0..n).map(|_| HashSet::new()).collect());
        }

        Self {
            level,
            edges: Vec::new(),
            forests,
            non_tree_adj,
        }
    }

    fn connected(&self, u: usize, v: usize) -> bool {
        self.forests[0].connected(u, v)
    }

    fn insert(&mut self, u: usize, v: usize) -> usize {
        let id_edge = self.edges.len();
        let mut edge = Edge {
            u,
            v,
            level: 0,
            is_tree: false,
            tree_level_base: None,
            tree_tokens: vec![None; self.level + 1],
        };

        if !self.connected(u, v) {
            let (token_uv, token_vu) = self.forests[0].link(u, v, id_edge);

            edge.is_tree = true;
            edge.tree_level_base = Some(0);
            edge.tree_tokens[0] = Some((token_uv, token_vu));
        } else {
            self.non_tree_adj[0][u].insert(id_edge);
            self.non_tree_adj[0][v].insert(id_edge);
        }

        self.edges.push(edge);
        id_edge
    }

    fn demote_edge(&mut self, id_edge: usize, i: usize) {
        if i == 0 {
            return;
        }

        let (u, v) = (self.edges[id_edge].u, self.edges[id_edge].v);
        let _ = self.non_tree_adj[i][u].remove(&id_edge);
        let _ = self.non_tree_adj[i][v].remove(&id_edge);

        self.non_tree_adj[i - 1][u].insert(id_edge);
        self.non_tree_adj[i - 1][v].insert(id_edge);
        self.edges[id_edge].level = i - 1;
    }

    fn remove(&mut self, id_edge: usize) {
        if id_edge >= self.edges.len() {
            return;
        }

        let (u, v) = (self.edges[id_edge].u, self.edges[id_edge].v);

        if !self.edges[id_edge].is_tree {
            let idx = self.edges[id_edge].level;

            self.non_tree_adj[idx][u].remove(&id_edge);
            self.non_tree_adj[idx][v].remove(&id_edge);
            return;
        }

        let base = self.edges[id_edge].tree_level_base.unwrap();

        for i in base..=self.level {
            if let Some((token_uv, token_vu)) = self.edges[id_edge].tree_tokens[i].take() {
                self.forests[i].cut(token_uv, token_vu);
            }
        }

        self.edges[id_edge].is_tree = false;
        self.edges[id_edge].tree_level_base = None;

        for i in (0..=base).rev() {
            let size_u = self.forests[i].component_size(u);
            let size_v = self.forests[i].component_size(v);
            let rep = if size_u <= size_v { u } else { v };
            let verts = self.forests[i].collect_vertices(rep);

            let mut scanned = HashSet::new();
            let mut candidate_cross = None;

            'scan: for &x in verts.iter() {
                let list = self.non_tree_adj[i][x].iter().cloned().collect::<Vec<_>>();

                for id_edge2 in list {
                    if !scanned.insert(id_edge2) {
                        continue;
                    }

                    let (a, b) = (self.edges[id_edge2].u, self.edges[id_edge2].v);
                    let y = if a == x {
                        b
                    } else if b == x {
                        a
                    } else {
                        continue;
                    };

                    if !self.forests[i].connected(x, y) {
                        candidate_cross = Some(id_edge2);
                        break 'scan;
                    }
                }
            }

            if let Some(edge2) = candidate_cross {
                let (a, b) = (self.edges[edge2].u, self.edges[edge2].v);

                self.non_tree_adj[i][a].remove(&edge2);
                self.non_tree_adj[i][b].remove(&edge2);

                self.edges[edge2].is_tree = true;
                self.edges[edge2].tree_level_base = Some(i);

                for j in i..=self.level {
                    if !self.forests[j].connected(a, b) {
                        let (token_uv, token_vu) = self.forests[j].link(a, b, edge2);
                        self.edges[edge2].tree_tokens[j] = Some((token_uv, token_vu));
                    }
                }
                break;
            } else {
                let mut to_demote = Vec::new();

                for &x in verts.iter() {
                    for &edge2 in self.non_tree_adj[i][x].iter() {
                        to_demote.push(edge2);
                    }
                }

                to_demote.sort();
                to_demote.dedup();

                for edge2 in to_demote {
                    self.demote_edge(edge2, i);
                }
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut graph = DynamicGraph::new(n);
    let mut edges = HashMap::new();
    let mut cnt_components = n;
    let mut f = 0;

    for _ in 0..q {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        let (x, y) = ((a ^ f) % n, (b ^ f) % n);

        if x < y {
            if let Some(id) = edges.remove(&(x, y)) {
                graph.remove(id);

                if !graph.connected(x, y) {
                    cnt_components += 1;
                }
            } else {
                let connected = graph.connected(x, y);
                let id = graph.insert(x, y);

                edges.insert((x, y), id);

                if !connected {
                    cnt_components -= 1;
                }
            }
        } else {
            writeln!(out, "{}", if graph.connected(x, y) { 1 } else { 0 }).unwrap();
        }

        f += cnt_components;
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '+' => Operator::Add,
            '-' => Operator::Sub,
            '*' => Operator::Mul,
            '/' => Operator::Div,
            _ => unreachable!(),
        }
    }
}

struct Graph {
    head: Vec<usize>,
    to: Vec<usize>,
    next: Vec<usize>,
    idx: usize,
}

impl Graph {
    fn new(size_nodes: usize, size_edges: usize) -> Self {
        Self {
            head: vec![0; size_nodes + 1],
            to: vec![0; size_edges],
            next: vec![0; size_edges],
            idx: 1,
        }
    }

    fn add_edge_internal(&mut self, u: usize, v: usize) {
        self.to[self.idx] = v;
        self.next[self.idx] = self.head[u];
        self.head[u] = self.idx;
        self.idx += 1;
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.add_edge_internal(u, v);
        self.add_edge_internal(v, u);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeType {
    Empty,
    EdgeToVertex,
    EdgeToEdge,
    VertexToVertex,
    VertexToEdge,
}

impl NodeType {
    fn is_empty(self) -> bool {
        self == NodeType::Empty
    }

    fn starts_with_vertex(self) -> bool {
        matches!(self, NodeType::VertexToVertex | NodeType::VertexToEdge)
    }

    fn ends_with_edge(self) -> bool {
        matches!(self, NodeType::EdgeToEdge | NodeType::VertexToEdge)
    }

    fn from(vertex_start: bool, edge_end: bool) -> Self {
        match (vertex_start, edge_end) {
            (true, true) => NodeType::VertexToEdge,
            (true, false) => NodeType::VertexToVertex,
            (false, true) => NodeType::EdgeToEdge,
            (false, false) => NodeType::EdgeToVertex,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    factor_multiply: i64,
    factor_add: i64,
    val_vertex: i64,
    val_edge: i64,
    op_edge: Operator,
    node_type: NodeType,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            factor_multiply: 1,
            factor_add: 0,
            val_vertex: 0,
            val_edge: 0,
            op_edge: Operator::Add,
            node_type: NodeType::Empty,
        }
    }
}

impl Node {
    fn new_with_vertex(val_vertex: i64) -> Self {
        Self {
            val_vertex,
            node_type: NodeType::VertexToVertex,
            ..Default::default()
        }
    }

    fn new_with_edge(val_edge: i64, op_edge: Operator) -> Self {
        Self {
            val_edge,
            op_edge,
            node_type: NodeType::EdgeToEdge,
            ..Default::default()
        }
    }

    fn compose(a1: i64, b1: i64, a2: i64, b2: i64) -> (i64, i64) {
        if a2 == 1 {
            if b2 == 0 {
                (a1, b1)
            } else {
                (a1, (b1 + b2) % MOD)
            }
        } else if a1 == 1 && b1 == 0 {
            (a2, b2)
        } else if a2 == 0 {
            (0, b2)
        } else if a1 == 0 {
            let b = (a2 * b1 + b2) % MOD;
            (0, b)
        } else {
            let a = (a1 * a2) % MOD;
            let b = (a2 * b1 + b2) % MOD;
            (a, b)
        }
    }

    fn convert_op(op: Operator, val: i64, inv: &Vec<i64>) -> (i64, i64) {
        match op {
            Operator::Add => (1, val),
            Operator::Sub => (1, if val == 0 { 0 } else { MOD - val }),
            Operator::Mul => (val, 0),
            Operator::Div => {
                if val == 0 {
                    (0, 0)
                } else {
                    (inv[val as usize], 0)
                }
            }
        }
    }

    fn concat(left: Node, right: Node, inv: &Vec<i64>) -> Node {
        if left.node_type.is_empty() {
            return right;
        }

        if right.node_type.is_empty() {
            return left;
        }

        let starts_with_vertex = left.node_type.starts_with_vertex();
        let ends_with_edge = right.node_type.ends_with_edge();
        let mut ret = Node::default();

        ret.node_type = NodeType::from(starts_with_vertex, ends_with_edge);

        if starts_with_vertex {
            ret.val_vertex = left.val_vertex;
        }

        if ends_with_edge {
            ret.val_edge = right.val_edge;
            ret.op_edge = right.op_edge;
        }

        if left.node_type.ends_with_edge() {
            let val = left.val_edge * 10000 + right.val_vertex;
            let (a, b) = Self::convert_op(left.op_edge, val, inv);

            // ret = right ∘ (a, b) ∘ left
            let (a_new, b_new) = Self::compose(left.factor_multiply, left.factor_add, a, b);
            let (a_new, b_new) =
                Self::compose(a_new, b_new, right.factor_multiply, right.factor_add);

            ret.factor_multiply = a_new;
            ret.factor_add = b_new;
        } else {
            let (a_new, b_new) = Self::compose(
                left.factor_multiply,
                left.factor_add,
                right.factor_multiply,
                right.factor_add,
            );

            ret.factor_multiply = a_new;
            ret.factor_add = b_new;
        }

        ret
    }

    fn calculate(&self) -> i64 {
        (self.factor_multiply * self.val_vertex + self.factor_add) % MOD
    }
}

#[derive(Debug)]
struct SegmentTree {
    size: usize,
    data_forward: Vec<Node>,
    data_backward: Vec<Node>,
    inv: Vec<i64>,
}

impl SegmentTree {
    fn new(n: usize, inv: Vec<i64>) -> Self {
        let mut size = 1;

        while size < n {
            size *= 2;
        }

        Self {
            size,
            data_forward: vec![Node::default(); size * 2],
            data_backward: vec![Node::default(); size * 2],
            inv,
        }
    }

    fn build(&mut self, data: &Vec<Node>) {
        let n = data.len();

        for i in 0..n {
            self.data_forward[self.size + i] = data[i];
            self.data_backward[self.size + i] = data[i];
        }

        for i in (1..self.size).rev() {
            let forward_left = self.data_forward[i * 2];
            let forward_right = self.data_forward[i * 2 + 1];
            self.data_forward[i] = Node::concat(forward_left, forward_right, &self.inv);

            let backward_left = self.data_backward[i * 2];
            let backward_right = self.data_backward[i * 2 + 1];
            self.data_backward[i] = Node::concat(backward_right, backward_left, &self.inv);
        }
    }

    fn update(&mut self, idx: usize, value: Node) {
        let mut idx = self.size + idx;

        self.data_forward[idx] = value;
        self.data_backward[idx] = value;
        idx /= 2;

        while idx != 0 {
            let forward_left = self.data_forward[idx * 2];
            let forward_right = self.data_forward[idx * 2 + 1];
            self.data_forward[idx] = Node::concat(forward_left, forward_right, &self.inv);

            let backward_left = self.data_backward[idx * 2];
            let backward_right = self.data_backward[idx * 2 + 1];
            self.data_backward[idx] = Node::concat(backward_right, backward_left, &self.inv);

            idx /= 2;
        }
    }

    fn query_forward(&self, left: usize, right: usize) -> Node {
        let mut idx_left = self.size + left;
        let mut idx_right = self.size + right + 1;
        let mut acc_left = Node::default();
        let mut acc_right = Node::default();

        while idx_left < idx_right {
            if idx_left % 2 == 1 {
                acc_left = Node::concat(acc_left, self.data_forward[idx_left], &self.inv);
                idx_left += 1;
            }

            if idx_right % 2 == 1 {
                idx_right -= 1;
                acc_right = Node::concat(self.data_forward[idx_right], acc_right, &self.inv);
            }

            idx_left /= 2;
            idx_right /= 2;
        }

        Node::concat(acc_left, acc_right, &self.inv)
    }

    fn query_backward(&self, left: usize, right: usize) -> Node {
        let mut idx_left = self.size + left;
        let mut idx_right = self.size + right + 1;
        let mut acc_left = Node::default();
        let mut acc_right = Node::default();

        while idx_left < idx_right {
            if idx_left % 2 == 1 {
                acc_left = Node::concat(self.data_backward[idx_left], acc_left, &self.inv);
                idx_left += 1;
            }

            if idx_right % 2 == 1 {
                idx_right -= 1;
                acc_right = Node::concat(acc_right, self.data_backward[idx_right], &self.inv);
            }

            idx_left /= 2;
            idx_right /= 2;
        }

        Node::concat(acc_right, acc_left, &self.inv)
    }
}

struct HLD {
    parent: Vec<usize>,
    depth: Vec<usize>,
    head: Vec<usize>,
    pos: Vec<usize>,
    pos_inv: Vec<usize>,
}

impl HLD {
    fn new(graph: &Graph, root: usize, n: usize) -> Self {
        let mut parent = vec![0; n + 1];
        let mut depth = vec![0; n + 1];

        let mut order = Vec::with_capacity(n);
        let mut stack = Vec::with_capacity(n);

        parent[root] = 0;
        depth[root] = 0;
        stack.push(root);

        while let Some(u) = stack.pop() {
            order.push(u);

            let mut ptr = graph.head[u];

            while ptr != 0 {
                let v = graph.to[ptr];

                if v != parent[u] {
                    parent[v] = u;
                    depth[v] = depth[u] + 1;
                    stack.push(v);
                }

                ptr = graph.next[ptr];
            }
        }

        let mut size = vec![0; n + 1];
        let mut heavy = vec![0; n + 1];

        for &u in order.iter().rev() {
            size[u] = 1;

            let mut max_size = 0;
            let mut max_heavy = 0;
            let mut ptr = graph.head[u];

            while ptr != 0 {
                let v = graph.to[ptr];

                if parent[v] == u {
                    size[u] += size[v];

                    if size[v] > max_size {
                        max_size = size[v];
                        max_heavy = v;
                    }
                }

                ptr = graph.next[ptr];
            }

            heavy[u] = max_heavy;
        }

        let mut head = vec![0; n + 1];
        let mut pos = vec![0; n + 1];
        let mut pos_inv = vec![0; n];
        let mut pos_curr = 0;

        let mut stack = Vec::with_capacity(n);
        stack.push((root, root));

        while let Some((u_start, h)) = stack.pop() {
            let mut u = u_start;

            while u != 0 {
                head[u] = h;
                pos[u] = pos_curr;
                pos_inv[pos_curr] = u;
                pos_curr += 1;

                let mut ptr = graph.head[u];

                while ptr != 0 {
                    let v = graph.to[ptr];

                    if parent[v] == u && v != heavy[u] {
                        stack.push((v, v));
                    }

                    ptr = graph.next[ptr];
                }

                u = heavy[u];
            }
        }

        Self {
            parent,
            depth,
            head,
            pos,
            pos_inv,
        }
    }

    fn position(&self, node: usize) -> usize {
        self.pos[node]
    }

    fn query_path(&self, tree: &SegmentTree, mut u: usize, mut v: usize) -> Node {
        let mut acc_left = Node::default();
        let mut acc_right = Node::default();

        while self.head[u] != self.head[v] {
            let head_u = self.head[u];
            let head_v = self.head[v];

            if self.depth[head_u] >= self.depth[head_v] {
                let left = self.pos[head_u];
                let right = self.pos[u];
                let val = tree.query_backward(left, right);

                acc_left = Node::concat(acc_left, val, &tree.inv);
                u = self.parent[head_u];
            } else {
                let left = self.pos[head_v];
                let right = self.pos[v];
                let val = tree.query_forward(left, right);

                acc_right = Node::concat(val, acc_right, &tree.inv);
                v = self.parent[head_v];
            }
        }

        if self.depth[u] >= self.depth[v] {
            let val = tree.query_backward(self.pos[v], self.pos[u]);
            acc_left = Node::concat(acc_left, val, &tree.inv);
        } else {
            let val = tree.query_forward(self.pos[u], self.pos[v]);
            acc_right = Node::concat(val, acc_right, &tree.inv);
        }

        Node::concat(acc_left, acc_right, &tree.inv)
    }
}

const MOD: i64 = 1_208_017;
const OPERAND_MAX: usize = 999_999;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut cards_integer_vertices = vec![(0, 0); n + 1];

    for i in 1..=n {
        cards_integer_vertices[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut cards_integer_edges = vec![0; n];
    let mut cards_operator_edges = vec![Operator::Add; n];

    let size_nodes = 2 * n - 1;
    let size_edges = 4 * (n - 1) + 5;
    let mut graph = Graph::new(size_nodes, size_edges);

    for i in 1..=n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        let p = Operator::from(scan.token::<char>());
        let c = scan.token::<i64>();

        cards_operator_edges[i] = p;
        cards_integer_edges[i] = c;

        let idx_node = n + i;
        graph.add_edge(u, idx_node);
        graph.add_edge(v, idx_node);
    }

    let mut inv = vec![0; OPERAND_MAX + 1];
    inv[1] = 1;

    for i in 2..=OPERAND_MAX {
        let (q, r) = (MOD / i as i64, MOD % i as i64);
        let val = (MOD - q * inv[r as usize] % MOD) % MOD;

        inv[i] = val;
    }

    let hld = HLD::new(&graph, 1, size_nodes);
    let mut base = vec![Node::default(); size_nodes];

    for i in 0..size_nodes {
        let pos = hld.pos_inv[i];

        if pos <= n {
            let (a, b) = cards_integer_vertices[pos];
            base[i] = Node::new_with_vertex(a * 100 + b);
        } else {
            base[i] =
                Node::new_with_edge(cards_integer_edges[pos - n], cards_operator_edges[pos - n]);
        }
    }

    let mut tree = SegmentTree::new(size_nodes, inv);
    tree.build(&base);

    for _ in 0..q {
        let cmd = scan.token::<char>();

        match cmd {
            '?' => {
                let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
                writeln!(out, "{}", hld.query_path(&tree, x, y).calculate()).unwrap();
            }
            'A' => {
                let (x, y) = (scan.token::<usize>(), scan.token::<i64>());
                cards_integer_vertices[x].0 = y;

                let val_new = cards_integer_vertices[x].0 * 100 + cards_integer_vertices[x].1;
                tree.update(hld.position(x), Node::new_with_vertex(val_new));
            }
            'B' => {
                let (x, y) = (scan.token::<usize>(), scan.token::<i64>());
                cards_integer_vertices[x].1 = y;

                let val_new = cards_integer_vertices[x].0 * 100 + cards_integer_vertices[x].1;
                tree.update(hld.position(x), Node::new_with_vertex(val_new));
            }
            'P' => {
                let (x, y) = (scan.token::<usize>(), scan.token::<char>());
                let op_new = Operator::from(y);

                cards_operator_edges[x] = op_new;

                let idx = n + x;
                tree.update(
                    hld.position(idx),
                    Node::new_with_edge(cards_integer_edges[x], op_new),
                );
            }
            'C' => {
                let (x, y) = (scan.token::<usize>(), scan.token::<i64>());
                cards_integer_edges[x] = y;

                let idx = n + x;
                tree.update(
                    hld.position(idx),
                    Node::new_with_edge(y, cards_operator_edges[x]),
                );
            }
            _ => unreachable!(),
        }
    }
}
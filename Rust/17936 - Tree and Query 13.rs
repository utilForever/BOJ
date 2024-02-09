use io::Write;
use std::{io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    fn token<T: str::FromStr>(&mut self) -> T {
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

#[derive(Clone, Copy)]
pub struct Data {
    sum: i64,
    min: i64,
    max: i64,
    size: i64,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            sum: 0,
            min: i64::MAX,
            max: i64::MIN,
            size: 0,
        }
    }
}

impl Data {
    fn new(val: i64) -> Self {
        Self {
            sum: val,
            min: val,
            max: val,
            size: 1,
        }
    }

    fn init(sum: i64, min: i64, max: i64, size: i64) -> Self {
        Self {
            sum,
            min,
            max,
            size,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Lazy {
    a: i64,
    b: i64,
}

impl Default for Lazy {
    fn default() -> Self {
        Self { a: 1, b: 0 }
    }
}

impl Lazy {
    fn new(a: i64, b: i64) -> Self {
        Self { a, b }
    }

    fn lazy(&self) -> bool {
        self.a != 1 || self.b != 0
    }
}

mod utils {
    use crate::{Data, Lazy};

    pub fn add_data_and_data(a: Data, b: Data) -> Data {
        Data {
            sum: a.sum + b.sum,
            min: a.min.min(b.min),
            max: a.max.max(b.max),
            size: a.size + b.size,
        }
    }

    pub fn add_lazy_and_lazy(a: Lazy, b: Lazy) -> Lazy {
        Lazy::new(a.a * b.a, a.b * b.a + b.b)
    }

    pub fn add_integer_and_lazy(a: i64, b: Lazy) -> i64 {
        a * b.a + b.b
    }

    pub fn add_data_and_lazy(a: Data, b: Lazy) -> Data {
        if a.size == 0 {
            a
        } else {
            Data::init(
                a.sum * b.a + b.b * a.size,
                a.min * b.a + b.b,
                a.max * b.a + b.b,
                a.size,
            )
        }
    }
}

// NOTE:
// - childs[0] and childs[1] are preferred children
// - childs[2] and childs[3] are virtual children
#[derive(Clone, Copy)]
pub struct Node {
    parent: usize,
    childs: [usize; 4],
    val: i64,
    path: Data,
    subtree: Data,
    all: Data,
    lazy_path: Lazy,
    lazy_subtree: Lazy,
    flip: bool,
    fake: bool,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            parent: 0,
            childs: [0, 0, 0, 0],
            val: 0,
            path: Data::default(),
            subtree: Data::default(),
            all: Data::default(),
            lazy_path: Lazy::default(),
            lazy_subtree: Lazy::default(),
            flip: false,
            fake: true,
        }
    }
}

impl Node {
    fn new(val: i64) -> Self {
        Node {
            parent: 0,
            childs: [0, 0, 0, 0],
            val,
            path: Data::new(val),
            subtree: Data::default(),
            all: Data::new(val),
            lazy_path: Lazy::default(),
            lazy_subtree: Lazy::default(),
            flip: false,
            fake: false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct FakeNode {
    idx: usize,
}

impl FakeNode {
    fn new(idx: usize) -> Self {
        Self { idx }
    }
}

struct TopTree {
    nodes: Vec<Node>,
    nodes_fake: Vec<FakeNode>,
}

impl TopTree {
    fn new(n: usize) -> Self {
        let nodes = vec![Node::default(); 2 * n + 1];
        let mut nodes_fake = Vec::new();

        for i in n + 1..=2 * n {
            nodes_fake.push(FakeNode::new(i));
        }

        Self { nodes, nodes_fake }
    }

    fn init(&mut self, nodes: Vec<Node>) {
        self.nodes[1..nodes.len()].copy_from_slice(&nodes[1..]);
    }

    fn push_flip(&mut self, u: usize) {
        if u == 0 {
            return;
        }

        self.nodes[u].childs.swap(0, 1);
        self.nodes[u].flip ^= true;
    }

    fn push_path(&mut self, u: usize, lazy: Lazy) {
        if u == 0 || self.nodes[u].fake {
            return;
        }

        self.nodes[u].val = utils::add_integer_and_lazy(self.nodes[u].val, lazy);
        self.nodes[u].path = utils::add_data_and_lazy(self.nodes[u].path, lazy);
        self.nodes[u].all = utils::add_data_and_data(self.nodes[u].path, self.nodes[u].subtree);
        self.nodes[u].lazy_path = utils::add_lazy_and_lazy(self.nodes[u].lazy_path, lazy);
    }

    fn push_subtree(&mut self, u: usize, is_virtual: bool, lazy: Lazy) {
        if u == 0 {
            return;
        }

        self.nodes[u].subtree = utils::add_data_and_lazy(self.nodes[u].subtree, lazy);
        self.nodes[u].lazy_subtree = utils::add_lazy_and_lazy(self.nodes[u].lazy_subtree, lazy);

        if !self.nodes[u].fake && is_virtual {
            self.push_path(u, lazy);
        } else {
            self.nodes[u].all = utils::add_data_and_data(self.nodes[u].path, self.nodes[u].subtree);
        }
    }

    fn push(&mut self, u: usize) {
        if u == 0 {
            return;
        }

        if self.nodes[u].flip {
            self.push_flip(self.nodes[u].childs[0]);
            self.push_flip(self.nodes[u].childs[1]);
            self.nodes[u].flip = false;
        }

        if self.nodes[u].lazy_path.lazy() {
            self.push_path(self.nodes[u].childs[0], self.nodes[u].lazy_path);
            self.push_path(self.nodes[u].childs[1], self.nodes[u].lazy_path);
            self.nodes[u].lazy_path = Lazy::default();
        }

        if self.nodes[u].lazy_subtree.lazy() {
            self.push_subtree(self.nodes[u].childs[0], false, self.nodes[u].lazy_subtree);
            self.push_subtree(self.nodes[u].childs[1], false, self.nodes[u].lazy_subtree);
            self.push_subtree(self.nodes[u].childs[2], true, self.nodes[u].lazy_subtree);
            self.push_subtree(self.nodes[u].childs[3], true, self.nodes[u].lazy_subtree);
            self.nodes[u].lazy_subtree = Lazy::default();
        }
    }

    fn pull(&mut self, u: usize) {
        if !self.nodes[u].fake {
            let sum_path_preferred = utils::add_data_and_data(
                self.nodes[self.nodes[u].childs[0]].path,
                self.nodes[self.nodes[u].childs[1]].path,
            );

            self.nodes[u].path =
                utils::add_data_and_data(sum_path_preferred, Data::new(self.nodes[u].val));
        }

        let sum_subtree_preferred = utils::add_data_and_data(
            self.nodes[self.nodes[u].childs[0]].subtree,
            self.nodes[self.nodes[u].childs[1]].subtree,
        );
        let sum_all_virtual = utils::add_data_and_data(
            self.nodes[self.nodes[u].childs[2]].all,
            self.nodes[self.nodes[u].childs[3]].all,
        );

        self.nodes[u].subtree = utils::add_data_and_data(sum_subtree_preferred, sum_all_virtual);
        self.nodes[u].all = utils::add_data_and_data(self.nodes[u].path, self.nodes[u].subtree);
    }

    fn attach(&mut self, u: usize, direction: usize, v: usize) {
        self.nodes[u].childs[direction] = v;
        self.nodes[v].parent = u;

        self.pull(u);
    }

    fn direction(&self, u: usize, pos: usize) -> i64 {
        let v = self.nodes[u].parent;

        if self.nodes[v].childs[pos] == u {
            pos as i64
        } else if self.nodes[v].childs[pos + 1] == u {
            pos as i64 + 1
        } else {
            -1
        }
    }

    fn rotate(&mut self, u: usize, pos: usize) {
        let v = self.nodes[u].parent;
        let w = self.nodes[v].parent;
        let dir_u = self.direction(u, pos);
        let mut dir_v = self.direction(v, pos);

        if dir_v == -1 && pos == 0 {
            dir_v = self.direction(v, 2);
        }

        if dir_u != -1 {
            let dir_u = dir_u as usize;
            self.attach(v, dir_u, self.nodes[u].childs[dir_u ^ 1]);
            self.attach(u, dir_u ^ 1, v);
        }

        if dir_v != -1 {
            let dir_v = dir_v as usize;
            self.attach(w, dir_v, u);
        } else {
            self.nodes[u].parent = w;
        }
    }

    fn splay(&mut self, u: usize, pos: usize) {
        self.push(u);

        while self.direction(u, pos) != -1 && (pos == 0 || self.nodes[self.nodes[u].parent].fake) {
            let v = self.nodes[u].parent;
            let w = self.nodes[v].parent;

            self.push(w);
            self.push(v);
            self.push(u);

            let dir_u = self.direction(u, pos);
            let dir_v = self.direction(v, pos);

            if dir_v != -1 && (pos == 0 || self.nodes[w].fake) {
                if dir_u == dir_v {
                    self.rotate(v, pos);
                } else {
                    self.rotate(u, pos);
                }
            }

            self.rotate(u, pos);
        }
    }

    fn add(&mut self, u: usize, v: usize) {
        if v == 0 {
            return;
        }

        for i in 2..4 {
            if self.nodes[u].childs[i] == 0 {
                self.attach(u, i, v);
                return;
            }
        }

        let w = self.nodes_fake.pop().unwrap().idx;

        self.attach(w, 2, self.nodes[u].childs[2]);
        self.attach(w, 3, v);
        self.attach(u, 2, w);
    }

    fn push_recursive(&mut self, u: usize) {
        if self.nodes[u].fake {
            self.push_recursive(self.nodes[u].parent);
        }

        self.push(u);
    }

    fn remove(&mut self, u: usize) {
        let v = self.nodes[u].parent;
        self.push_recursive(v);

        let dir_u = self.direction(u, 2);
        let dir_v = self.direction(v, 2);

        if self.nodes[v].fake {
            let w = self.nodes[v].parent;

            if dir_u != -1 && dir_v != -1 {
                let dir_u = dir_u as usize;
                let dir_v = dir_v as usize;
                self.attach(w, dir_v, self.nodes[v].childs[dir_u ^ 1]);
            }

            if self.nodes[w].fake {
                self.splay(w, 2);
            }

            self.nodes_fake.push(FakeNode::new(v));
        } else {
            if dir_u != -1 {
                let dir_u = dir_u as usize;
                self.attach(v, dir_u, 0);
            }
        }

        self.nodes[u].parent = 0;
    }

    fn parent(&mut self, u: usize) -> usize {
        let v = self.nodes[u].parent;

        if !self.nodes[v].fake {
            return v;
        }

        self.splay(v, 2);

        self.nodes[v].parent
    }

    fn access(&mut self, u: usize) -> usize {
        let mut v = u;

        self.splay(u, 0);
        self.add(u, self.nodes[u].childs[1]);
        self.attach(u, 1, 0);

        while self.nodes[u].parent != 0 {
            v = self.parent(u);

            self.splay(v, 0);
            self.remove(u);
            self.add(v, self.nodes[v].childs[1]);
            self.attach(v, 1, u);
            self.splay(u, 0);
        }

        v
    }

    fn make_root(&mut self, u: usize) {
        self.access(u);
        self.push_flip(u);
    }

    fn link(&mut self, u: usize, v: usize) {
        self.make_root(u);
        self.access(v);
        self.add(v, u);
    }

    fn _cut(&mut self, u: usize, v: usize) {
        self.make_root(u);
        self.access(v);

        self.nodes[u].parent = 0;
        self.nodes[v].childs[0] = 0;

        self.pull(v);
    }
}

// Reference: https://infossm.github.io/blog/2021/03/21/toptree/
// Reference: https://codeforces.com/blog/entry/103726
// Reference: https://etyu39.tistory.com/8
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nodes = vec![Node::default(); n + 1];
    let mut edges = vec![(0, 0); n - 1];

    for edge in edges.iter_mut().take(n - 1) {
        *edge = (scan.token::<usize>(), scan.token::<usize>());
    }

    for node in nodes.iter_mut().take(n + 1).skip(1) {
        *node = Node::new(scan.token::<i64>());
    }

    let mut r = scan.token::<usize>();

    let mut top_tree = TopTree::new(n);
    top_tree.init(nodes);

    for (x, y) in edges {
        top_tree.link(x, y);
    }

    for _ in 0..m {
        let op = scan.token::<i64>();

        if op == 0 {
            let (x, y) = (scan.token::<usize>(), scan.token::<i64>());
            let lazy = Lazy::new(0, y);

            top_tree.make_root(r);
            top_tree.access(x);
            top_tree.nodes[x].val = utils::add_integer_and_lazy(top_tree.nodes[x].val, lazy);

            for i in 2..4 {
                top_tree.push_subtree(top_tree.nodes[x].childs[i], true, lazy);
            }
        } else if op == 1 {
            r = scan.token::<usize>();
        } else if op == 2 {
            let (x, y, z) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );

            top_tree.make_root(x);
            top_tree.access(y);
            top_tree.push_path(y, Lazy::new(0, z));
        } else if op == 3 {
            let x = scan.token::<usize>();

            top_tree.make_root(r);
            top_tree.access(x);

            let mut ret = Data::new(top_tree.nodes[x].val);

            for i in 2..4 {
                ret =
                    utils::add_data_and_data(ret, top_tree.nodes[top_tree.nodes[x].childs[i]].all);
            }

            writeln!(out, "{}", ret.min).unwrap();
        } else if op == 4 {
            let x = scan.token::<usize>();

            top_tree.make_root(r);
            top_tree.access(x);

            let mut ret = Data::new(top_tree.nodes[x].val);

            for i in 2..4 {
                ret =
                    utils::add_data_and_data(ret, top_tree.nodes[top_tree.nodes[x].childs[i]].all);
            }

            writeln!(out, "{}", ret.max).unwrap();
        } else if op == 5 {
            let (x, y) = (scan.token::<usize>(), scan.token::<i64>());
            let lazy = Lazy::new(1, y);

            top_tree.make_root(r);
            top_tree.access(x);
            top_tree.nodes[x].val = utils::add_integer_and_lazy(top_tree.nodes[x].val, lazy);

            for i in 2..4 {
                top_tree.push_subtree(top_tree.nodes[x].childs[i], true, lazy);
            }
        } else if op == 6 {
            let (x, y, z) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );

            top_tree.make_root(x);
            top_tree.access(y);
            top_tree.push_path(y, Lazy::new(1, z));
        } else if op == 7 {
            let (x, y) = (scan.token::<usize>(), scan.token::<usize>());

            top_tree.make_root(x);
            top_tree.access(y);

            let ret = top_tree.nodes[y].path;
            writeln!(out, "{}", ret.min).unwrap();
        } else if op == 8 {
            let (x, y) = (scan.token::<usize>(), scan.token::<usize>());

            top_tree.make_root(x);
            top_tree.access(y);

            let ret = top_tree.nodes[y].path;
            writeln!(out, "{}", ret.max).unwrap();
        } else if op == 9 {
            let (x, y) = (scan.token::<usize>(), scan.token::<usize>());

            top_tree.make_root(r);
            top_tree.access(y);

            if top_tree.access(x) == x {
                continue;
            }

            let child = top_tree.nodes[x].childs[0];
            top_tree.nodes[child].parent = 0;
            top_tree.nodes[x].childs[0] = 0;

            top_tree.pull(x);
            top_tree.access(y);
            top_tree.add(y, x);
        } else if op == 10 {
            let (x, y) = (scan.token::<usize>(), scan.token::<usize>());

            top_tree.make_root(x);
            top_tree.access(y);

            let ret = top_tree.nodes[y].path;
            writeln!(out, "{}", ret.sum).unwrap();
        } else {
            let x = scan.token::<usize>();

            top_tree.make_root(r);
            top_tree.access(x);

            let mut ret = Data::new(top_tree.nodes[x].val);

            for i in 2..4 {
                ret =
                    utils::add_data_and_data(ret, top_tree.nodes[top_tree.nodes[x].childs[i]].all);
            }

            writeln!(out, "{}", ret.sum).unwrap();
        }
    }
}

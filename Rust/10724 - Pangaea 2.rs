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
}

#[derive(Debug)]
struct Node {
    left: *mut Node,
    right: *mut Node,
    parent: *mut Node,
    value: i64,
    idx: usize,
    max: (i64, usize),
    flip: bool,
}

impl Node {
    fn new(value: i64, idx: usize) -> *mut Self {
        Box::into_raw(Box::new(Self {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent: std::ptr::null_mut(),
            value,
            idx,
            max: (value, idx),
            flip: false,
        }))
    }

    fn init(&mut self, value: i64, idx: usize) {
        self.left = std::ptr::null_mut();
        self.right = std::ptr::null_mut();
        self.parent = std::ptr::null_mut();
        self.value = value;
        self.idx = idx;
        self.max = (value, idx);
        self.flip = false;
    }

    unsafe fn is_root(&mut self) -> bool {
        self.parent == std::ptr::null_mut()
            || ((*self.parent).left != self && (*self.parent).right != self)
    }

    unsafe fn is_left(&mut self) -> bool {
        (*self).parent != std::ptr::null_mut() && (*self.parent).left == self
    }

    unsafe fn rotate(&mut self) {
        (*(*self).parent).push();
        (*self).push();

        if (*self).is_left() {
            if self.right != std::ptr::null_mut() {
                (*self.right).parent = self.parent;
            }

            (*self.parent).left = self.right;
            self.right = self.parent;
        } else {
            if self.left != std::ptr::null_mut() {
                (*self.left).parent = self.parent;
            }

            (*self.parent).right = self.left;
            self.left = self.parent;
        }

        if !(*self.parent).is_root() {
            if (*self.parent).is_left() {
                (*(*self.parent).parent).left = self;
            } else {
                (*(*self.parent).parent).right = self;
            }
        }

        let temp = self.parent;
        self.parent = (*temp).parent;
        (*temp).parent = self;

        (*temp).update();
        self.update();
    }

    unsafe fn update(&mut self) {
        self.max = (self.value, self.idx);

        if self.left != std::ptr::null_mut() {
            self.max = self.max.max((*self.left).max);
        }

        if self.right != std::ptr::null_mut() {
            self.max = self.max.max((*self.right).max);
        }
    }

    unsafe fn push(&mut self) {
        if !self.flip {
            return;
        }

        let temp = (*self).left;
        (*self).left = (*self).right;
        (*self).right = temp;

        if (*self).left != std::ptr::null_mut() {
            (*(*self).left).flip ^= self.flip;
        }

        if (*self).right != std::ptr::null_mut() {
            (*(*self).right).flip ^= self.flip;
        }

        self.flip = false;
    }
}

struct LinkCutTree {
    nodes: Vec<*mut Node>,
}

impl LinkCutTree {
    unsafe fn new(n: usize) -> Self {
        let mut nodes = vec![std::ptr::null_mut(); n + 1];

        for i in 1..=n {
            nodes[i] = Node::new(0, 0);
        }

        Self { nodes }
    }

    unsafe fn get_parent(&mut self, mut node: *mut Node) -> *mut Node {
        // Make chain to root
        self.access(node);

        // node is root
        if (*node).left == std::ptr::null_mut() {
            return std::ptr::null_mut();
        }

        // Get predecessor
        node = (*node).left;
        (*node).push();

        while (*node).right != std::ptr::null_mut() {
            node = (*node).right;
            (*node).push();
        }

        // Amortized
        self.access(node);

        node
    }

    unsafe fn get_lca(&mut self, x: *mut Node, y: *mut Node) -> *mut Node {
        self.access(x);
        self.access(y);
        self.splay(x);

        if (*x).parent == std::ptr::null_mut() {
            x
        } else {
            (*x).parent
        }
    }

    unsafe fn splay(&mut self, node: *mut Node) {
        while !(*node).is_root() {
            if !(*(*node).parent).is_root() {
                (*(*(*node).parent).parent).push();
            }

            (*(*node).parent).push();
            (*node).push();

            if !(*(*node).parent).is_root() {
                if (*node).is_left() == (*(*node).parent).is_left() {
                    (*(*node).parent).rotate();
                } else {
                    (*node).rotate();
                }
            }

            (*node).rotate();
        }

        (*node).push();
    }

    unsafe fn access(&mut self, node: *mut Node) {
        // Untie lower node
        self.splay(node);
        (*node).right = std::ptr::null_mut();
        (*node).update();

        // Tie upper node
        while (*node).parent != std::ptr::null_mut() {
            let parent = (*node).parent;
            self.splay(parent);

            (*parent).right = node;
            (*parent).update();

            self.splay(node);
        }
    }

    unsafe fn link(&mut self, child: *mut Node, parent: *mut Node) {
        self.access(child);
        self.access(parent);

        (*child).left = parent;
        (*parent).parent = child;

        (*child).update();
    }

    unsafe fn cut(&mut self, node: *mut Node) {
        self.access(node);

        if (*node).left != std::ptr::null_mut() {
            (*(*node).left).parent = std::ptr::null_mut();
            (*node).left = std::ptr::null_mut();

            (*node).update();
        }
    }

    unsafe fn make_root(&mut self, node: *mut Node) {
        self.access(node);
        self.splay(node);

        (*node).flip ^= true;
    }

    unsafe fn get_max(&mut self, x: *mut Node, y: *mut Node) -> (i64, usize) {
        let lca = self.get_lca(x, y);
        let mut ret = ((*lca).value, (*lca).idx);

        self.access(x);
        self.splay(lca);

        if (*lca).right != std::ptr::null_mut() {
            ret = ret.max((*(*lca).right).max);
        }

        self.access(y);
        self.splay(lca);

        if (*lca).right != std::ptr::null_mut() {
            ret = ret.max((*(*lca).right).max);
        }

        ret
    }
}

unsafe fn process_dfs(
    tree: &mut LinkCutTree,
    graph: &Vec<Vec<(usize, i64)>>,
    edge_info: &mut Vec<(usize, usize)>,
    idx: &mut usize,
    n: usize,
    curr: i64,
    prev: i64,
) {
    for &(next, cost) in graph[curr as usize].iter() {
        if next as i64 == prev {
            continue;
        }

        process_dfs(tree, graph, edge_info, idx, n, next as i64, curr);

        *idx += 1;

        (*tree.nodes[n + *idx]).init(cost, *idx);
        tree.link(tree.nodes[next], tree.nodes[n + *idx]);
        tree.link(tree.nodes[n + *idx], tree.nodes[curr as usize]);

        edge_info[*idx] = (curr as usize, next);
    }
}

// Reference: https://justicehui.github.io/hard-algorithm/2021/01/01/link-cut-tree/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    // Number in Problem : 0 ~ n - 1 => Convert to 1 ~ n
    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

        unsafe {
            let mut tree = LinkCutTree::new(2 * n + m);
            let mut graph = vec![Vec::new(); n + 1];
            let mut edge_info = vec![(0, 0); n + m];
            let mut cost_total = 0;
            let mut idx = 0;

            for i in 2..=n {
                let (u, c) = (scan.token::<usize>() + 1, scan.token::<i64>());
                graph[i].push((u, c));
                graph[u].push((i, c));

                cost_total += c;
            }

            process_dfs(&mut tree, &graph, &mut edge_info, &mut idx, n, 1, -1);

            let mut ret = 0;

            for _ in 0..m {
                let (u, v, c) = (
                    scan.token::<usize>() + 1,
                    scan.token::<usize>() + 1,
                    scan.token::<i64>(),
                );

                let (max_val, max_idx) = tree.get_max(tree.nodes[u], tree.nodes[v]);

                if max_val <= c || max_idx == 0 {
                    ret = ret ^ cost_total;
                    continue;
                }

                cost_total += c - max_val;
                idx += 1;

                let (from, to) = edge_info[max_idx];

                if tree.get_parent(tree.nodes[n as usize + max_idx]) == tree.nodes[from as usize] {
                    tree.cut(tree.nodes[n as usize + max_idx]);
                } else {
                    tree.cut(tree.nodes[from as usize]);
                }

                if tree.get_parent(tree.nodes[n as usize + max_idx]) == tree.nodes[to as usize] {
                    tree.cut(tree.nodes[n as usize + max_idx]);
                } else {
                    tree.cut(tree.nodes[to as usize]);
                }

                (*tree.nodes[n + idx]).init(c, idx);

                tree.make_root(tree.nodes[u]);
                tree.link(tree.nodes[u], tree.nodes[n + idx]);
                tree.link(tree.nodes[n + idx], tree.nodes[v]);

                edge_info[idx] = (u, v);

                ret = ret ^ cost_total;
            }

            writeln!(out, "{ret}").unwrap();
        }
    }
}

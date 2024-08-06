use io::Write;
use std::{collections::BTreeSet, io, str};

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
    min: (i64, usize),
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
            min: (value, idx),
            flip: false,
        }))
    }

    fn init(&mut self, value: i64, idx: usize) {
        self.left = std::ptr::null_mut();
        self.right = std::ptr::null_mut();
        self.parent = std::ptr::null_mut();
        self.value = value;
        self.idx = idx;
        self.min = (value, idx);
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
        self.min = (self.value, self.idx);

        if self.left != std::ptr::null_mut() {
            self.min = self.min.min((*self.left).min);
        }

        if self.right != std::ptr::null_mut() {
            self.min = self.min.min((*self.right).min);
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
            nodes[i] = Node::new(i64::MAX, 0);
        }

        Self { nodes }
    }

    unsafe fn get_root(&mut self, mut node: *mut Node) -> *mut Node {
        // Make chain to root
        self.access(node);

        // Get top node
        while (*node).left != std::ptr::null_mut() {
            node = (*node).left;
            (*node).push();
        }

        // Amortized
        self.access(node);

        node
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

    unsafe fn get_min(&mut self, x: *mut Node, y: *mut Node) -> (i64, usize) {
        let lca = self.get_lca(x, y);
        let mut ret = ((*lca).value, (*lca).idx);

        self.access(x);
        self.splay(lca);

        if (*lca).right != std::ptr::null_mut() {
            ret = ret.min((*(*lca).right).min);
        }

        self.access(y);
        self.splay(lca);

        if (*lca).right != std::ptr::null_mut() {
            ret = ret.min((*(*lca).right).min);
        }

        ret
    }
}

// Reference: https://justicehui.github.io/hard-algorithm/2021/01/01/link-cut-tree/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());

    unsafe {
        let mut tree = LinkCutTree::new(n + q + 1);
        let mut edge_info = vec![(0, 0); q + 1];
        let mut cost_info = BTreeSet::new();
        let mut idx = 0;

        for _ in 0..q {
            let command = scan.token::<i64>();

            if command == 1 {
                let (i, j, d) = (
                    scan.token::<usize>(),
                    scan.token::<usize>(),
                    scan.token::<i64>(),
                );

                if tree.get_root(tree.nodes[i]) == tree.get_root(tree.nodes[j]) {
                    let (min_cost, min_idx) = tree.get_min(tree.nodes[i], tree.nodes[j]);

                    if min_cost >= d {
                        continue;
                    }

                    let (from, to) = edge_info[min_idx];

                    if tree.get_parent(tree.nodes[n as usize + min_idx])
                        == tree.nodes[from as usize]
                    {
                        tree.cut(tree.nodes[n as usize + min_idx]);
                    } else {
                        tree.cut(tree.nodes[from as usize]);
                    }

                    if tree.get_parent(tree.nodes[n as usize + min_idx]) == tree.nodes[to as usize]
                    {
                        tree.cut(tree.nodes[n as usize + min_idx]);
                    } else {
                        tree.cut(tree.nodes[to as usize]);
                    }

                    cost_info.remove(&(min_cost, min_idx));
                }

                idx += 1;

                (*tree.nodes[n + idx]).init(d, idx);

                tree.make_root(tree.nodes[i]);
                tree.link(tree.nodes[i], tree.nodes[n + idx]);
                tree.link(tree.nodes[n + idx], tree.nodes[j]);

                edge_info[idx] = (i, j);
                cost_info.insert((d, idx));
            } else if command == 2 {
                let x = scan.token::<i64>();

                while !cost_info.is_empty() {
                    if cost_info.iter().next().unwrap().0 >= x {
                        break;
                    }

                    let (min_cost, min_idx) = cost_info.iter().next().unwrap().clone();
                    let (from, to) = edge_info[min_idx];

                    cost_info.remove(&(min_cost, min_idx));

                    if tree.get_parent(tree.nodes[n as usize + min_idx])
                        == tree.nodes[from as usize]
                    {
                        tree.cut(tree.nodes[n as usize + min_idx]);
                    } else {
                        tree.cut(tree.nodes[from as usize]);
                    }

                    if tree.get_parent(tree.nodes[n as usize + min_idx]) == tree.nodes[to as usize]
                    {
                        tree.cut(tree.nodes[n as usize + min_idx]);
                    } else {
                        tree.cut(tree.nodes[to as usize]);
                    }
                }
            } else {
                let (i, j) = (scan.token::<usize>(), scan.token::<usize>());

                writeln!(
                    out,
                    "{}",
                    if tree.get_root(tree.nodes[i]) != tree.get_root(tree.nodes[j]) {
                        0
                    } else {
                        tree.get_min(tree.nodes[i], tree.nodes[j]).0
                    }
                )
                .unwrap();
            }
        }
    }
}

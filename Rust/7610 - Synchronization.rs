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

#[derive(Debug, Clone)]
struct Node {
    left: *mut Node,
    right: *mut Node,
    parent: *mut Node,
    value: i64,
}

impl Node {
    fn new(value: i64) -> Self {
        Self {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent: std::ptr::null_mut(),
            value,
        }
    }

    unsafe fn is_root(&mut self) -> bool {
        self.parent == std::ptr::null_mut()
            || ((*self.parent).left != self && (*self.parent).right != self)
    }

    unsafe fn is_left(&mut self) -> bool {
        (*self).parent != std::ptr::null_mut() && (*self.parent).left == self
    }

    unsafe fn rotate(&mut self) {
        if (*self).is_root() {
            return;
        }
        
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
    }
}

struct LinkCutTree {
    nodes: Vec<Node>,
}

impl LinkCutTree {
    unsafe fn new(n: usize) -> Self {
        let nodes = vec![Node::new(1); n + 1];
        Self { nodes }
    }

    unsafe fn get_root(&mut self, pos: usize) -> *mut Node {
        let node = &mut self.nodes[pos] as *mut Node;
        self.get_root_internal(node)
    }

    unsafe fn get_root_internal(&mut self, mut node: *mut Node) -> *mut Node {
        // Make chain to root
        self.access(node);

        // Get top node
        while (*node).left != std::ptr::null_mut() {
            node = (*node).left;
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

        while (*node).right != std::ptr::null_mut() {
            node = (*node).right;
        }

        // Amortized
        self.access(node);

        node
    }

    unsafe fn splay(&mut self, node: *mut Node) {
        while !(*node).is_root() {
            if !(*(*node).parent).is_root() {
                if (*node).is_left() == (*(*node).parent).is_left() {
                    (*(*node).parent).rotate();
                } else {
                    (*node).rotate();
                }
            }

            (*node).rotate();
        }
    }

    unsafe fn access(&mut self, node: *mut Node) {
        // Untie lower node
        self.splay(node);
        (*node).right = std::ptr::null_mut();

        // Tie upper node
        while (*node).parent != std::ptr::null_mut() {
            let parent = (*node).parent;
            self.splay(parent);

            (*parent).right = node;

            self.splay(node);
        }
    }

    unsafe fn link(&mut self, parent: *mut Node, child: *mut Node) {
        self.access(child);
        self.access(parent);

        (*child).left = parent;
        (*parent).parent = child;
    }

    unsafe fn cut(&mut self, node: *mut Node) {
        self.access(node);

        if (*node).left == std::ptr::null_mut() {
            return;
        }

        (*(*node).left).parent = std::ptr::null_mut();
        (*node).left = std::ptr::null_mut();
    }

    unsafe fn link_edge(
        &mut self,
        edges: &Vec<(usize, usize)>,
        information: &Vec<i64>,
        pos: usize,
    ) {
        let (x, y) = edges[pos];
        let node_x = &mut self.nodes[x] as *mut Node;
        let node_y = &mut self.nodes[y] as *mut Node;

        let root_x = self.get_root(x);
        let root_y = self.get_root(y);
        let val = (*root_x).value + (*root_y).value - information[pos];

        self.link(node_x, node_y);

        let root = self.get_root(x);
        (*root).value = val;
    }

    unsafe fn cut_edge(
        &mut self,
        edges: &Vec<(usize, usize)>,
        information: &mut Vec<i64>,
        pos: usize,
    ) {
        let (x, y) = edges[pos];
        let node_x = &mut self.nodes[x] as *mut Node;
        let node_y = &mut self.nodes[y] as *mut Node;

        let root = self.get_root(x);
        let val = (*root).value;

        if self.get_parent(node_x) == node_y {
            self.cut(node_x);
        } else {
            self.cut(node_y);
        }

        let root_x = self.get_root(x);
        let root_y = self.get_root(y);
        (*root_x).value = val;
        (*root_y).value = val;

        information[pos] = val;
    }
}

// Reference: https://justicehui.github.io/hard-algorithm/2021/01/01/link-cut-tree/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, q) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    unsafe {
        let mut tree = LinkCutTree::new(n);
        let mut edges = vec![(0, 0); n + 1];
        let mut is_available = vec![false; n + 1];
        let mut information = vec![0; n + 1];

        for i in 1..n {
            edges[i] = (scan.token::<usize>(), scan.token::<usize>());
        }

        for _ in 0..m {
            let d = scan.token::<usize>();

            if is_available[d] {
                tree.cut_edge(&edges, &mut information, d);
            } else {
                tree.link_edge(&edges, &information, d);
            }

            is_available[d] = !is_available[d];
        }

        for _ in 0..q {
            let c = scan.token::<usize>();
            let root = tree.get_root(c);

            writeln!(out, "{}", (*root).value).unwrap();
        }
    }
}

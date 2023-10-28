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
    count: i64,
    sum: i64,
    flip: bool,
}

impl Node {
    fn new(value: i64) -> *mut Self {
        Box::into_raw(Box::new(Self {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent: std::ptr::null_mut(),
            value,
            count: 1,
            sum: value,
            flip: false,
        }))
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
        self.count = 1;
        self.sum = self.value;

        if self.left != std::ptr::null_mut() {
            self.count += (*self.left).count;
            self.sum += (*self.left).sum;
        }

        if self.right != std::ptr::null_mut() {
            self.count += (*self.right).count;
            self.sum += (*self.right).sum;
        }
    }

    unsafe fn push(&mut self) {
        if self.flip {
            let temp = (*self).left;
            (*self).left = (*self).right;
            (*self).right = temp;
        }

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
    unsafe fn _new(n: usize) -> Self {
        let mut nodes = vec![std::ptr::null_mut(); n + 1];

        for i in 1..=n {
            nodes[i] = Node::new(i as i64);
        }

        Self { nodes }
    }

    unsafe fn new_with_values(n: usize, values: &Vec<i64>) -> Self {
        let mut nodes = vec![std::ptr::null_mut(); n + 1];

        for i in 1..=n {
            nodes[i] = Node::new(values[i]);
        }

        Self { nodes }
    }

    unsafe fn is_connect(&mut self, x: *mut Node, y: *mut Node) -> bool {
        self.get_root(x) == self.get_root(y)
    }

    unsafe fn get_root(&mut self, mut node: *mut Node) -> *mut Node {
        // Make chain to root
        self.access(node);

        // Get top node
        while (*node).left != std::ptr::null_mut() {
            node = (*node).left;
        }

        // Amortized
        self.splay(node);

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
        self.splay(node);

        node
    }

    unsafe fn _get_depth(&mut self, node: *mut Node) -> i64 {
        // Make chain to root
        self.access(node);

        // node is root
        if (*node).left == std::ptr::null_mut() {
            return 0;
        }

        (*(*node).left).count
    }

    unsafe fn _get_ancestor(&mut self, mut node: *mut Node, nth: i64) -> *mut Node {
        let mut nth = self._get_depth(node) - nth;

        loop {
            let count = (*(*node).left).count;

            if count == nth {
                self.access(node);
                return node;
            }

            if count < nth {
                nth -= count + 1;
                node = (*node).right;
            } else {
                node = (*node).left;
            }
        }
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
        }
        (*node).left = std::ptr::null_mut();

        (*node).update();
    }

    unsafe fn _update_value(&mut self, node: *mut Node, value: i64) {
        self.access(node);
        (*node).value = value;
        (*node).update();
    }

    unsafe fn query_vertex(&mut self, x: *mut Node, y: *mut Node) -> i64 {
        let lca = self.get_lca(x, y);
        let mut ret = (*lca).value;

        // x to before lca == left->right
        self.access(x);
        self.splay(lca);

        if (*lca).right != std::ptr::null_mut() {
            ret += (*(*lca).right).sum;
        }

        // y to before lca == left->right
        self.access(y);
        self.splay(lca);

        if (*lca).right != std::ptr::null_mut() {
            ret += (*(*lca).right).sum;
        }

        ret
    }

    unsafe fn make_root(&mut self, node: *mut Node) {
        self.access(node);
        self.splay(node);

        (*node).flip ^= true;
    }

    unsafe fn _update_path(&mut self, x: *mut Node, y: *mut Node, _value: i64) {
        // Original root
        let root = self.get_root(x);

        // Make x to root, tie with y
        self.make_root(x);
        self.access(y);

        // Update value
        self.splay(x);
        (*x).push();

        // Revert
        self.make_root(root);
    }
}

fn find(parent: &mut Vec<usize>, node: usize) -> usize {
    if parent[node] == node {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn process_union(
    parent: &mut Vec<usize>,
    hard_accumulated: &mut Vec<i64>,
    mut a: usize,
    mut b: usize,
) {
    a = find(parent, a);
    b = find(parent, b);

    if a == b {
        return;
    } else if a > b {
        parent[a] = b;
        hard_accumulated[b] += hard_accumulated[a];
    } else {
        parent[b] = a;
        hard_accumulated[a] += hard_accumulated[b];
    }
}

// Reference: https://justicehui.github.io/hard-algorithm/2021/01/01/link-cut-tree/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut hard = vec![0; n + 1];
    let mut hard_accumulated = vec![0; n + 1];

    for i in 1..=n {
        hard[i] = scan.token::<i64>();
        hard_accumulated[i] = hard[i];
    }

    // National highway
    let mut parent = vec![0; n + 1];

    for i in 1..=n {
        parent[i] = i;
    }

    let hard_total = hard.iter().sum::<i64>();

    for _ in 0..m {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        process_union(&mut parent, &mut hard_accumulated, a, b);
    }

    // Expressway
    unsafe {
        let mut tree = LinkCutTree::new_with_values(n, &hard);
        let q = scan.token::<i64>();

        for _ in 0..q {
            let (p, a, b) = (
                scan.token::<i64>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
            );

            if p == 1 {
                process_union(&mut parent, &mut hard_accumulated, a, b);
            } else if p == 2 {
                // If area a and b are not connected by national highway, print -1
                if find(&mut parent, a) != find(&mut parent, b) {
                    writeln!(out, "-1").unwrap();
                    continue;
                }

                // If area a and b have a cycle by expressway, print -1
                if tree.is_connect(tree.nodes[a], tree.nodes[b]) {
                    writeln!(out, "-1").unwrap();
                    continue;
                }

                tree.make_root(tree.nodes[a]);
                tree.link(tree.nodes[a], tree.nodes[b]);
            } else if p == 3 {
                if tree.get_parent(tree.nodes[a]) == tree.nodes[b] {
                    tree.cut(tree.nodes[a]);
                } else if tree.get_parent(tree.nodes[b]) == tree.nodes[a] {
                    tree.cut(tree.nodes[b]);
                } else {
                    writeln!(out, "-1").unwrap();
                }
            } else if p == 4 {
                writeln!(out, "{}", hard_total - hard_accumulated[1]).unwrap();
            } else if p == 5 {
                // Check expressway and national highway are connected
                let mut ret = 0;

                ret += if find(&mut parent, a) == 1 {
                    if tree.is_connect(tree.nodes[a], tree.nodes[1]) {
                        hard[a] * 2
                    } else {
                        hard[a]
                    }
                } else {
                    0
                };

                ret += if find(&mut parent, b) == 1 {
                    if tree.is_connect(tree.nodes[b], tree.nodes[1]) {
                        hard[b] * 2
                    } else {
                        hard[b]
                    }
                } else {
                    0
                };

                writeln!(out, "{ret}").unwrap();
            } else {
                // If area a and b are not connected by expressway, print -1
                if !tree.is_connect(tree.nodes[a], tree.nodes[b]) {
                    writeln!(out, "-1").unwrap();
                    continue;
                }

                // If area a and root are not connected by national highway, print 0
                if find(&mut parent, a) != 1 {
                    writeln!(out, "0").unwrap();
                    continue;
                }

                let mut ret = tree.query_vertex(tree.nodes[a], tree.nodes[b]);

                if tree.is_connect(tree.nodes[a], tree.nodes[1]) {
                    ret *= 2;
                }

                writeln!(out, "{ret}").unwrap();
            }
        }
    }
}

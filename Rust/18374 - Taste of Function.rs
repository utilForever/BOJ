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
    val_f: usize,
    lazy: i64,
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
            lazy: 0,
            val_f: 0,
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

    unsafe fn update_with_value(&mut self, value: i64) {
        self.value = value;
        self.update();
    }

    unsafe fn push(&mut self) {
        self.update_with_value(self.value + self.lazy);

        if self.flip {
            let temp = (*self).left;
            (*self).left = (*self).right;
            (*self).right = temp;
        }

        if (*self).left != std::ptr::null_mut() {
            (*(*self).left).lazy += self.lazy;
            (*(*self).left).flip ^= self.flip;
        }

        if (*self).right != std::ptr::null_mut() {
            (*(*self).right).lazy += self.lazy;
            (*(*self).right).flip ^= self.flip;
        }

        self.lazy = 0;
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
            nodes[i] = Node::new(0);
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
        self.access(node);

        node
    }

    unsafe fn _get_parent(&mut self, mut node: *mut Node) -> *mut Node {
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
        self.splay_to_root(node);

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
        self.splay_to_root(x);

        if (*x).parent == std::ptr::null_mut() {
            x
        } else {
            (*x).parent
        }
    }

    unsafe fn splay_to_root(&mut self, node: *mut Node) {
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
        self.splay_to_root(node);
        (*node).right = std::ptr::null_mut();
        (*node).update();

        // Tie upper node
        while (*node).parent != std::ptr::null_mut() {
            let parent = (*node).parent;
            self.splay_to_root(parent);

            (*parent).right = node;
            (*parent).update();

            self.splay_to_root(node);
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

    unsafe fn _query_vertex(&mut self, x: *mut Node, y: *mut Node) -> i64 {
        let lca = self.get_lca(x, y);
        let mut ret = (*lca).value;

        // x to before lca == left->right
        self.access(x);
        self.splay_to_root(lca);

        if (*lca).right != std::ptr::null_mut() {
            ret += (*(*lca).right).sum;
        }

        // y to before lca == left->right
        self.access(y);
        self.splay_to_root(lca);

        if (*lca).right != std::ptr::null_mut() {
            ret += (*(*lca).right).sum;
        }

        ret
    }

    unsafe fn _make_root(&mut self, node: *mut Node) {
        self.access(node);
        self.splay_to_root(node);

        (*node).flip ^= true;
    }

    unsafe fn _update_path(&mut self, x: *mut Node, y: *mut Node, value: i64) {
        // Original root
        let root = self.get_root(x);

        // Make x to root, tie with y
        self._make_root(x);
        self.access(y);

        // Update value
        self.splay_to_root(x);
        (*x).lazy += value;
        (*x).push();

        // Revert
        self._make_root(root);
    }

    unsafe fn process_query1(&mut self, i: usize, j: usize) {
        let root = self.get_root(self.nodes[i]);
        self.cut(self.nodes[i]);

        if self.get_root(self.nodes[j]) != self.nodes[i] {
            self.link(self.nodes[i], self.nodes[j]);
        }

        (*self.nodes[i]).val_f = j;

        if root == self.nodes[i] {
            return;
        }

        if self.get_root(self.nodes[(*root).val_f]) != root {
            self.link(root, self.nodes[(*root).val_f]);
        }
    }

    unsafe fn process_query2(&mut self, i: usize, x: i64) {
        self.access(self.nodes[i]);
        (*self.nodes[i]).update_with_value(x);
    }

    unsafe fn process_query3(&mut self, x: usize) -> i64 {
        let root = self.get_root(self.nodes[x]);

        if self.get_root(self.nodes[(*root).val_f]) != root {
            self.access(self.nodes[x]);
            return (*self.nodes[x]).sum;
        }

        let mut ret = 0;

        self.access(self.nodes[x]);

        ret += (*self.nodes[x]).sum;

        self.access(self.nodes[(*root).val_f]);

        ret += (*self.nodes[(*root).val_f]).sum;

        let lca = self.get_lca(self.nodes[x], self.nodes[(*root).val_f]);

        self.access(lca);

        ret -= (*lca).sum;

        ret
    }
}

// Reference: https://justicehui.github.io/hard-algorithm/2021/01/01/link-cut-tree/
// Reference: https://blog.naver.com/melon940925/222150174912
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());

    unsafe {
        let mut tree = LinkCutTree::new(n);

        for i in 1..=n {
            let val = scan.token::<usize>();
            (*tree.nodes[i]).val_f = val;
        }

        for i in 1..=n {
            let taste = scan.token::<i64>();
            (*tree.nodes[i]).update_with_value(taste);
        }

        for i in 1..=n {
            let val_f = (*tree.nodes[i]).val_f;

            if tree.is_connect(tree.nodes[i], tree.nodes[val_f]) {
                continue;
            }

            tree.link(tree.nodes[i], tree.nodes[val_f]);
        }

        for _ in 0..q {
            let command = scan.token::<i64>();

            if command == 1 {
                let (i, j) = (scan.token::<usize>(), scan.token::<usize>());
                tree.process_query1(i, j);
            } else if command == 2 {
                let (i, x) = (scan.token::<usize>(), scan.token::<i64>());
                tree.process_query2(i, x);
            } else {
                let x = scan.token::<usize>();
                writeln!(out, "{}", tree.process_query3(x)).unwrap();
            }
        }
    }
}

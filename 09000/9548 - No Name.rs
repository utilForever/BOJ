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

struct Node {
    left: *mut Node,
    right: *mut Node,
    parent: *mut Node,
    value: i64,
    count: i64,
}

impl Node {
    fn new(value: i64) -> *mut Self {
        Box::into_raw(Box::new(Self {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent: std::ptr::null_mut(),
            value,
            count: 1,
        }))
    }

    fn new_with_parent(value: i64, parent: *mut Self) -> *mut Self {
        Box::into_raw(Box::new(Self {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent,
            value,
            count: 1,
        }))
    }

    unsafe fn update(&mut self) {
        self.count = 1;

        if self.left != std::ptr::null_mut() {
            self.count += (*self.left).count;
        }

        if self.right != std::ptr::null_mut() {
            self.count += (*self.right).count;
        }
    }

    unsafe fn push(&mut self) {
        // Do nothing
    }
}

struct SplayTree {
    root: *mut Node,
    ptr: Vec<*mut Node>,
}

impl SplayTree {
    unsafe fn _init(&mut self, n: usize) {
        if self.root != std::ptr::null_mut() {
            drop(Box::from_raw(self.root));
        }

        self.root = Node::new(1_000_000_007);
        self.ptr = vec![std::ptr::null_mut(); n + 1];

        let mut node = self.root;

        for i in 1..=n {
            (*node).right = Node::new_with_parent(i as i64, node);
            self.ptr[i] = (*node).right;
            node = (*node).right;
        }

        (*node).right = Node::new_with_parent(1_000_000_007, node);

        for i in (1..=n).rev() {
            (*self.ptr[i]).update();
        }

        self.splay(self.ptr[n / 2], std::ptr::null_mut());
    }

    unsafe fn init_with_string(&mut self, value: String) {
        if self.root != std::ptr::null_mut() {
            drop(Box::from_raw(self.root));
        }

        let n = value.len();

        self.root = Node::new(1_000_000_007);
        self.ptr = vec![std::ptr::null_mut(); n + 1];

        let values = value.chars().collect::<Vec<_>>();
        let mut node = self.root;

        for i in 1..=n {
            (*node).right = Node::new_with_parent(values[i - 1] as i64, node);
            self.ptr[i] = (*node).right;
            node = (*node).right;
        }

        (*node).right = Node::new_with_parent(1_000_000_007, node);

        for i in (1..=n).rev() {
            (*self.ptr[i]).update();
        }

        self.splay(self.ptr[(n / 2).max(1)], std::ptr::null_mut());
    }

    unsafe fn _get_all(&mut self, vals: &mut Vec<i64>, node: *mut Node) {
        (*node).push();

        if (*node).left != std::ptr::null_mut() {
            self._get_all(vals, (*node).left);
        }

        if (*node).value.abs() != 1_000_000_007 {
            vals.push((*node).value);
        }

        if (*node).right != std::ptr::null_mut() {
            self._get_all(vals, (*node).right);
        }
    }

    unsafe fn gather(&mut self, left: i64, right: i64) -> *mut Node {
        self.kth(right + 1);

        let node = self.root;

        self.kth(left - 1);
        self.splay(node, self.root);

        (*(*self.root).right).left
    }

    unsafe fn rotate(&mut self, node: *mut Node) {
        let parent = (*node).parent;
        let child;

        if parent == std::ptr::null_mut() {
            return;
        }

        if node == (*parent).left {
            child = (*node).right;
            (*parent).left = child;
            (*node).right = parent;
        } else {
            child = (*node).left;
            (*parent).right = child;
            (*node).left = parent;
        }

        (*node).parent = (*parent).parent;
        (*parent).parent = node;

        if child != std::ptr::null_mut() {
            (*child).parent = parent;
        }

        if (*node).parent != std::ptr::null_mut() {
            if (*(*node).parent).left == parent {
                (*(*node).parent).left = node;
            } else {
                (*(*node).parent).right = node;
            }
        } else {
            self.root = node;
        }

        (*parent).update();
        (*node).update();
    }

    unsafe fn splay(&mut self, node: *mut Node, grandparent: *mut Node) {
        while (*node).parent != grandparent {
            let parent = (*node).parent;

            if (*parent).parent != grandparent {
                (*(*parent).parent).push();
            }

            (*parent).push();
            (*node).push();

            if (*parent).parent == grandparent {
                self.rotate(node);
                continue;
            }

            let parent_of_parent = (*parent).parent;

            if ((*parent).left == node) == ((*parent_of_parent).left == parent) {
                self.rotate(parent);
                self.rotate(node);
            } else {
                self.rotate(node);
                self.rotate(node);
            }
        }

        (*node).push();

        if grandparent == std::ptr::null_mut() {
            self.root = node;
        }
    }

    unsafe fn _insert(&mut self, value: i64) {
        let mut node = self.root;
        let parent: *mut *mut Node;

        if node == std::ptr::null_mut() {
            let node = Node::new(value);
            self.root = node;

            return;
        }

        loop {
            if value == (*node).value {
                return;
            }

            if value < (*node).value {
                if (*node).left == std::ptr::null_mut() {
                    parent = &mut (*node).left;
                    break;
                }

                node = (*node).left;
            } else {
                if (*node).right == std::ptr::null_mut() {
                    parent = &mut (*node).right;
                    break;
                }

                node = (*node).right;
            }
        }

        let node_new = Node::new(value);
        (*node_new).parent = node;
        *parent = node;

        self.splay(node_new, std::ptr::null_mut());
    }

    unsafe fn insert_kth(&mut self, k: i64, value: String) {
        self.kth(k);

        let mut node = (*self.root).left;
        (*node).push();

        while (*node).right != std::ptr::null_mut() {
            node = (*node).right;
            (*node).push();
        }

        let values = value.chars().collect::<Vec<_>>();
        let temp = node;

        for value in values {
            (*node).right = Node::new_with_parent(value as i64, node);
            node = (*node).right;
        }

        while node != (*temp).parent {
            (*node).update();
            node = (*node).parent;
        }

        self.splay(node, std::ptr::null_mut());
    }

    unsafe fn _find(&mut self, value: i64) -> bool {
        let mut node = self.root;

        if node == std::ptr::null_mut() {
            return false;
        }

        while node != std::ptr::null_mut() {
            if value == (*node).value {
                break;
            }

            if value < (*node).value {
                if (*node).left == std::ptr::null_mut() {
                    break;
                }

                node = (*node).left;
            } else {
                if (*node).right == std::ptr::null_mut() {
                    break;
                }

                node = (*node).right;
            }
        }

        self.splay(node, std::ptr::null_mut());

        (*node).value == value
    }

    unsafe fn _delete(&mut self, value: i64) {
        if !self._find(value) {
            return;
        }

        let node = self.root;

        if (*node).left != std::ptr::null_mut() && (*node).right != std::ptr::null_mut() {
            self.root = (*node).left;
            (*self.root).parent = std::ptr::null_mut();

            let mut node_new = self.root;

            while (*node_new).right != std::ptr::null_mut() {
                node_new = (*node_new).right;
            }

            (*node_new).right = (*node).right;
            (*(*node).right).parent = node_new;

            drop(Box::from_raw(node));
            return;
        }

        if (*node).left != std::ptr::null_mut() {
            self.root = (*node).left;
            (*self.root).parent = std::ptr::null_mut();

            drop(Box::from_raw(node));
            return;
        }

        if (*node).right != std::ptr::null_mut() {
            self.root = (*node).right;
            (*self.root).parent = std::ptr::null_mut();

            drop(Box::from_raw(node));
            return;
        }

        self.root = std::ptr::null_mut();
        drop(Box::from_raw(node));
    }

    unsafe fn kth(&mut self, mut k: i64) {
        let mut node = self.root;
        (*node).push();

        loop {
            while (*node).left != std::ptr::null_mut() && (*(*node).left).count > k {
                node = (*node).left;
                (*node).push();
            }

            if (*node).left != std::ptr::null_mut() {
                k -= (*(*node).left).count;
            }

            if k == 0 {
                break;
            }

            k -= 1;
            node = (*node).right;
            (*node).push();
        }

        self.splay(node, std::ptr::null_mut());
    }
}

unsafe fn process_inorder(node: *mut Node, ret: &mut Vec<i64>) {
    let mut stack: Vec<*mut Node> = Vec::new();
    let mut curr = node;

    while !stack.is_empty() || curr != std::ptr::null_mut() {
        while curr != std::ptr::null_mut() {
            stack.push(curr);
            curr = (*curr).left;
        }

        if !stack.is_empty() {
            curr = stack.pop().unwrap();
        }

        if curr != std::ptr::null_mut() && (*curr).value.abs() != 1_000_000_007 {
            ret.push((*curr).value);
        }

        curr = (*curr).right;
    }
}

// Reference: https://justicehui.github.io/hard-algorithm/2018/11/12/SplayTree1/
// Reference: https://justicehui.github.io/hard-algorithm/2018/11/13/SplayTree2/
// Reference: https://justicehui.github.io/hard-algorithm/2019/10/22/SplayTree3/
// Reference: https://justicehui.github.io/hard-algorithm/2019/10/23/SplayTree4/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut tree = SplayTree {
            root: Node::new(0),
            ptr: Vec::new(),
        };

        unsafe {
            let s: String = scan.token::<String>();
            tree.init_with_string(s);

            loop {
                let command = scan.token::<String>();

                if command == "END" {
                    break;
                }

                if command == "I" {
                    let (r, x) = (scan.token::<String>(), scan.token::<i64>());
                    tree.insert_kth(x + 1, r);
                } else {
                    let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
                    let values = tree.gather(x + 1, y + 1);
                    let mut ret = Vec::new();

                    process_inorder(values, &mut ret);

                    let ret = ret.iter().map(|&x| x as u8 as char).collect::<String>();
                    writeln!(out, "{ret}").unwrap();
                }
            }
        }
    }
}

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
    min: i64,
    max: i64,
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
            min: value,
            max: value,
            sum: value,
            flip: false,
        }))
    }

    fn new_with_parent(value: i64, parent: *mut Self) -> *mut Self {
        Box::into_raw(Box::new(Self {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent,
            value,
            min: value,
            max: value,
            sum: value,
            count: 1,
            flip: false,
        }))
    }

    unsafe fn update(&mut self) {
        self.count = 1;
        self.min = self.value;
        self.max = self.value;
        self.sum = self.value;

        if self.left != std::ptr::null_mut() {
            self.count += (*self.left).count;
            self.min = self.min.min((*self.left).min);
            self.max = self.max.max((*self.left).max);
            self.sum += (*self.left).sum;
        }

        if self.right != std::ptr::null_mut() {
            self.count += (*self.right).count;
            self.min = self.min.min((*self.right).min);
            self.max = self.max.max((*self.right).max);
            self.sum += (*self.right).sum;
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
            (*(*self).left).flip ^= true;
        }

        if (*self).right != std::ptr::null_mut() {
            (*(*self).right).flip ^= true;
        }

        self.flip = false;
    }
}

struct SplayTree {
    root: *mut Node,
    ptr: Vec<*mut Node>,
}

impl SplayTree {
    unsafe fn init(&mut self, n: usize) {
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

    unsafe fn get_all(&mut self, vals: &mut Vec<i64>, node: *mut Node) {
        (*node).push();

        if (*node).left != std::ptr::null_mut() {
            self.get_all(vals, (*node).left);
        }

        if (*node).value.abs() != 1_000_000_007 {
            vals.push((*node).value);
        }

        if (*node).right != std::ptr::null_mut() {
            self.get_all(vals, (*node).right);
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

    unsafe fn flip(&mut self, left: i64, right: i64) {
        let node = self.gather(left, right);
        (*node).flip ^= true;
    }

    unsafe fn shift(&mut self, left: i64, right: i64, mut index: i64) {
        self.gather(left, right);
        index %= right - left + 1;

        if index < 0 {
            index += right - left + 1;
        }

        if index > 0 {
            self.flip(left, right);
            self.flip(left, left + index - 1);
            self.flip(left + index, right);
        }
    }

    unsafe fn get_index(&mut self, index: usize) {
        self.splay(self.ptr[index], std::ptr::null_mut());
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

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut tree = SplayTree {
        root: Node::new(0),
        ptr: Vec::new(),
    };

    unsafe {
        tree.init(n);

        for _ in 0..q {
            let command = scan.token::<i64>();

            if command == 1 {
                let (l, r) = (scan.token::<i64>(), scan.token::<i64>());
                tree.flip(l, r);

                let val = tree.gather(l, r);
                writeln!(out, "{} {} {}", (*val).min, (*val).max, (*val).sum).unwrap();
            } else if command == 2 {
                let (l, r, x) = (
                    scan.token::<i64>(),
                    scan.token::<i64>(),
                    scan.token::<i64>(),
                );

                let val = tree.gather(l, r);
                writeln!(out, "{} {} {}", (*val).min, (*val).max, (*val).sum).unwrap();

                tree.shift(l, r, x);
            } else if command == 3 {
                let i = scan.token::<i64>();
                tree.kth(i);

                writeln!(out, "{}", (*tree.root).value).unwrap();
            } else {
                let x = scan.token::<usize>();
                tree.get_index(x);

                writeln!(out, "{}", (*(*tree.root).left).count).unwrap();
            }
        }

        let mut ret = Vec::new();
        tree.get_all(&mut ret, tree.root);

        for val in ret {
            write!(out, "{} ", val).unwrap();
        }

        writeln!(out).unwrap();
    }
}

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

struct LazyNode {
    value: i64,
    seq_init: i64,
    seq_common_diff: i64,
}

impl LazyNode {
    fn new(value: i64, seq_init: i64, seq_common_diff: i64) -> Self {
        Self {
            value,
            seq_init,
            seq_common_diff,
        }
    }
}

struct Node {
    left: *mut Node,
    right: *mut Node,
    parent: *mut Node,
    value: i64,
    count: i64,
    sum: i64,
    lazy: LazyNode,
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
            lazy: LazyNode::new(0, 0, 0),
            flip: false,
        }))
    }

    fn new_with_parent(value: i64, parent: *mut Self) -> *mut Self {
        Box::into_raw(Box::new(Self {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent,
            value,
            count: 1,
            sum: value,
            lazy: LazyNode::new(0, 0, 0),
            flip: false,
        }))
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
        // Lazy propagation
        let lazy = &mut (*self).lazy;

        if lazy.value != 0 || lazy.seq_init != 0 || lazy.seq_common_diff != 0 {
            let size_node = (*self).count;
            let size_left = if (*self).left != std::ptr::null_mut() {
                (*(*self).left).count
            } else {
                0
            };
            let size_right = size_node - size_left - 1;

            if lazy.value > 0 {
                self.value = lazy.value + lazy.seq_init + lazy.seq_common_diff * size_left;

                if self.left != std::ptr::null_mut() {
                    let left = &mut *(*self).left;

                    left.lazy.value = lazy.value;
                    left.lazy.seq_init = lazy.seq_init;
                    left.lazy.seq_common_diff = lazy.seq_common_diff;
                    left.sum = lazy.value * size_left
                        + lazy.seq_init * size_left
                        + lazy.seq_common_diff * size_left * (size_left - 1) / 2;
                }

                if self.right != std::ptr::null_mut() {
                    let right = &mut *(*self).right;

                    right.lazy.value = lazy.value;
                    right.lazy.seq_init = lazy.seq_init + (size_left + 1) * lazy.seq_common_diff;
                    right.lazy.seq_common_diff = lazy.seq_common_diff;
                    right.sum = lazy.value * size_right
                        + (lazy.seq_init + (size_left + 1) * lazy.seq_common_diff) * size_right
                        + lazy.seq_common_diff * size_right * (size_right - 1) / 2;
                }
            } else {
                self.value += lazy.seq_init + lazy.seq_common_diff * size_left;

                if self.left != std::ptr::null_mut() {
                    let left = &mut *(*self).left;

                    left.lazy.seq_init += lazy.seq_init;
                    left.lazy.seq_common_diff += lazy.seq_common_diff;
                    left.sum += lazy.seq_init * size_left
                        + lazy.seq_common_diff * size_left * (size_left - 1) / 2;
                }

                if self.right != std::ptr::null_mut() {
                    let right = &mut *(*self).right;

                    right.lazy.seq_init += lazy.seq_init + (size_left + 1) * lazy.seq_common_diff;
                    right.lazy.seq_common_diff += lazy.seq_common_diff;
                    right.sum += (lazy.seq_init + (size_left + 1) * lazy.seq_common_diff)
                        * size_right
                        + lazy.seq_common_diff * size_right * (size_right - 1) / 2;
                }
            }

            lazy.value = 0;
            lazy.seq_init = 0;
            lazy.seq_common_diff = 0;
        }
        // Flip
        if self.flip {
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

    unsafe fn init_with_nums(&mut self, n: usize, nums: Vec<i64>) {
        if self.root != std::ptr::null_mut() {
            drop(Box::from_raw(self.root));
        }

        self.root = Node::new(1_000_000_007);
        self.ptr = vec![std::ptr::null_mut(); n + 1];

        let mut node = self.root;

        for i in 1..=n {
            (*node).right = Node::new_with_parent(nums[i - 1], node);
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

    unsafe fn insert_kth(&mut self, k: i64, value: i64) {
        self.kth(k);

        let mut node = (*self.root).left;
        (*node).push();

        while (*node).right != std::ptr::null_mut() {
            node = (*node).right;
            (*node).push();
        }

        (*node).right = Node::new_with_parent(value, node);
        (*(*node).right).update();

        self.splay((*node).right, std::ptr::null_mut());
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

    unsafe fn _flip(&mut self, left: i64, right: i64) {
        let node = self.gather(left, right);
        (*node).flip ^= true;
    }

    unsafe fn _shift(&mut self, left: i64, right: i64, mut index: i64) {
        self.gather(left, right);
        index %= right - left + 1;

        if index < 0 {
            index += right - left + 1;
        }

        if index > 0 {
            self._flip(left, right);
            self._flip(left, left + index - 1);
            self._flip(left + index, right);
        }
    }

    unsafe fn value_update(&mut self, left: i64, right: i64, value: i64) {
        let node = self.gather(left, right);
        let count = (*node).count;

        (*node).sum = value * count;
        (*node).lazy = LazyNode::new(value, 0, 0);
    }

    unsafe fn value_add(&mut self, left: i64, right: i64, value: i64) {
        let node = self.gather(left, right);
        let count = (*node).count;
        let lazy = &(*node).lazy;

        (*node).sum += value * count * (count + 1) / 2;
        (*node).lazy = LazyNode::new(
            lazy.value,
            lazy.seq_init + value,
            lazy.seq_common_diff + value,
        );
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
        let mut nums = vec![0; n];

        for i in 0..n {
            nums[i] = scan.token::<i64>();
        }

        tree.init_with_nums(n, nums);

        for _ in 0..q {
            let command = scan.token::<i64>();

            if command == 1 {
                let (a, b, x) = (
                    scan.token::<i64>(),
                    scan.token::<i64>(),
                    scan.token::<i64>(),
                );
                tree.value_update(a, b, x);
            } else if command == 2 {
                let (a, b, x) = (
                    scan.token::<i64>(),
                    scan.token::<i64>(),
                    scan.token::<i64>(),
                );
                tree.value_add(a, b, x);
            } else if command == 3 {
                let (c, x) = (scan.token::<i64>(), scan.token::<i64>());
                tree.insert_kth(c, x);
            } else {
                let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
                let val = tree.gather(a, b);

                writeln!(out, "{}", (*val).sum).unwrap();
            }
        }
    }
}

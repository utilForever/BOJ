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
    value: bool,
    count: i64,
    cnt_left: i64,
    cnt_right: i64,
    cnt_left_rev: i64,
    cnt_right_rev: i64,
    flip: bool,
    switch: bool,
}

impl Node {
    fn new(value: bool) -> *mut Self {
        Box::into_raw(Box::new(Self {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent: std::ptr::null_mut(),
            value,
            count: 1,
            cnt_left: if value { 1 } else { 0 },
            cnt_right: if value { 0 } else { 1 },
            cnt_left_rev: if value { 1 } else { 0 },
            cnt_right_rev: if value { 0 } else { 1 },
            flip: false,
            switch: false,
        }))
    }

    fn new_with_parent(value: bool, parent: *mut Self) -> *mut Self {
        Box::into_raw(Box::new(Self {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent,
            value,
            count: 1,
            cnt_left: if value { 1 } else { 0 },
            cnt_right: if value { 0 } else { 1 },
            cnt_left_rev: if value { 1 } else { 0 },
            cnt_right_rev: if value { 0 } else { 1 },
            flip: false,
            switch: false,
        }))
    }

    unsafe fn update(&mut self) {
        self.count = 1;

        if self.value {
            self.cnt_left = 1;
            self.cnt_right = 0;
            self.cnt_left_rev = 1;
            self.cnt_right_rev = 0;
        } else {
            self.cnt_left = 0;
            self.cnt_right = 1;
            self.cnt_left_rev = 0;
            self.cnt_right_rev = 1;
        }

        if self.left != std::ptr::null_mut() {
            self.count += (*self.left).count;
        }

        if self.right != std::ptr::null_mut() {
            self.count += (*self.right).count;
        }

        if self.left != std::ptr::null_mut() && self.right != std::ptr::null_mut() {
            (*self.left).push();
            (*self.right).push();

            let cnt_min = self.cnt_right.min((*self.left).cnt_left);
            let cnt_min_inv = self.cnt_left_rev.min((*self.left).cnt_right_rev);

            self.cnt_left += (*self.left).cnt_left - cnt_min;
            self.cnt_right += (*self.left).cnt_right - cnt_min;
            self.cnt_left_rev += (*self.left).cnt_left_rev - cnt_min_inv;
            self.cnt_right_rev += (*self.left).cnt_right_rev - cnt_min_inv;

            let cnt_min = self.cnt_left.min((*self.right).cnt_right);
            let cnt_min_inv = self.cnt_right_rev.min((*self.right).cnt_left_rev);

            self.cnt_left += (*self.right).cnt_left - cnt_min;
            self.cnt_right += (*self.right).cnt_right - cnt_min;
            self.cnt_left_rev += (*self.right).cnt_left_rev - cnt_min_inv;
            self.cnt_right_rev += (*self.right).cnt_right_rev - cnt_min_inv;
        } else if self.left != std::ptr::null_mut() {
            (*self.left).push();

            let cnt_min = self.cnt_right.min((*self.left).cnt_left);
            let cnt_min_inv = self.cnt_left_rev.min((*self.left).cnt_right_rev);

            self.cnt_left += (*self.left).cnt_left - cnt_min;
            self.cnt_right += (*self.left).cnt_right - cnt_min;
            self.cnt_left_rev += (*self.left).cnt_left_rev - cnt_min_inv;
            self.cnt_right_rev += (*self.left).cnt_right_rev - cnt_min_inv;
        } else if self.right != std::ptr::null_mut() {
            (*self.right).push();

            let cnt_min = self.cnt_left.min((*self.right).cnt_right);
            let cnt_min_inv = self.cnt_right_rev.min((*self.right).cnt_left_rev);

            self.cnt_left += (*self.right).cnt_left - cnt_min;
            self.cnt_right += (*self.right).cnt_right - cnt_min;
            self.cnt_left_rev += (*self.right).cnt_left_rev - cnt_min_inv;
            self.cnt_right_rev += (*self.right).cnt_right_rev - cnt_min_inv;
        }
    }

    unsafe fn push(&mut self) {
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

            std::mem::swap(&mut self.cnt_left, &mut self.cnt_left_rev);
            std::mem::swap(&mut self.cnt_right, &mut self.cnt_right_rev);

            self.flip = false;
        }

        if self.switch {
            self.value ^= true;

            if (*self).left != std::ptr::null_mut() {
                (*(*self).left).switch ^= true;
            }

            if (*self).right != std::ptr::null_mut() {
                (*(*self).right).switch ^= true;
            }

            std::mem::swap(&mut self.cnt_left, &mut self.cnt_right_rev);
            std::mem::swap(&mut self.cnt_right, &mut self.cnt_left_rev);

            self.switch = false;
        }
    }
}

struct SplayTree {
    root: *mut Node,
    ptr: Vec<*mut Node>,
}

impl SplayTree {
    unsafe fn init(&mut self, n: usize, s: &Vec<char>) {
        if self.root != std::ptr::null_mut() {
            drop(Box::from_raw(self.root));
        }

        self.root = Node::new(true);
        self.ptr = vec![std::ptr::null_mut(); n + 1];

        let mut node = self.root;

        for i in 1..=n {
            (*node).right = Node::new_with_parent(if s[i - 1] == '(' { true } else { false }, node);
            self.ptr[i] = (*node).right;
            node = (*node).right;
        }

        (*node).right = Node::new_with_parent(false, node);

        for i in (1..=n).rev() {
            (*self.ptr[i]).update();
        }

        if n == 1 {
            return;
        }

        self.splay(self.ptr[n / 2], std::ptr::null_mut());
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

    unsafe fn switch(&mut self, left: i64, right: i64) {
        let node = self.gather(left, right);
        (*node).switch ^= true;
    }
}

// Reference: SUAPC 2023 Summer Editorial
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let s = scan.token::<String>().chars().collect::<Vec<_>>();

    let mut tree = SplayTree {
        root: Node::new(true),
        ptr: Vec::new(),
    };

    unsafe {
        tree.init(n, &s);

        for _ in 0..q {
            let (command, l, r) = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );

            if command == 1 {
                tree.switch(l, r);
            } else if command == 2 {
                tree.flip(l, r);
            } else if command == 3 {
                tree.flip(l, r);
                tree.switch(l, r);
            } else {
                tree.gather(l, r);
                let curr = (*(*tree.root).right).left;

                writeln!(
                    out,
                    "{}",
                    ((*curr).count - (*curr).cnt_left - (*curr).cnt_right) / 2
                )
                .unwrap();
            }
        }
    }
}

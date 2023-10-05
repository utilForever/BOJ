use io::Write;
use std::{io, str, cmp::Ordering};
use Ordering::Less;

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

pub trait Ext {
    type Item;

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    fn lower_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;
}

impl<T> Ext for [T] {
    type Item = T;
    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.lower_bound_by(|y| y.cmp(x))
    }
    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0;
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Less { mid } else { base };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp == Less) as usize
    }
}

struct Node {
    left: *mut Node,
    right: *mut Node,
    parent: *mut Node,
    value: i64,
    count: i64,
    lazy: i64,
    idx: i64
}

impl Node {
    fn new_with_parent(value: i64, idx: i64, parent: *mut Self) -> *mut Self {
        Box::into_raw(Box::new(Self {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent,
            value,
            count: 1,
            lazy: 0,
            idx,
        }))
    }

    unsafe fn update(&mut self) {
        self.count = 1;

        if self.left != std::ptr::null_mut() {
            (*self.left).push();
            self.count += (*self.left).count;
        }

        if self.right != std::ptr::null_mut() {
            (*self.right).push();
            self.count += (*self.right).count;
        }
    }

    unsafe fn push(&mut self) {
        if self.lazy != 0 {
            self.value += self.lazy;
            self.idx += self.lazy;

            if self.left != std::ptr::null_mut() {
                (*self.left).lazy += self.lazy;
            }

            if self.right != std::ptr::null_mut() {
                (*self.right).lazy += self.lazy;
            }

            self.lazy = 0;
        }
    }
}

struct SplayTree {
    root: *mut Node,
}

impl SplayTree {   
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

    unsafe fn find(&mut self, value: i64) -> i64 {
        let mut node = self.root;
        let mut ret = 0;

        while node != std::ptr::null_mut() {
            (*node).push();

            if (*node).value >= value {
                node = (*node).left;
            } else {
                ret = ret.max((*node).idx);
                node = (*node).right;
            }
        }

        node = self.root;

        while node != std::ptr::null_mut() {
            if (*node).value >= value {
                node = (*node).left;
            } else {
                if (*node).idx == ret {
                    self.splay(node, std::ptr::null_mut());
                    break;
                }

                node = (*node).right;
            }
        }

        ret
    }

    unsafe fn delete(&mut self) {
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

// Reference: https://justicehui.github.io/hard-algorithm/2018/11/12/SplayTree1/
// Reference: https://justicehui.github.io/hard-algorithm/2018/11/13/SplayTree2/
// Reference: https://justicehui.github.io/hard-algorithm/2019/10/22/SplayTree3/
// Reference: https://justicehui.github.io/hard-algorithm/2019/10/23/SplayTree4/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut scores = vec![vec![0; n]; 3];
    let mut ret = 0;

    for i in 0..3 {
        for j in 0..n {
            scores[i][j] = scan.token::<i64>();
        }
    }

    let mut tree = SplayTree {
        root: Node::new_with_parent(-1, 0, std::ptr::null_mut()),
    };

    unsafe {
        (*tree.root).right = Node::new_with_parent(2_000_000_001, 2_000_000_001, tree.root);

        for i in 0..n {
            let left = scores[0][i].min(scores[1][i]).min(scores[2][i]);
            let right = scores[0][i].max(scores[1][i]).max(scores[2][i]);

            let pos_left = tree.find(left);
            let pos_right = tree.find(right);

            if pos_left == pos_right {
                if pos_right != ret {
                    tree.kth(pos_left + 1);
                    (*tree.root).update();
                    (*tree.root).value = left;
                } else {
                    tree.kth(pos_left);

                    let node = Node::new_with_parent(left, pos_left + 1, tree.root);
                    (*node).right = (*tree.root).right;
                    (*(*tree.root).right).parent = node;
                    (*tree.root).right = node;

                    ret += 1;
                }
            } else {
                tree.kth(pos_left);

                let node = Node::new_with_parent(left, pos_left + 1, tree.root);
                (*node).right = (*tree.root).right;
                (*(*tree.root).right).parent = node;
                (*tree.root).right = node;

                if pos_right != ret {
                    tree.kth(pos_right + 2);
                    tree.delete();
                } else {
                    ret += 1;
                }

                tree.gather(pos_left + 2, pos_right + 1);
                (*(*(*tree.root).right).left).lazy += 1;
                (*(*(*tree.root).right).left).update();
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}

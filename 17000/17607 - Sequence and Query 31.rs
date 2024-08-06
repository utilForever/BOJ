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
    max: i64,
    max_left: i64,
    max_right: i64,
}

impl LazyNode {
    fn new(max: i64, max_left: i64, max_right: i64) -> Self {
        Self {
            max,
            max_left,
            max_right,
        }
    }
}

struct Node {
    left: *mut Node,
    right: *mut Node,
    parent: *mut Node,
    value: i64,
    count: i64,
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
            lazy: LazyNode::new(value, value, value),
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
            lazy: LazyNode::new(value, value, value),
            flip: false,
        }))
    }

    unsafe fn update(&mut self) {
        self.push();

        self.count = 1;
        self.lazy.max = self.value;
        self.lazy.max_left = self.value;
        self.lazy.max_right = self.value;

        if self.left != std::ptr::null_mut() && self.right != std::ptr::null_mut() {
            (*self.left).push();
            (*self.right).push();

            self.count += (*self.left).count + (*self.right).count;
            self.lazy.max_left = (*self.left).lazy.max_left
                + if (*self.left).count == (*self.left).lazy.max_left && self.value == 1 {
                    1 + (*self.right).lazy.max_left
                } else {
                    0
                };
            self.lazy.max_right = (*self.right).lazy.max_right
                + if (*self.right).count == (*self.right).lazy.max_right && self.value == 1 {
                    1 + (*self.left).lazy.max_right
                } else {
                    0
                };
            self.lazy.max =
                (*self.left)
                    .lazy
                    .max
                    .max((*self.right).lazy.max)
                    .max(if self.value == 1 {
                        (*self.left).lazy.max_right + 1 + (*self.right).lazy.max_left
                    } else {
                        0
                    });
        } else if self.left != std::ptr::null_mut() {
            (*self.left).push();

            self.count += (*self.left).count;
            self.lazy.max_left = (*self.left).lazy.max_left
                + if (*self.left).count == (*self.left).lazy.max_left {
                    self.value
                } else {
                    0
                };
            self.lazy.max_right = if self.value == 1 {
                1 + (*self.left).lazy.max_right
            } else {
                0
            };
            self.lazy.max = (*self.left).lazy.max.max(if self.value == 1 {
                1 + (*self.left).lazy.max_right
            } else {
                0
            });
        } else if self.right != std::ptr::null_mut() {
            (*self.right).push();

            self.count += (*self.right).count;
            self.lazy.max_left = if self.value == 1 {
                1 + (*self.right).lazy.max_left
            } else {
                0
            };
            self.lazy.max_right = (*self.right).lazy.max_right
                + if (*self.right).count == (*self.right).lazy.max_right {
                    self.value
                } else {
                    0
                };
            self.lazy.max = (*self.right).lazy.max.max(if self.value == 1 {
                1 + (*self.right).lazy.max_left
            } else {
                0
            });
        }
    }

    unsafe fn push(&mut self) {
        if self.flip {
            let temp = (*self).left;
            (*self).left = (*self).right;
            (*self).right = temp;

            let temp = (*self).lazy.max_left;
            (*self).lazy.max_left = (*self).lazy.max_right;
            (*self).lazy.max_right = temp;

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

        let q = scan.token::<i64>();

        for _ in 0..q {
            let command = scan.token::<i64>();

            if command == 1 {
                let (l, r) = (scan.token::<i64>(), scan.token::<i64>());
                tree.flip(l, r);
            } else {
                let (l, r) = (scan.token::<i64>(), scan.token::<i64>());
                let ret = tree.gather(l, r);

                writeln!(out, "{}", (*ret).lazy.max).unwrap();
            }
        }
    }
}

use io::Write;
use std::{
    cmp::Ordering,
    io::{self, BufWriter, StdoutLock},
    str,
};

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

type NodeRef = Option<Box<Node>>;

#[derive(Clone)]
pub struct Node {
    value: i64,
    cnt: usize,

    min: i64,
    max: i64,
    sum: i64,

    is_dummy: bool,
    is_flipped: bool,

    left: NodeRef,
    right: NodeRef,
}

impl Node {
    fn new_node_ref(value: i64) -> NodeRef {
        Some(Box::new(Node {
            value,
            cnt: 1,
            min: value,
            max: value,
            sum: value,
            is_dummy: false,
            is_flipped: false,
            left: None,
            right: None,
        }))
    }

    // fn get_min(tree: &mut NodeRef) -> Option<i64> {
    //     let mut current = tree;

    //     while let Some(node) = current {
    //         current = match node.left {
    //             Some(_) => &mut current.as_mut()?.left,
    //             None => break,
    //         }
    //     }

    //     let node = current.take()?;
    //     *current = node.right;

    //     Some(node.value)
    // }

    // fn remove(tree: &mut NodeRef) -> Option<i64> {
    //     let mut node = tree.take()?;

    //     *tree = match (node.left.as_ref(), node.right.as_ref()) {
    //         (None, None) => None,
    //         (Some(_), None) => node.left.take(),
    //         (None, Some(_)) => node.right.take(),
    //         (Some(_), Some(_)) => Some(Box::new(Node {
    //             value: Self::get_min(&mut node.right)?,
    //             cnt: 1,
    //             left: node.left.take(),
    //             right: node.right.take(),
    //         })),
    //     };

    //     Some(node.value)
    // }

    fn left_rotate(root: &mut NodeRef) {
        if let Some(mut node) = root.take() {
            if let Some(mut new_root) = node.right.take() {
                node.right = new_root.left.take();

                new_root.left = Some(node);
                Node::update(&mut new_root.left);

                *root = Some(new_root);
                Node::update(root);
            }
        }
    }

    fn right_rotate(root: &mut NodeRef) {
        if let Some(mut node) = root.take() {
            if let Some(mut new_root) = node.left.take() {
                node.left = new_root.right.take();

                new_root.right = Some(node);
                Node::update(&mut new_root.right);

                *root = Some(new_root);
                Node::update(root);
            }
        }
    }

    fn update(node: &mut NodeRef) {
        if let Some(mut node) = node.take() {
            node.cnt = 1;
            node.min = node.value;
            node.max = node.value;
            node.sum = node.value;

            if let Some(left) = &node.left {
                node.cnt += left.cnt;
                node.min = node.min.min(left.min);
                node.max = node.max.max(left.max);
                node.sum += left.sum;
            }

            if let Some(right) = &node.right {
                node.cnt += right.cnt;
                node.min = node.min.min(right.min);
                node.max = node.max.max(right.max);
                node.sum += right.sum;
            }
        }
    }

    fn push(node: &mut NodeRef) {
        if let Some(mut node) = node.take() {
            if !node.is_flipped {
                return;
            }

            std::mem::swap(&mut node.left, &mut node.right);

            if let Some(left) = &mut node.left {
                left.is_flipped ^= true;
            }

            if let Some(right) = &mut node.right {
                right.is_flipped ^= true;
            }

            node.is_flipped = false;
        }
    }
}

#[derive(Default)]
pub struct SplayTree {
    root: NodeRef,
    nodes: Vec<NodeRef>,
}

impl SplayTree {
    fn init(&mut self, n: i64) {
        if self.root.is_some() {
            self.root = None;
        }

        self.root = Some(Box::new(Node {
            value: i64::MIN,
            cnt: 1,
            min: i64::MIN,
            max: i64::MIN,
            sum: i64::MIN,
            is_dummy: false,
            is_flipped: false,
            left: None,
            right: None,
        }));
        self.nodes = vec![None; (n + 1) as usize];

        let mut node = self.root_mut().take().unwrap();

        for i in 1..=n {
            node.right = Some(Box::new(Node {
                value: i,
                cnt: 1,
                min: i,
                max: i,
                sum: i,
                is_dummy: false,
                is_flipped: false,
                left: None,
                right: None,
            }));

            self.nodes[i as usize] = node.right.take();
            node = node.right.take().unwrap();
        }

        node.right = Some(Box::new(Node {
            value: i64::MAX,
            cnt: 1,
            min: i64::MAX,
            max: i64::MAX,
            sum: i64::MAX,
            is_dummy: false,
            is_flipped: false,
            left: None,
            right: None,
        }));

        self.root_mut().as_mut().unwrap().is_dummy = true;
        node.right.as_mut().unwrap().is_dummy = true;

        for i in (1..=n).rev() {
            Node::update(&mut self.nodes[i as usize]);
        }

        Self::splay(&mut self.nodes[n as usize / 2], &(n / 2));
    }

    fn root_mut(&mut self) -> &mut NodeRef {
        &mut self.root
    }

    fn root(&self) -> &NodeRef {
        &self.root
    }

    // fn insert(&mut self, value: i64) {
    //     Self::splay(self.root_mut(), &value);

    //     let root = self.root_mut().take();

    //     *self.root_mut() = match root {
    //         Some(mut node) => match node.value.cmp(&value) {
    //             Ordering::Equal => Some(node),
    //             Ordering::Less => Some(Box::new(Node {
    //                 value,
    //                 cnt: 1,
    //                 right: node.right.take(),
    //                 left: Some(node),
    //             })),
    //             Ordering::Greater => Some(Box::new(Node {
    //                 value,
    //                 cnt: 1,
    //                 left: node.left.take(),
    //                 right: Some(node),
    //             })),
    //         },
    //         None => Node::new_node_ref(value),
    //     }
    // }

    // fn remove(&mut self, value: &i64) -> Option<i64> {
    //     Self::splay(self.root_mut(), value);

    //     let node = self.root_mut().as_mut()?;

    //     match node.value.cmp(value) {
    //         Ordering::Equal => Node::remove(self.root_mut()),
    //         _ => None,
    //     }
    // }

    // fn find(&mut self, value: &i64) -> bool {
    //     Self::splay(self.root_mut(), value);

    //     self.root()
    //         .as_ref()
    //         .map_or(false, |node| &node.value == value)
    // }

    fn kth(&mut self, mut k: usize) {
        let mut node = self.root_mut().take();
        Node::push(&mut node);

        loop {
            if let Some(node) = node {
                while let Some(left) = &node.left {
                    if left.cnt > k {
                        node = node.left.take().unwrap();
                    } else {
                        break;
                    }
                }

                if let Some(left) = &node.left {
                    k -= left.cnt;
                }

                k -= 1;

                if k == 0 {
                    break;
                }

                node = node.right.take().unwrap();
            }
        }

        Self::splay(&mut node, &node.unwrap().value);
    }

    fn get_idx(&mut self, idx: usize) -> i64 {
        Self::splay(&mut self.nodes[idx], &self.nodes[idx].unwrap().value);

        self.root().unwrap().left.unwrap().value
    }

    fn gather(&mut self, left: usize, right: usize) -> NodeRef {
        self.kth(right + 1);

        let tmp = self.root_mut().take();

        self.kth(left - 1);
    }

    fn print(tree: &mut NodeRef, out: &mut BufWriter<StdoutLock>) {
        Node::push(tree);

        if let Some(node) = tree {
            Self::print(&mut node.left, out);

            if !node.is_dummy {
                writeln!(out, "{} ", node.value).unwrap();
            }

            Self::print(&mut node.right, out);
        }
    }

    fn splay(tree: &mut NodeRef, value: &i64) {
        if let Some(grandparent) = tree.as_mut() {
            match grandparent.value.cmp(value) {
                Ordering::Greater => Self::splay_left(tree, value),
                Ordering::Less => Self::splay_right(tree, value),
                Ordering::Equal => (),
            }
        }
    }

    fn splay_left(tree: &mut NodeRef, value: &i64) {
        let grandparent = tree.as_mut().unwrap();

        if let Some(parent) = grandparent.left.as_mut() {
            match parent.value.cmp(value) {
                Ordering::Greater => {
                    Self::splay(&mut parent.left, value);
                    Node::right_rotate(tree);
                }
                Ordering::Less => {
                    Self::splay(&mut parent.right, value);
                    Node::left_rotate(tree);
                }
                Ordering::Equal => (),
            }

            Node::right_rotate(tree);
        }
    }

    fn splay_right(tree: &mut NodeRef, value: &i64) {
        let grandparent = tree.as_mut().unwrap();

        if let Some(parent) = grandparent.right.as_mut() {
            match parent.value.cmp(value) {
                Ordering::Greater => {
                    Self::splay(&mut parent.left, value);
                    Node::right_rotate(tree);
                }
                Ordering::Less => {
                    Self::splay(&mut parent.right, value);
                    Node::left_rotate(tree);
                }
                Ordering::Equal => (),
            }

            Node::left_rotate(tree);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<i64>(), scan.token::<i64>());

    let mut tree = SplayTree::default();
    tree.init(n);

    for _ in 0..q {
        let op = scan.token::<i64>();

        match op {
            1 => {
                let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
                tree.flip(l, r);

                let node = tree.gather(l, r);
                writeln!(out, "{} {} {}", node.min, node.max, node.sum);
            }
            2 => {
                let (l, r, x) = (
                    scan.token::<usize>(),
                    scan.token::<usize>(),
                    scan.token::<i64>(),
                );
                let node = tree.gather(l, r);

                writeln!(out, "{} {} {}", node.min, node.max, node.sum);

                tree.shift(l, r, x);
            }
            3 => {
                let i = scan.token::<usize>();
                tree.kth(i);

                writeln!(out, "{}", tree.root().as_ref().unwrap().value).unwrap();
            }
            4 => {
                let x = scan.token::<usize>();

                writeln!(out, "{}", tree.get_idx(x)).unwrap();
            }
            _ => (),
        }
    }

    SplayTree::print(&mut tree.root(), &mut out);
}

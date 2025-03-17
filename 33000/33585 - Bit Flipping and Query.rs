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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const BITS: i32 = 19;

#[derive(Default)]
struct Node {
    count: usize,
    lazy: u32,
    children: [Option<Box<Node>>; 2],
}

impl Node {
    fn new() -> Self {
        Node {
            count: 0,
            lazy: 0,
            children: [None, None],
        }
    }

    fn propagate(&mut self, depth: i32) {
        if depth < 0 || self.lazy == 0 {
            return;
        }

        let flip_curr = (self.lazy >> depth) & 1;
        let mask_lower = (1 << depth) - 1;

        for child in self.children.iter_mut() {
            if let Some(ref mut child) = child {
                child.lazy ^= self.lazy & mask_lower;
            }
        }

        if flip_curr == 1 {
            self.children.swap(0, 1);
        }

        self.lazy = 0;
    }

    fn insert(&mut self, val: u32, depth: i32) {
        self.count += 1;

        if depth < 0 {
            return;
        }

        self.propagate(depth);

        let bit = ((val >> depth) & 1) as usize;

        if self.children[bit].is_none() {
            self.children[bit] = Some(Box::new(Node::new()));
        }

        self.children[bit].as_mut().unwrap().insert(val, depth - 1);
    }

    fn kth(&self, k: usize, depth: i32, acc: u32, lazy: u32) -> u32 {
        let lazy_total = lazy ^ self.lazy;

        if depth < 0 {
            return acc;
        }

        let bit_flip = (lazy_total >> depth) & 1;
        let child_left_idx = bit_flip as usize;
        let child_right_idx = 1 - child_left_idx;

        let left_count = self.children[child_left_idx]
            .as_ref()
            .map_or(0, |child| child.count);

        if k <= left_count {
            self.children[child_left_idx]
                .as_ref()
                .unwrap()
                .kth(k, depth - 1, acc, lazy_total)
        } else {
            self.children[child_right_idx].as_ref().unwrap().kth(
                k - left_count,
                depth - 1,
                acc | (1 << depth),
                lazy_total,
            )
        }
    }
}

fn split(
    node: Option<Box<Node>>,
    threshold: u32,
    depth: i32,
    prefix: u32,
) -> (Option<Box<Node>>, Option<Box<Node>>) {
    if node.is_none() {
        return (None, None);
    }

    let mut node = node.unwrap();

    if depth >= 0 {
        node.propagate(depth);
    }

    let range = if depth >= 0 { 1 << (depth + 1) } else { 1 };

    if prefix + range <= threshold {
        return (Some(node), None);
    }

    if prefix >= threshold {
        return (None, Some(node));
    }

    let mut node_left = Box::new(Node::new());
    let mut node_right = Box::new(Node::new());

    if depth >= 0 {
        for b in 0..2 {
            let new_prefix = prefix | ((b as u32) << depth);
            let (l_child, r_child) =
                split(node.children[b].take(), threshold, depth - 1, new_prefix);

            node_left.children[b] = l_child;
            node_right.children[b] = r_child;
        }
    }

    let node_left_child_left = node_left.children[0]
        .as_ref()
        .map_or(0, |child| child.count);
    let node_left_child_right = node_left.children[1]
        .as_ref()
        .map_or(0, |child| child.count);
    node_left.count = node_left_child_left + node_left_child_right;

    let node_right_child_left = node_right.children[0]
        .as_ref()
        .map_or(0, |child| child.count);
    let node_right_child_right = node_right.children[1]
        .as_ref()
        .map_or(0, |child| child.count);
    node_right.count = node_right_child_left + node_right_child_right;

    let ret_left = if node_left.count == 0 {
        None
    } else {
        Some(node_left)
    };

    let ret_right = if node_right.count == 0 {
        None
    } else {
        Some(node_right)
    };

    (ret_left, ret_right)
}

fn merge(a: Option<Box<Node>>, b: Option<Box<Node>>, depth: i32) -> Option<Box<Node>> {
    match (a, b) {
        (None, None) => None,
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (Some(mut a), Some(mut b)) => {
            if depth < 0 {
                a.count += b.count;
                return Some(a);
            }

            a.propagate(depth);
            b.propagate(depth);

            for i in 0..2 {
                a.children[i] = merge(a.children[i].take(), b.children[i].take(), depth - 1);
            }

            let left = a.children[0].as_ref().map_or(0, |child| child.count);
            let right = a.children[1].as_ref().map_or(0, |child| child.count);

            a.count = left + right;

            Some(a)
        }
    }
}

struct BitTrie {
    root: Option<Box<Node>>,
}

impl BitTrie {
    fn new() -> Self {
        BitTrie { root: None }
    }

    fn insert(&mut self, val: u32) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new()));
        }

        self.root.as_mut().unwrap().insert(val, BITS);
    }

    fn kth(&self, k: usize) -> Option<u32> {
        if let Some(ref root) = self.root {
            if k > root.count {
                return None;
            }

            Some(root.kth(k, BITS, 0, 0))
        } else {
            None
        }
    }

    fn flip_bit(&mut self, start: u32, end: u32, bit: u32) {
        let (left, tmp) = split(self.root.take(), start, BITS, 0);
        let (mut mid, right) = split(tmp, end + 1, BITS, 0);

        if let Some(ref mut mid) = mid {
            mid.lazy ^= 1 << bit;
        }

        self.root = merge(merge(left, mid, BITS), right, BITS);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut trie = BitTrie::new();

    for _ in 0..n {
        let num = scan.token::<u32>();
        trie.insert(num);
    }

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let (l, r, k) = (
                scan.token::<u32>(),
                scan.token::<u32>(),
                scan.token::<u32>(),
            );
            trie.flip_bit(l, r, k);
        } else {
            let k = scan.token::<usize>();
            writeln!(out, "{}", trie.kth(k).unwrap()).unwrap();
        }
    }
}

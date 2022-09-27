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

#[derive(Clone)]
struct Node {
    max: i64,
    min: i64,
    rise_max: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self {
            max: val,
            min: val,
            rise_max: 0,
        }
    }

    fn merge(&mut self, other: &Self) -> Node {
        Node {
            max: self.max.max(other.max),
            min: self.min.min(other.min),
            rise_max: self.rise_max.max(other.rise_max).max(other.max - self.min),
        }
    }
}

fn update(tree: &mut Vec<Node>, cur: usize, index: usize, val: i64, start: usize, end: usize) {
    if index > end || index < start {
        return;
    }

    if start == end {
        tree[cur] = Node::new(val);
        return;
    }

    if start != end {
        let mid = (start + end) / 2;
        update(tree, cur * 2, index, val, start, mid);
        update(tree, cur * 2 + 1, index, val, mid + 1, end);

        let mut left = tree[cur * 2].clone();
        let right = tree[cur * 2 + 1].clone();
        tree[cur] = left.merge(&right);
    }
}

fn query(tree: &Vec<Node>, cur: usize, start: usize, end: usize, i: usize, j: usize) -> Node {
    if i > end {
        return Node::new(1_000_000_000_000);
    }

    if j < start {
        return Node::new(-1_000_000_000_000);
    }

    if i <= start && j >= end {
        return tree[cur].clone();
    }

    let mid = (start + end) / 2;
    let mut left = query(tree, cur * 2, start, mid, i, j);
    let right = query(tree, cur * 2 + 1, mid + 1, end, i, j);

    left.merge(&right)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut arr = vec![1; n + 1];
    let mut tree = vec![Node::new(0); 4 * (n + 1)];

    for i in 1..=n {
        arr[i] = scan.token();
        update(&mut tree, 1, i, arr[i], 1, n);
    }

    let q = scan.token::<i64>();

    for _ in 1..=q {
        let num = scan.token::<i64>();

        if num == 1 {
            let (k, x) = (scan.token::<usize>(), scan.token::<i64>());
            update(&mut tree, 1, k, x, 1, n);
        } else {
            let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(out, "{}", query(&tree, 1, 1, n, a, b).rise_max).unwrap();
        }
    }
}

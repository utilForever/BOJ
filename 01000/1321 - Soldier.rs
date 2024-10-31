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

#[derive(Clone, Debug)]
struct Node {
    val: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self { val }
    }

    fn merge(&mut self, other: &Self) -> Node {
        let mut ret = Node::new(0);
        ret.val = self.val + other.val;

        ret
    }
}

struct SegmentTree {
    size: usize,
    data: Vec<Node>,
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![Node::new(0); real_n * 4],
        }
    }

    pub fn update(&mut self, index: usize, val: i64) {
        self.update_internal(index, val, 1, 1, self.size);
    }

    fn update_internal(
        &mut self,
        index: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if index > node_end || index < node_start {
            return;
        }

        if node_start == node_end {
            self.data[node].val += val;
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(index, val, node * 2, node_start, mid);
        self.update_internal(index, val, node * 2 + 1, mid + 1, node_end);

        let mut left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();
        self.data[node] = left.merge(&right);
    }

    fn query(&mut self, start: usize, end: usize, idx: i64) -> usize {
        self.query_internal(start, end, idx, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        idx: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> usize {
        if node_start == node_end {
            return node_start;
        }

        let mid = (node_start + node_end) / 2;
        let val_left = self.data[node * 2].val;

        if idx <= val_left {
            return self.query_internal(start, end, idx, node * 2, node_start, mid);
        } else {
            return self.query_internal(
                start,
                end,
                idx - val_left,
                node * 2 + 1,
                mid + 1,
                node_end,
            );
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut tree = SegmentTree::new(n + 1);

    for i in 1..=n {
        let soldiers = scan.token::<i64>();
        tree.update(i, soldiers);
    }

    let m = scan.token::<i64>();

    for _ in 0..m {
        let command = scan.token::<i64>();

        if command == 1 {
            let (i, a) = (scan.token::<usize>(), scan.token::<i64>());
            tree.update(i, a);
        } else {
            let idx = scan.token::<i64>();
            writeln!(out, "{}", tree.query(1, n, idx)).unwrap();
        }
    }
}

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
        Node { val }
    }

    fn merge(&mut self, other: &Self) -> Node {
        Node {
            val: self.val + other.val,
        }
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

    fn query(&mut self, start: usize, end: usize, k: i64) -> Node {
        self.query_internal(start, end, 1, 1, self.size, k)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
        k: i64,
    ) -> Node {
        if node_start == node_end {
            return Node::new(node_start as i64);
        }

        let mid = (node_start + node_end) / 2;

        if k <= self.data[node * 2].val {
            self.query_internal(start, end, node * 2, node_start, mid, k)
        } else {
            self.query_internal(
                start,
                end,
                node * 2 + 1,
                mid + 1,
                node_end,
                k - self.data[node * 2].val,
            )
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut tree = SegmentTree::new(2_000_001);

    for _ in 0..n {
        let command = scan.token::<i64>();
        let x = scan.token::<usize>();

        if command == 1 {
            tree.update(x, 1);
        } else {
            let ret = tree.query(1, 2_000_000, x as i64);
            writeln!(out, "{}", ret.val).unwrap();
            tree.update(ret.val as usize, -1);
        }
    }
}

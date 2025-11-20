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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

#[derive(Debug, Clone, Default)]
struct Node {
    len: i64,
    prefix: i64,
    suffix: i64,
    best: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self {
            len: 1,
            prefix: val,
            suffix: val,
            best: val,
        }
    }

    fn identity() -> Self {
        Self {
            len: 0,
            prefix: 0,
            suffix: 0,
            best: 0,
        }
    }

    fn merge(&self, other: &Self) -> Node {
        if self.len == 0 {
            return other.clone();
        }

        if other.len == 0 {
            return self.clone();
        }

        let len = self.len + other.len;
        let prefix = if self.prefix == self.len {
            self.len + other.prefix
        } else {
            self.prefix
        };
        let suffix = if other.suffix == other.len {
            other.len + self.suffix
        } else {
            other.suffix
        };
        let best = self.best.max(other.best).max(self.suffix + other.prefix);

        Node {
            len,
            prefix,
            suffix,
            best,
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
            self.data[node] = Node::new(val);
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(index, val, node * 2, node_start, mid);
        self.update_internal(index, val, node * 2 + 1, mid + 1, node_end);

        let left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();
        self.data[node] = left.merge(&right);
    }

    fn query(&mut self, start: usize, end: usize) -> Node {
        self.query_internal(start, end, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> Node {
        if end < node_start || node_end < start {
            return Node::identity();
        }

        if start <= node_start && node_end <= end {
            return self.data[node].clone();
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        Node::merge(&left, &right)
    }
}

fn convert_mask(c: char) -> u8 {
    match c {
        'K' => 0,
        'R' => 4,
        'G' => 2,
        'B' => 1,
        'Y' => 6,
        'C' => 3,
        'P' => 5,
        'W' => 7,
        _ => unreachable!(),
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut tree_red = SegmentTree::new(n + 1);
    let mut tree_green = SegmentTree::new(n + 1);
    let mut tree_blue = SegmentTree::new(n + 1);

    for _ in 0..k {
        let cmd = scan.token::<String>();

        if cmd == "Q" {
            let (i, j) = (scan.token::<usize>(), scan.token::<usize>());
            let ret_red = tree_red.query(i, j).best;
            let ret_green = tree_green.query(i, j).best;
            let ret_blue = tree_blue.query(i, j).best;

            writeln!(out, "{}", ret_red.max(ret_green).max(ret_blue)).unwrap();
        } else {
            let (i, x) = (scan.token::<usize>(), scan.token::<char>());
            let mask = convert_mask(x);

            let val_red = if mask & 4 == 0 { 1 } else { 0 };
            let val_green = if mask & 2 == 0 { 1 } else { 0 };
            let val_blue = if mask & 1 == 0 { 1 } else { 0 };

            tree_red.update(i, val_red);
            tree_green.update(i, val_green);
            tree_blue.update(i, val_blue);
        }
    }
}

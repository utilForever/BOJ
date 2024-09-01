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

const TRANSITIONS_ZERO: [usize; 9] = [0, 3, 3, 3, 9, 7, 2, 3, 2];
const TRANSITIONS_ONE: [usize; 9] = [0, 4, 8, 5, 6, 9, 9, 4, 8];
const ACCEPTS: [bool; 9] = [false, false, true, false, false, false, false, true, true];

#[derive(Clone, Default)]
struct Node {
    table: [[usize; 9]; 4],
}

impl Node {
    fn new(val: i64) -> Self {
        Node::construct(
            &TRANSITIONS_ZERO,
            if val == 0 {
                &TRANSITIONS_ZERO
            } else {
                &TRANSITIONS_ONE
            },
            if val == 0 {
                &TRANSITIONS_ONE
            } else {
                &TRANSITIONS_ZERO
            },
            &TRANSITIONS_ONE,
        )
    }

    pub fn construct(
        table00: &[usize; 9],
        table01: &[usize; 9],
        table10: &[usize; 9],
        table11: &[usize; 9],
    ) -> Node {
        let mut table = [[0; 9]; 4];

        for i in 1..=8 {
            table[0][i] = table00[i];
            table[1][i] = table01[i];
            table[2][i] = table10[i];
            table[3][i] = table11[i];
        }

        Node { table }
    }
}

struct LazySegmentTree {
    size: usize,
    data: Vec<Node>,
    lazy: Vec<(bool, bool)>,
}

impl LazySegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![Node::default(); real_n * 4],
            lazy: vec![(false, true); real_n * 4],
        }
    }

    fn merge(a: &Node, b: &Node) -> Node {
        let mut table = [[0; 9]; 4];

        for i in 0..4 {
            for j in 1..=8 {
                table[i][j] = if a.table[i][j] == 9 {
                    9
                } else {
                    b.table[i][a.table[i][j]]
                };
            }
        }

        Node { table }
    }

    pub fn construct(&mut self, arr: &[i64], start: usize, end: usize) {
        self.construct_internal(arr, 1, start, end);
    }

    fn construct_internal(&mut self, arr: &[i64], node: usize, start: usize, end: usize) -> Node {
        if start == end {
            self.data[node] = Node::new(arr[start]);
            self.data[node].clone()
        } else {
            let mid = (start + end) / 2;

            let left = self.construct_internal(arr, node * 2, start, mid);
            let right = self.construct_internal(arr, node * 2 + 1, mid + 1, end);

            self.data[node] = LazySegmentTree::merge(&left, &right);
            self.data[node].clone()
        }
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if start != end {
            self.lazy[node * 2] = (
                if self.lazy[node * 2].0 {
                    self.lazy[node].1
                } else {
                    self.lazy[node].0
                },
                if self.lazy[node * 2].1 {
                    self.lazy[node].1
                } else {
                    self.lazy[node].0
                },
            );
            self.lazy[node * 2 + 1] = (
                if self.lazy[node * 2 + 1].0 {
                    self.lazy[node].1
                } else {
                    self.lazy[node].0
                },
                if self.lazy[node * 2 + 1].1 {
                    self.lazy[node].1
                } else {
                    self.lazy[node].0
                },
            );
        }

        let idx = if self.lazy[node].0 { 2 } else { 0 } + if self.lazy[node].1 { 1 } else { 0 };

        self.data[node] = Node::construct(
            &self.data[node].table[0],
            &self.data[node].table[idx],
            &self.data[node].table[idx ^ 3],
            &self.data[node].table[3],
        );
        self.lazy[node] = (false, true);
    }

    pub fn update(&mut self, start: usize, end: usize, val: (bool, bool)) {
        self.update_internal(start, end, val, 1, 1, self.size);
    }

    fn update_internal(
        &mut self,
        start: usize,
        end: usize,
        val: (bool, bool),
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return;
        }

        if start <= node_start && node_end <= end {
            self.lazy[node] = (
                if self.lazy[node].0 { val.1 } else { val.0 },
                if self.lazy[node].1 { val.1 } else { val.0 },
            );
            self.propagate(node, node_start, node_end);
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(start, end, val, node * 2, node_start, mid);
        self.update_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = LazySegmentTree::merge(&self.data[node * 2], &self.data[node * 2 + 1]);
    }

    pub fn can_accept(&mut self, n: usize) -> bool {
        let ret = self.query(1, n);
        let state_final = ret.table[1][1];

        state_final != 9 && ACCEPTS[state_final]
    }

    pub fn query(&mut self, start: usize, end: usize) -> Node {
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
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return Node::default();
        }

        if start <= node_start && node_end <= end {
            return self.data[node].clone();
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        LazySegmentTree::merge(&left, &right)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = scan
        .token::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();
    let q = scan.token::<i64>();

    nums.insert(0, 0);

    let mut tree = LazySegmentTree::new(n);
    tree.construct(&nums, 1, n);

    writeln!(out, "{}", if tree.can_accept(n) { "YES" } else { "NO" }).unwrap();

    for _ in 0..q {
        let (bit, l, r) = (
            scan.token::<String>().chars().collect::<Vec<_>>(),
            scan.token::<usize>() + 1,
            scan.token::<usize>() + 1,
        );
        let val = (bit[0] == '1', bit[1] == '1');
        tree.update(l, r, val);

        writeln!(out, "{}", if tree.can_accept(n) { "YES" } else { "NO" }).unwrap();
    }
}

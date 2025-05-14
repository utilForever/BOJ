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

const MOD: i64 = 1_000_000_009;

#[derive(Copy, Clone)]
struct Node {
    matrix: [[i64; 3]; 3],
    valid: bool,
    rows: usize,
    cols: usize,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            matrix: [[0; 3]; 3],
            valid: false,
            rows: 0,
            cols: 0,
        }
    }
}

impl Node {
    fn new() -> Self {
        Self {
            matrix: [[0; 3]; 3],
            valid: true,
            rows: 2,
            cols: 2,
        }
    }

    fn merge(a: &Node, b: &Node) -> Node {
        if !a.valid || !b.valid || a.cols != b.rows {
            return Node::default();
        }

        let mut ret = Node::new();
        ret.rows = a.rows;
        ret.cols = b.cols;

        for i in 0..a.rows {
            for j in 0..b.cols {
                let mut sum = 0;

                for k in 0..a.cols {
                    sum = (sum + a.matrix[i][k] * b.matrix[k][j]) % MOD;
                }

                ret.matrix[i][j] = sum;
            }
        }

        ret
    }
}

#[derive(Clone)]
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
            data: vec![Node::default(); real_n * 4],
        }
    }

    pub fn build(&mut self, idx: usize, start: usize, end: usize) {
        if start == end {
            self.data[idx] = Node::new();
            return;
        }

        let mid = (start + end) / 2;
        self.build(idx * 2, start, mid);
        self.build(idx * 2 + 1, mid + 1, end);

        let left = self.data[idx * 2].clone();
        let right = self.data[idx * 2 + 1].clone();

        self.data[idx] = Node::merge(&left, &right);
    }

    pub fn update_matrix(&mut self, cmd: i64, idx: usize) {
        self.update_matrix_internal(cmd, idx, 1, 1, self.size);
    }

    fn update_matrix_internal(
        &mut self,
        cmd: i64,
        idx: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if idx > node_end || idx < node_start {
            return;
        }

        if node_start == node_end {
            match cmd {
                2 => {
                    for c in 0..self.data[node].cols {
                        self.data[node].matrix[2][c] = 0;
                    }

                    self.data[node].rows = 3;
                }
                3 => {
                    self.data[node].rows = 2;
                }
                4 => {
                    for r in 0..self.data[node].rows {
                        self.data[node].matrix[r][2] = 0;
                    }

                    self.data[node].cols = 3;
                }
                5 => {
                    self.data[node].cols = 2;
                }
                _ => unreachable!(),
            }

            self.data[node].valid = true;
            return;
        }

        let mid = (node_start + node_end) / 2;

        if idx <= mid {
            self.update_matrix_internal(cmd, idx, node * 2, node_start, mid);
        } else {
            self.update_matrix_internal(cmd, idx, node * 2 + 1, mid + 1, node_end);
        }

        let left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();

        self.data[node] = Node::merge(&left, &right);
    }

    fn update_val(&mut self, idx: usize, row: usize, col: usize, val: i64) {
        self.update_val_interval(idx, row, col, val, 1, 1, self.size);
    }

    fn update_val_interval(
        &mut self,
        idx: usize,
        row: usize,
        col: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if node_start == node_end {
            self.data[node].matrix[row][col] = val;
            self.data[node].valid = true;
            return;
        }

        let mid = (node_start + node_end) / 2;

        if idx <= mid {
            self.update_val_interval(idx, row, col, val, node * 2, node_start, mid);
        } else {
            self.update_val_interval(idx, row, col, val, node * 2 + 1, mid + 1, node_end);
        }

        let left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();

        self.data[node] = Node::merge(&left, &right);
    }

    fn query(&mut self, start: usize, end: usize) -> Option<Node> {
        self.query_internal(start, end, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> Option<Node> {
        if end < node_start || node_end < start {
            return None;
        }

        if start <= node_start && node_end <= end {
            return Some(self.data[node].clone());
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        match (left, right) {
            (Some(l), Some(r)) => Some(Node::merge(&l, &r)),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let q = scan.token::<i64>();

    let mut tree = SegmentTree::new(n);
    tree.build(1, 1, n);

    for _ in 0..q {
        let cmd = scan.token::<i64>();

        if cmd == 1 {
            let (l, r, i, j) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>() - 1,
                scan.token::<usize>() - 1,
            );
            let ret = tree.query(l, r);

            if let Some(val) = ret {
                if val.valid {
                    writeln!(out, "{}", val.matrix[i][j]).unwrap();
                } else {
                    writeln!(out, "-1").unwrap();
                }
            } else {
                writeln!(out, "-1").unwrap();
            }
        } else if cmd >= 2 && cmd <= 5 {
            let i = scan.token::<usize>();
            tree.update_matrix(cmd, i);
        } else {
            let (i, j, k, v) = (
                scan.token::<usize>(),
                scan.token::<usize>() - 1,
                scan.token::<usize>() - 1,
                scan.token::<i64>(),
            );
            tree.update_val(i, j, k, v);
        }
    }
}

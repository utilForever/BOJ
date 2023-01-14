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
    values: [[i64; 3]; 3],
}

impl Node {
    fn new(val: i64) -> Self {
        Self {
            values: [[val; 3]; 3],
        }
    }

    fn merge(&mut self, other: &Self) -> Node {
        let mut ret = Node::new(1 << 60);

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        if j == k {
                            continue;
                        }

                        ret.values[i][l] =
                            ret.values[i][l].min(self.values[i][j] + other.values[k][l]);
                    }
                }
            }
        }

        ret
    }
}

struct SegmentTree {
    size: usize,
    data: Vec<Node>,
    r: i64,
    c: i64,
}

impl SegmentTree {
    pub fn new(n: usize, r: i64, c: i64) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![Node::new(1 << 60); real_n * 4],
            r,
            c,
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
            self.data[node] = Node::new(1 << 60);

            if val > 0 {
                self.data[node].values[0][0] = self.r.min((val.abs() + 1) * self.c);
                self.data[node].values[1][1] = 0;
                self.data[node].values[2][2] = self.c * val.abs();
            } else if val < 0 {
                self.data[node].values[0][0] = 0;
                self.data[node].values[1][1] = self.r.min((val.abs() + 1) * self.c);
                self.data[node].values[2][2] = self.c * val.abs();
            } else {
                self.data[node].values[0][0] = self.c;
                self.data[node].values[1][1] = self.c;
                self.data[node].values[2][2] = 0;
            }

            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(index, val, node * 2, node_start, mid);
        self.update_internal(index, val, node * 2 + 1, mid + 1, node_end);

        let mut left = self.data[node * 2].clone();
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
            return Node::new(1 << 60);
        }

        if start <= node_start && node_end <= end {
            return self.data[node].clone();
        }

        let mid = (node_start + node_end) / 2;
        let mut left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.merge(&right)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, r, c, t) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    let mut tree = SegmentTree::new(n, r, c);
    let mut arr = vec![0; n + 1];

    for i in 1..=n {
        arr[i] = scan.token::<i64>();
        tree.update(i, arr[i]);
    }

    let calculate = |tree: &mut SegmentTree| {
        let ret = tree.query(1, n);
        let mut ans = 1 << 60;

        for i in 0..3 {
            for j in 0..3 {
                ans = ans.min(ret.values[i][j]);
            }
        }

        ans
    };

    writeln!(out, "{}", calculate(&mut tree)).unwrap();

    for _ in 0..t {
        let (k, v) = (scan.token::<usize>(), scan.token::<i64>());
        tree.update(k, v);

        writeln!(out, "{}", calculate(&mut tree)).unwrap();
    }
}

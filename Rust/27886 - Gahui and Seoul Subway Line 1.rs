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

struct LazySegmentTree {
    size: usize,
    data: Vec<(i64, i64)>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        let mut ret = Self {
            size: n,
            data: vec![(0, 0); real_n * 4],
            lazy: vec![0; real_n * 4],
        };

        ret.init(1, 1, ret.size);

        ret
    }

    fn init(&mut self, node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = (0, start as i64);
            return;
        }

        let mid = (start + end) / 2;
        self.init(node * 2, start, mid);
        self.init(node * 2 + 1, mid + 1, end);

        self.data[node] = self.data[node * 2].min(self.data[node * 2 + 1]);
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if start != end {
            self.lazy[node * 2] += self.lazy[node];
            self.lazy[node * 2 + 1] += self.lazy[node];
        }

        self.data[node].0 += self.lazy[node];
        self.lazy[node] = 0;
    }

    pub fn update(&mut self, start: usize, end: usize, val: i64) {
        self.update_internal(start, end, val, 1, 1, self.size);
    }

    fn update_internal(
        &mut self,
        start: usize,
        end: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return;
        }

        if start <= node_start && node_end <= end {
            self.lazy[node] += val;
            self.propagate(node, node_start, node_end);
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(start, end, val, node * 2, node_start, mid);
        self.update_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = self.data[node * 2].min(self.data[node * 2 + 1]);
    }

    pub fn query(&mut self, start: usize, end: usize) -> (i64, i64) {
        self.query_internal(start, end, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> (i64, i64) {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return (i64::MAX, i64::MAX);
        }

        if start <= node_start && node_end <= end {
            return self.data[node];
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.min(right)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());

    let mut tree = LazySegmentTree::new(n + 1);
    let mut arr = vec![(0, 0, 0); q];

    for i in 0..q {
        arr[i] = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        tree.update(arr[i].0, arr[i].1, 1);
    }

    arr.sort_by(|a, b| a.2.cmp(&b.2));

    let mut ret = vec![1; n + 1];
    let mut idx1 = 0;

    while idx1 < q {
        let mut idx2 = idx1;

        while idx2 < q && arr[idx1].2 == arr[idx2].2 {
            idx2 += 1;
        }

        if arr[idx1].2 != 1 {
            for i in idx1..idx2 {
                tree.update(arr[i].0, arr[i].1, -1);
            }

            for i in idx1..idx2 {
                let val = tree.query(arr[i].0, arr[i].1);

                if val.0 != 0 {
                    writeln!(out, "-1").unwrap();
                    return;
                }

                ret[val.1 as usize] = arr[i].2;
            }

            for i in idx1..idx2 {
                tree.update(arr[i].0, arr[i].1, 1);
            }
        }

        idx1 = idx2;
    }

    for val in ret[1..].iter() {
        write!(out, "{} ", val).unwrap();
    }

    writeln!(out).unwrap();
}

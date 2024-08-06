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
    root: usize,
    data: Vec<i64>,
    lazy: Vec<bool>,
}

impl LazySegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            root: real_n,
            data: vec![0; real_n * 2],
            lazy: vec![false; real_n * 2],
        }
    }

    pub fn construct(&mut self, arr: &[i64]) {
        for i in 0..arr.len() {
            self.data[i + self.root] = arr[i];
        }

        for i in (1..=self.root - 1).rev() {
            self.data[i] = self.data[i * 2] ^ self.data[i * 2 + 1];
        }
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] {
            if node < self.root {
                self.lazy[node * 2] = !self.lazy[node * 2];
                self.lazy[node * 2 + 1] = !self.lazy[node * 2 + 1];

                let mut temp = 0;

                if self.lazy[node * 2] {
                    temp += (end - start) as i64 / 2 - self.data[node * 2];
                } else {
                    temp += self.data[node * 2];
                }

                if self.lazy[node * 2 + 1] {
                    temp += (end - start) as i64 / 2 - self.data[node * 2 + 1];
                } else {
                    temp += self.data[node * 2 + 1];
                }

                self.data[node] = temp;
            } else {
                self.data[node] ^= 1;
            }

            self.lazy[node] = false;
        }
    }

    pub fn update(&mut self, start: usize, end: usize) {
        self.update_internal(start, end, 1, 0, self.root);
    }

    fn update_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        self.propagate(node, node_start, node_end);

        if end <= node_start || node_end <= start {
            return;
        }

        if start <= node_start && node_end <= end {
            self.lazy[node] = !self.lazy[node];
            self.propagate(node, node_start, node_end);
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(start, end, node * 2, node_start, mid);
        self.update_internal(start, end, node * 2 + 1, mid, node_end);

        self.data[node] = self.data[node * 2] + self.data[node * 2 + 1];
    }

    pub fn query(&mut self, start: usize, end: usize) -> i64 {
        self.query_internal(start, end, 1, 0, self.root)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> i64 {
        self.propagate(node, node_start, node_end);

        if end <= node_start || node_end <= start {
            return 0;
        }

        if start <= node_start && node_end <= end {
            return self.data[node];
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid, node_end);

        left + right
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    let mut tree = LazySegmentTree::new(n);
    let arr = vec![0; n];

    tree.construct(&arr);

    for _ in 0..m {
        let (o, s, t) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if o == 0 {
            tree.update(s, t + 1);
        } else {
            writeln!(out, "{}", tree.query(s, t + 1)).unwrap();
        }
    }
}

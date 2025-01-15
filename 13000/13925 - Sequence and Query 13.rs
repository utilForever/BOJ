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

const MOD: i64 = 1_000_000_007;

#[derive(Debug, Clone)]
struct LazyInfo {
    flag_set: bool,
    val_set: i64,
    val_add: i64,
    val_mul: i64,
}

impl LazyInfo {
    fn new() -> Self {
        Self {
            flag_set: false,
            val_set: 0,
            val_add: 0,
            val_mul: 1,
        }
    }
}

struct LazySegmentTree {
    size: usize,
    data: Vec<i64>,
    lazy: Vec<LazyInfo>,
}

impl LazySegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;

        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![0; real_n * 4],
            lazy: vec![LazyInfo::new(); real_n * 4],
        }
    }

    pub fn construct(&mut self, arr: &[i64], start: usize, end: usize) {
        self.construct_internal(arr, 1, start, end);
    }

    fn construct_internal(&mut self, arr: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = arr[start];
        } else {
            let mid = (start + end) / 2;

            self.construct_internal(arr, node * 2, start, mid);
            self.construct_internal(arr, node * 2 + 1, mid + 1, end);
            self.data[node] = (self.data[node * 2] + self.data[node * 2 + 1]) % MOD;
        }
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        let mid = (start + end) / 2;
        let left = node * 2;
        let right = node * 2 + 1;

        if self.lazy[node].flag_set {
            self.propagate_set(left, start, mid, self.lazy[node].val_set);
            self.propagate_set(right, mid + 1, end, self.lazy[node].val_set);
            self.lazy[node].flag_set = false;
        }

        let val_mul = self.lazy[node].val_mul;

        if val_mul != 1 {
            self.propagate_mul(left, start, mid, val_mul);
            self.propagate_mul(right, mid + 1, end, val_mul);
            self.lazy[node].val_mul = 1;
        }

        let val_add = self.lazy[node].val_add;

        if val_add != 0 {
            self.propagate_add(left, start, mid, val_add);
            self.propagate_add(right, mid + 1, end, val_add);
            self.lazy[node].val_add = 0;
        }
    }

    fn propagate_set(&mut self, idx: usize, start: usize, end: usize, val: i64) {
        let len = end - start + 1;

        self.data[idx] = (val * len as i64) % MOD;
        self.lazy[idx].flag_set = true;
        self.lazy[idx].val_set = val;
        self.lazy[idx].val_add = 0;
        self.lazy[idx].val_mul = 1;
    }

    fn propagate_add(&mut self, idx: usize, start: usize, end: usize, val: i64) {
        let len = end - start + 1;

        if self.lazy[idx].flag_set {
            self.lazy[idx].val_set = (self.lazy[idx].val_set + val) % MOD;
        } else {
            self.lazy[idx].val_add = (self.lazy[idx].val_add + val) % MOD;
        }

        self.data[idx] = (self.data[idx] + val * len as i64 % MOD) % MOD;
    }

    fn propagate_mul(&mut self, idx: usize, _start: usize, _end: usize, val: i64) {
        if self.lazy[idx].flag_set {
            self.lazy[idx].val_set = (self.lazy[idx].val_set * val) % MOD;
        } else {
            self.lazy[idx].val_add = (self.lazy[idx].val_add * val) % MOD;
            self.lazy[idx].val_mul = (self.lazy[idx].val_mul * val) % MOD;
        }

        self.data[idx] = (self.data[idx] * val) % MOD;
    }

    pub fn update(&mut self, start: usize, end: usize, command: i64, val: i64) {
        self.update_internal(start, end, command, val, 1, 1, self.size);
    }

    fn update_internal(
        &mut self,
        start: usize,
        end: usize,
        command: i64,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if end < node_start || node_end < start {
            return;
        }

        if start <= node_start && node_end <= end {
            match command {
                1 => self.propagate_add(node, node_start, node_end, val),
                2 => self.propagate_mul(node, node_start, node_end, val),
                3 => self.propagate_set(node, node_start, node_end, val),
                _ => {}
            }

            return;
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;
        self.update_internal(start, end, command, val, node * 2, node_start, mid);
        self.update_internal(start, end, command, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = (self.data[node * 2] + self.data[node * 2 + 1]) % MOD;
    }

    pub fn query(&mut self, start: usize, end: usize) -> i64 {
        self.query_internal(start, end, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> i64 {
        if end < node_start || node_end < start {
            return 0;
        }

        if start <= node_start && node_end <= end {
            return self.data[node] % MOD;
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        (left + right) % MOD
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut tree = LazySegmentTree::new(n);
    let mut arr = vec![0; n + 1];

    for i in 1..=n {
        arr[i] = scan.token::<i64>();
    }

    tree.construct(&arr, 1, n);

    let m = scan.token::<usize>();

    for _ in 0..m {
        let command = scan.token::<i64>();

        if command >= 1 && command <= 3 {
            let (x, y, v) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            tree.update(x, y, command, v);
        } else {
            let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(out, "{}", tree.query(x, y)).unwrap();
        }
    }
}

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

const Y_MAX: i64 = 1_000_000_000_000_000_000;
const NEG_INF: i64 = i64::MIN / 4;

struct LazySegmentTree {
    size: usize,
    max: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;

        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            max: vec![NEG_INF; real_n * 4],
            lazy: vec![0; real_n * 4],
        }
    }

    fn init(&mut self, arr: &Vec<i64>) {
        self.init_internal(arr, 1, 1, self.size);
    }

    fn init_internal(&mut self, arr: &Vec<i64>, node: usize, start: usize, end: usize) {
        if start == end {
            self.max[node] = arr[start - 1];
            return;
        }

        let mid = (start + end) / 2;
        
        self.init_internal(arr, node * 2, start, mid);
        self.init_internal(arr, node * 2 + 1, mid + 1, end);

        self.max[node] = self.max[node * 2].max(self.max[node * 2 + 1]);
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if start != end {
            self.lazy[node * 2] += self.lazy[node];
            self.lazy[node * 2 + 1] += self.lazy[node];
        }

        self.max[node] += self.lazy[node];
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

        self.max[node] = self.max[node * 2].max(self.max[node * 2 + 1]);
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
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return NEG_INF;
        }

        if start <= node_start && node_end <= end {
            return self.max[node];
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        left.max(right)
    }
}

fn calculate(
    cnt: &Vec<i64>,
    order: &Vec<(usize, usize)>,
    dp_prev: &Vec<i64>,
    m: usize,
) -> Vec<i64> {
    let mut tree = LazySegmentTree::new(dp_prev.len());
    tree.init(dp_prev);

    let mut dp = vec![NEG_INF; m];
    let mut idx = 0;

    for i in (0..m).rev() {
        while idx < order.len() && order[idx].1 >= i {
            let l = order[idx].0;

            tree.update(l + 1, m, -1);
            idx += 1;
        }

        if i == 0 {
            continue;
        }

        let val = tree.query(1, i);

        if val < NEG_INF / 2 {
            continue;
        }

        dp[i] = cnt[i] + val;
    }

    dp
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut intervals = vec![(0, 0); n];
    let mut cnt_unavoidable = 0;

    for i in 0..n {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        let mut l = y - x + 1;
        let mut r = y + x - 1;

        l = l.max(1);
        r = r.min(Y_MAX);

        if l <= r {
            intervals[i] = (l, r);
        } else {
            cnt_unavoidable += 1;
        }
    }

    if intervals.is_empty() {
        writeln!(out, "{cnt_unavoidable}").unwrap();
        return;
    }

    let mut compressed = Vec::with_capacity(intervals.len() * 2 + 2);

    compressed.push(1);
    compressed.push(Y_MAX);

    for &(l, r) in intervals.iter() {
        compressed.push(l);
        compressed.push(r);
    }

    compressed.sort_unstable();
    compressed.dedup();

    let m = compressed.len();

    let mut diff = vec![0; m + 1];
    let mut intervals_idx = Vec::with_capacity(intervals.len());

    for &(l, r) in intervals.iter() {
        let idx_l = compressed.binary_search(&l).unwrap();
        let idx_r = compressed.binary_search(&r).unwrap();

        intervals_idx.push((idx_l, idx_r));

        diff[idx_l] += 1;

        if idx_r + 1 < m {
            diff[idx_r + 1] -= 1;
        }
    }

    let mut cnt = vec![0; m];
    let mut acc = 0;

    for i in 0..m {
        acc += diff[i];
        cnt[i] = acc;
    }

    let mut order = intervals_idx.clone();
    order.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let dp1 = cnt.clone();
    let dp2 = calculate(&cnt, &order, &dp1, m);
    let dp3 = calculate(&cnt, &order, &dp2, m);

    let max1 = *dp1.iter().max().unwrap_or(&NEG_INF);
    let max2 = *dp2.iter().max().unwrap_or(&NEG_INF);
    let max3 = *dp3.iter().max().unwrap_or(&NEG_INF);

    let cnt_coverable = intervals.len() as i64;
    let ret = cnt_unavoidable + (cnt_coverable - max1.max(max2).max(max3).max(0));

    writeln!(out, "{ret}").unwrap();
}

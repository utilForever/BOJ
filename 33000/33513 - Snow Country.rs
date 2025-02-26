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

struct FenwickTree {
    n: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            n,
            data: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.data[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut ret = 0;

        while idx > 0 {
            ret += self.data[idx];
            idx -= idx & (!idx + 1);
        }

        ret
    }

    fn query_range(&self, start: usize, end: usize) -> i64 {
        self.query(end) - if start > 1 { self.query(start - 1) } else { 0 }
    }
}

#[derive(Clone, Copy)]
struct Node {
    min: i64,
    max: i64,
    add: i64,
}

struct SegmentTree {
    n: usize,
    data: Vec<Node>,
}

impl SegmentTree {
    fn new(nums: &Vec<i64>) -> Self {
        let n = nums.len();
        let data = vec![
            Node {
                min: 0,
                max: 0,
                add: 0
            };
            4 * n
        ];

        let mut tree = SegmentTree { n, data };
        tree.build(nums, 1, 0, n - 1);

        tree
    }

    fn build(&mut self, nums: &Vec<i64>, node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = Node {
                min: nums[start],
                max: nums[start],
                add: 0,
            };
        } else {
            let mid = (start + end) / 2;

            self.build(nums, node * 2, start, mid);
            self.build(nums, node * 2 + 1, mid + 1, end);
            self.merge(node);
        }
    }

    fn merge(&mut self, node: usize) {
        self.data[node].min = self.data[node * 2].min.min(self.data[node * 2 + 1].min);
        self.data[node].max = self.data[node * 2].max.max(self.data[node * 2 + 1].max);
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if self.data[node].add != 0 && start < end {
            self.data[node * 2].min += self.data[node].add;
            self.data[node * 2].max += self.data[node].add;
            self.data[node * 2].add += self.data[node].add;

            self.data[node * 2 + 1].min += self.data[node].add;
            self.data[node * 2 + 1].max += self.data[node].add;
            self.data[node * 2 + 1].add += self.data[node].add;

            self.data[node].add = 0;
        }
    }

    fn update(&mut self, start: usize, end: usize, delta: i64) {
        self.update_internal(start, end, delta, 1, 0, self.n - 1);
    }

    fn update_internal(
        &mut self,
        start: usize,
        end: usize,
        delta: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if end < node_start || node_end < start {
            return;
        }

        if start <= node_start && node_end <= end {
            self.data[node].min += delta;
            self.data[node].max += delta;
            self.data[node].add += delta;
            return;
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;
        self.update_internal(start, end, delta, node * 2, node_start, mid);
        self.update_internal(start, end, delta, node * 2 + 1, mid + 1, node_end);

        self.merge(node);
    }

    fn query(&mut self, start: usize, end: usize) -> (i64, i64) {
        self.query_internal(start, end, 1, 0, self.n - 1)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> (i64, i64) {
        if end < node_start || node_end < start {
            return (i64::MAX, i64::MIN);
        }

        if start <= node_start && node_end <= end {
            return (self.data[node].min, self.data[node].max);
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;
        let (min_left, max_left) = self.query_internal(start, end, node * 2, node_start, mid);
        let (min_right, max_right) =
            self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        (min_left.min(min_right), max_left.max(max_right))
    }

    fn query_point(&mut self, pos: usize) -> i64 {
        self.query_point_internal(pos, 1, 0, self.n - 1)
    }

    fn query_point_internal(
        &mut self,
        pos: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> i64 {
        if node_start == node_end {
            return self.data[node].min;
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;

        if pos <= mid {
            self.query_point_internal(pos, node * 2, node_start, mid)
        } else {
            self.query_point_internal(pos, node * 2 + 1, mid + 1, node_end)
        }
    }
}

fn query_segment_tree(
    seg: &mut Option<SegmentTree>,
    positions: &Vec<usize>,
    start: usize,
    end: usize,
) -> Option<(i64, i64)> {
    if let Some(tree) = seg.as_mut() {
        let lower_bound = positions.binary_search(&start).unwrap_or_else(|x| x);
        let upper_bound = positions.binary_search(&end).unwrap_or_else(|x| x);

        if lower_bound < upper_bound {
            Some(tree.query(lower_bound, upper_bound - 1))
        } else {
            None
        }
    } else {
        None
    }
}

fn update_segment_tree(
    seg: &mut Option<SegmentTree>,
    positions: &Vec<usize>,
    node: usize,
    delta: i64,
) {
    if let Some(tree) = seg.as_mut() {
        let pos = positions.binary_search(&node).unwrap_or_else(|x| x);

        if pos < positions.len() {
            tree.update(pos, positions.len() - 1, delta);
        }
    }
}

fn process_query(
    snows: &Vec<i64>,
    seg_even: &mut Option<SegmentTree>,
    seg_odd: &mut Option<SegmentTree>,
    pos_even: &Vec<usize>,
    pos_odd: &Vec<usize>,
    left: usize,
    right: usize,
    total: i64,
    base: i64,
    prefix_sum_r: i64,
) -> Option<i64> {
    let len = right - left + 1;
    let x_chosen;

    if left % 2 == 0 {
        let x_same = if let Some((min, _)) = query_segment_tree(seg_even, pos_even, left, right) {
            min - base
        } else {
            i64::MAX
        };

        if let Some((_, max)) = query_segment_tree(seg_odd, pos_odd, left, right) {
            if max > base {
                return None;
            }
        }

        if len % 2 == 0 {
            if (right - 1) % 2 != 0 {
                return None;
            }

            let pos = pos_even.binary_search(&(right - 1)).ok()?;
            let val = seg_even.as_mut()?.query_point(pos) - base;

            if val != snows[right] {
                return None;
            }

            x_chosen = x_same;
        } else {
            let x_forced = snows[right] + (prefix_sum_r - base);

            if x_forced > x_same {
                return None;
            }

            x_chosen = x_forced;
        }
    } else {
        let x_same = if let Some((_, max)) = query_segment_tree(seg_odd, pos_odd, left, right) {
            base - max
        } else {
            i64::MAX
        };

        if let Some((min, _)) = query_segment_tree(seg_even, pos_even, left, right) {
            if min < base {
                return None;
            }
        }

        if len % 2 == 0 {
            if (right - 1) % 2 != 1 {
                return None;
            }

            let pos = pos_odd.binary_search(&(right - 1)).ok()?;
            let val = base - seg_odd.as_mut()?.query_point(pos);

            if val != snows[right] {
                return None;
            }

            x_chosen = x_same;
        } else {
            let x_forced = snows[right] + (base - prefix_sum_r);

            if x_forced > x_same {
                return None;
            }

            x_chosen = x_forced;
        }
    }

    if x_chosen < 0 {
        return None;
    }

    let diff = total - (len as i64) * x_chosen;

    if diff < 0 || diff % 2 != 0 {
        return None;
    }

    Some(diff / 2)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut snows = vec![0; n + 1];

    for i in 1..=n {
        snows[i] = scan.token::<i64>();
    }

    let mut prefix_sum = vec![0; n + 1];

    for i in 1..=n {
        let sign = if i % 2 == 0 { 1 } else { -1 };
        prefix_sum[i] = prefix_sum[i - 1] + sign * snows[i];
    }

    let mut fenwick_a = FenwickTree::new(n);
    let mut fenwick_b = FenwickTree::new(n);

    for i in 1..=n {
        let sign = if i % 2 == 0 { 1 } else { -1 };
        fenwick_a.update(i, snows[i]);
        fenwick_b.update(i, sign * snows[i]);
    }

    let mut pos_even = Vec::new();
    let mut pos_odd = Vec::new();

    for i in 1..=n {
        if i % 2 == 0 {
            pos_even.push(i);
        } else {
            pos_odd.push(i);
        }
    }

    let vals_even = pos_even
        .iter()
        .map(|&idx| prefix_sum[idx])
        .collect::<Vec<_>>();
    let vals_odd = pos_odd
        .iter()
        .map(|&idx| prefix_sum[idx])
        .collect::<Vec<_>>();

    let mut seg_even = if !vals_even.is_empty() {
        Some(SegmentTree::new(&vals_even))
    } else {
        None
    };
    let mut seg_odd = if !vals_odd.is_empty() {
        Some(SegmentTree::new(&vals_odd))
    } else {
        None
    };

    let q = scan.token::<i64>();

    for _ in 0..q {
        let command = scan.token::<i32>();

        if command == 1 {
            let (i, v) = (scan.token::<usize>(), scan.token::<i64>());
            let snow_old = snows[i];
            let diff = v - snow_old;

            snows[i] = v;

            fenwick_a.update(i, diff);
            let sign = if i % 2 == 0 { 1 } else { -1 };
            let delta = sign * diff;
            fenwick_b.update(i, delta);

            update_segment_tree(&mut seg_even, &pos_even, i, delta);
            update_segment_tree(&mut seg_odd, &pos_odd, i, delta);
        } else {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            let total = fenwick_a.query_range(l, r);
            let base = if l == 1 { 0 } else { fenwick_b.query(l - 1) };
            let prefix_sum_r = fenwick_b.query(r - 1);

            if let Some(ret) = process_query(
                &snows,
                &mut seg_even,
                &mut seg_odd,
                &pos_even,
                &pos_odd,
                l,
                r,
                total,
                base,
                prefix_sum_r,
            ) {
                writeln!(out, "{ret}").unwrap();
            } else {
                writeln!(out, "-1").unwrap();
            }
        }
    }
}

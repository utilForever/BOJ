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
    val: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self { val }
    }

    fn merge(&self, other: &Self) -> Node {
        Node {
            val: self.val.min(other.val),
        }
    }
}

const INF: i64 = i64::MAX / 4;

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
            data: vec![Node::new(INF); real_n * 4],
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

    fn query(&mut self, start: usize, end: usize, threshold: i64) -> Option<usize> {
        self.query_internal(start, end, threshold, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        threshold: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> Option<usize> {
        if end < node_start || node_end < start {
            return None;
        }

        if self.data[node].val >= threshold {
            return None;
        }

        if node_start == node_end {
            return Some(node_start);
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, threshold, node * 2, node_start, mid);

        if left.is_some() {
            return left;
        }

        self.query_internal(start, end, threshold, node * 2 + 1, mid + 1, node_end)
    }
}

#[derive(Clone, Debug)]
struct Segment {
    l: usize,
    r: usize,
    size: i64,
}

impl Segment {
    fn new(l: usize, r: usize, size: i64) -> Self {
        Self { l, r, size }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut segments = Vec::with_capacity(n);
    let mut prefix_sum = Vec::with_capacity(n + 1);
    let mut prefix_cnt = Vec::with_capacity(n + 1);

    prefix_sum.push(0);
    prefix_cnt.push(0);

    let mut tree = SegmentTree::new(n + 1);
    tree.update(1, 0);

    let mut ret = Vec::new();

    for i in 1..=n {
        let (t, s) = (scan.token::<i64>(), scan.token::<i64>());
        let w = match t {
            -1 => s,
            0 => 0,
            1 => -s,
            _ => unreachable!(),
        };

        segments.push(Segment::new(i, i, s));
        prefix_sum.push(prefix_sum.last().unwrap() + w);
        prefix_cnt.push(prefix_cnt.last().unwrap() + if w <= 0 { 1 } else { 0 });
        tree.update(segments.len() + 1, *prefix_sum.last().unwrap());

        loop {
            let m = segments.len();

            if m == 0 {
                break;
            }

            let k = match tree.query(1, m, prefix_sum[m]) {
                Some(idx) => idx - 1,
                None => break,
            };

            if prefix_cnt[m] - prefix_cnt[k] == 0 {
                break;
            }

            let l = segments[k].l;
            let r = segments[m - 1].r;

            ret.push((l, r));

            let mut sum_size = 0;

            while segments.len() > k {
                let segment = segments.pop().unwrap();

                sum_size += segment.size;
                prefix_sum.pop();
                prefix_cnt.pop();
            }

            segments.push(Segment::new(l, r, sum_size));

            let sum_new = *prefix_sum.last().unwrap() + sum_size;
            let cnt_new = *prefix_cnt.last().unwrap();

            prefix_sum.push(sum_new);
            prefix_cnt.push(cnt_new);
            tree.update(segments.len() + 1, sum_new);
        }
    }

    if *prefix_cnt.last().unwrap() != 0 {
        writeln!(out, "-1").unwrap();
        return;
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for (l, r) in ret {
        writeln!(out, "{l} {r}").unwrap();
    }
}

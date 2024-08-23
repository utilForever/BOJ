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

#[derive(Clone, Default)]
struct Node {
    sum: i64,
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
            data: vec![Node::default(); real_n * 4],
        }
    }

    fn merge(a: &Node, b: &Node) -> Node {
        Node {
            sum: a.sum.min(b.sum),
        }
    }

    pub fn construct(&mut self, arr: &[i64], start: usize, end: usize) {
        self.construct_internal(arr, 1, start, end);
    }

    fn construct_internal(&mut self, arr: &[i64], node: usize, start: usize, end: usize) -> Node {
        if start == end {
            self.data[node] = Node { sum: arr[start] };
            self.data[node].clone()
        } else {
            let mid = (start + end) / 2;

            let left = self.construct_internal(arr, node * 2, start, mid);
            let right = self.construct_internal(arr, node * 2 + 1, mid + 1, end);

            self.data[node] = SegmentTree::merge(&left, &right);
            self.data[node].clone()
        }
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
        if end < node_start || node_end < start {
            return Node { sum: i64::MAX };
        }

        if start <= node_start && node_end <= end {
            return self.data[node].clone();
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        Node {
            sum: left.sum.min(right.sum),
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut nums = vec![0; n + 1];

    // nums[i]: The number of '(' - the number of ')' in [1..=i]
    for i in 1..=n {
        nums[i] = nums[i - 1] + if s[i - 1] == '(' { 1 } else { -1 };
    }

    let mut tree = SegmentTree::new(n);
    tree.construct(&nums, 1, n);

    let m = scan.token::<i64>();
    let mut ret = 0;

    for _ in 0..m {
        let (i, j) = (scan.token::<usize>(), scan.token::<usize>());

        // nums[j] == nums[i - 1]: The number of '(' - the number of ')' in [i..=j] is 0
        // tree.query(i, j).sum >= nums[i - 1]: The number of '(' - the number of ')' in [i..=j] is non-negative
        // it means the number of '(' is greater than or equal to the number of ')' in [i..=i], [i..=i+1], ..., [i..=j]
        if nums[j] == nums[i - 1] && tree.query(i, j).sum >= nums[i - 1] {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}

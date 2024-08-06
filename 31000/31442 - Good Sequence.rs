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
    sum: i64,
    min: i64,
    max: i64,
    left: i64,
    left_cnt: i64,
    right: i64,
    right_cnt: i64,
    valid_zero: bool,
    valid_one: bool,
}

impl Node {
    fn new(val: i64) -> Self {
        let val_converted = if val == 1 { 1 } else { -1 };

        Self {
            sum: val_converted,
            min: val_converted,
            max: val_converted,
            left: val,
            left_cnt: 1,
            right: val,
            right_cnt: 1,
            valid_zero: false,
            valid_one: false,
        }
    }

    fn merge(&mut self, other: &Self, start: usize, mid: usize, end: usize) -> Node {
        let mut ret = Node::new(0);

        ret.sum = self.sum + other.sum;
        ret.min = self.min.min(self.sum + other.min);
        ret.max = self.max.max(self.sum + other.max);

        ret.valid_zero = if (self.min <= self.sum + other.min && self.valid_zero)
            || (self.min >= self.sum + other.min && other.valid_zero)
        {
            true
        } else {
            false
        };
        ret.valid_one = if (self.max >= self.sum + other.max && self.valid_one)
            || (self.max <= self.sum + other.max && other.valid_one)
        {
            true
        } else {
            false
        };

        ret.left = self.left;

        if self.left_cnt == (mid - start + 1) as i64 && self.left == other.left {
            ret.left_cnt = self.left_cnt + other.left_cnt;
        } else {
            ret.left_cnt = self.left_cnt;

            let length = if self.right == other.left {
                self.right_cnt + other.left_cnt
            } else {
                other.left_cnt
            };

            if length % 2 == 0 {
                if other.left == 1 && self.sum + other.left_cnt == ret.max {
                    ret.valid_one = true;
                } else if other.left == 0 && self.sum - other.left_cnt == ret.min {
                    ret.valid_zero = true;
                }
            }
        }

        ret.right = other.right;

        if other.right_cnt == (end - mid) as i64 && self.right == other.right {
            ret.right_cnt = self.right_cnt + other.right_cnt;
        } else {
            ret.right_cnt = other.right_cnt;
        }

        ret
    }

    fn flip(&self) -> Node {
        let mut ret = Node::new(0);

        ret.sum = -self.sum;
        ret.min = -self.max;
        ret.max = -self.min;

        ret.left = self.left ^ 1;
        ret.left_cnt = self.left_cnt;
        ret.right = self.right ^ 1;
        ret.right_cnt = self.right_cnt;

        ret.valid_zero = self.valid_one;
        ret.valid_one = self.valid_zero;

        ret
    }
}

struct LazySegmentTree {
    size: usize,
    data: Vec<Node>,
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
            data: vec![Node::new(0); real_n * 4],
            lazy: vec![0; real_n * 4],
        }
    }

    pub fn construct(&mut self, arr: &[i64], start: usize, end: usize) {
        self.construct_internal(arr, 1, start, end);
    }

    fn construct_internal(&mut self, arr: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = Node::new(arr[start]);
            return;
        }

        let mid = (start + end) / 2;

        self.construct_internal(arr, node * 2, start, mid);
        self.construct_internal(arr, node * 2 + 1, mid + 1, end);

        let mut left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();

        self.data[node] = left.merge(&right, start, mid, end);
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] != 0 {
            self.data[node] = self.data[node].flip();

            if start != end {
                self.lazy[node * 2] ^= 1;
                self.lazy[node * 2 + 1] ^= 1;
            }

            self.lazy[node] = 0;
        }
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
            self.lazy[node] = 1;
            self.propagate(node, node_start, node_end);
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(start, end, val, node * 2, node_start, mid);
        self.update_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        let mut left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();

        self.data[node] = left.merge(&right, node_start, mid, node_end);
    }

    fn query(&mut self) -> bool {
        self.data[1].left == 1 && !self.data[1].valid_zero && self.data[1].min == 0
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; 2 * n + 1];

    for i in 1..=2 * n {
        nums[i] = scan.token::<i64>();
    }

    let mut nums_for_tree = vec![0; 2 * n - 1];

    for i in 2..=2 * n - 1 {
        nums_for_tree[i - 1] = nums[i];
    }

    let mut tree = LazySegmentTree::new(2 * n - 2);
    tree.construct(&nums_for_tree, 1, 2 * n - 2);

    writeln!(
        out,
        "{}",
        if nums[1] == 1 && nums[2 * n] == 0 && tree.query() {
            "YES"
        } else {
            "NO"
        }
    )
    .unwrap();

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());

        if l == 1 {
            nums[1] ^= 1;
        }

        if r == 2 * n {
            nums[2 * n] ^= 1;
        }

        tree.update(l - 1, r - 1, 1);

        writeln!(
            out,
            "{}",
            if nums[1] == 1 && nums[2 * n] == 0 && tree.query() {
                "YES"
            } else {
                "NO"
            }
        )
        .unwrap();
    }
}

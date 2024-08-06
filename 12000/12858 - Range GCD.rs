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

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        let val = max;

        max = min;
        min = val;
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

struct LazySegmentTree {
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; (n + 1) * 4],
            lazy: vec![0; (n + 1) * 4],
        }
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] == 0 {
            return;
        }

        self.tree[node] += (end - start + 1) as i64 * self.lazy[node];

        if start != end {
            self.lazy[node * 2] += self.lazy[node];
            self.lazy[node * 2 + 1] += self.lazy[node];
        }

        self.lazy[node] = 0;
    }

    fn update(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
        val: i64,
    ) {
        self.propagate(node, start, end);

        if start > right || end < left {
            return;
        }

        if start >= left && end <= right {
            self.tree[node] += (end - start + 1) as i64 * val;

            if start != end {
                self.lazy[node * 2] += val;
                self.lazy[node * 2 + 1] += val;
            }

            return;
        }

        let mid = (start + end) / 2;

        self.update(node * 2, start, mid, left, right, val);
        self.update(node * 2 + 1, mid + 1, end, left, right, val);

        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }

    fn query(&mut self, node: usize, start: usize, end: usize, val: usize) -> i64 {
        self.propagate(node, start, end);

        if start > val || end < val {
            return 0;
        }

        if start == end {
            return self.tree[node];
        }

        let mid = (start + end) / 2;

        return self.query(node * 2, start, mid, val) + self.query(node * 2 + 1, mid + 1, end, val);
    }
}

struct SegmentTree {
    tree: Vec<i64>,
    bias: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; (n + 1) * 4],
            bias: {
                let mut bias = 1;

                while bias < (n + 1) {
                    bias *= 2;
                }

                bias
            },
        }
    }

    fn update(&mut self, idx: usize, val: i64) {
        let mut idx = idx + self.bias;
        self.tree[idx] = val;

        idx /= 2;

        while idx != 0 {
            self.tree[idx] = gcd(self.tree[idx * 2], self.tree[(idx * 2) | 1]);
            idx /= 2;
        }
    }

    fn query(&mut self, left: usize, right: usize) -> i64 {
        let mut left = left + self.bias;
        let mut right = right + self.bias;
        let mut ret = 0;

        while left < right {
            if (left & 1) != 0 {
                ret = gcd(ret, self.tree[left]);
                left += 1;
            }

            if (!right & 1) != 0 {
                ret = gcd(ret, self.tree[right]);
                right -= 1;
            }

            left /= 2;
            right /= 2;
        }

        if left == right {
            ret = gcd(ret, self.tree[left]);
        }

        ret
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    let mut lazy_segment_tree = LazySegmentTree::new(100_000);
    let mut segment_tree = SegmentTree::new(100_000);

    for i in 1..=n {
        let elem = scan.token::<i64>();
        lazy_segment_tree.update(1, 1, 100_000, i, i, elem);
    }

    for i in 1..n {
        let now = lazy_segment_tree.query(1, 1, 100_000, i)
            - lazy_segment_tree.query(1, 1, 100_000, i + 1);
        segment_tree.update(i, now.abs());
    }

    let q = scan.token::<usize>();

    for _ in 0..q {
        let (t, a, b) = (
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if t == 0 {
            let mut now = segment_tree.query(a, b - 1);
            now = gcd(now, lazy_segment_tree.query(1, 1, 100_000, a));

            writeln!(out, "{}", now).unwrap();
        } else {
            lazy_segment_tree.update(1, 1, 100_000, a, b, t);

            let now = lazy_segment_tree.query(1, 1, 100_000, a - 1)
                - lazy_segment_tree.query(1, 1, 100_000, a);
            segment_tree.update(a - 1, now.abs());

            let now = lazy_segment_tree.query(1, 1, 100_000, b)
                - lazy_segment_tree.query(1, 1, 100_000, b + 1);
            segment_tree.update(b, now.abs());
        }
    }
}

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

struct PermutationTree {
    size: usize,
    val: Vec<i64>,         // Values stored in the segment tree nodes
    data: Vec<(i64, i64)>, // (Minimum value, count of min values) in the range
    lazy: Vec<(i64, i64)>, // Lazy updates: (increment for min values, increment for counts)
}

impl PermutationTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            val: vec![0; real_n * 4],
            data: vec![(0, 0); real_n * 4],
            lazy: vec![(0, 0); real_n * 4],
        }
    }

    fn merge(left: &(i64, i64), right: &(i64, i64)) -> (i64, i64) {
        if left.0 < right.0 {
            (left.0, left.1)
        } else if left.0 > right.0 {
            (right.0, right.1)
        } else {
            (left.0, left.1 + right.1)
        }
    }

    pub fn construct(&mut self, start: usize, end: usize) {
        self.construct_internal(1, start, end);
    }

    fn construct_internal(&mut self, node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = (start as i64, 1);
            return;
        } else {
            let mid = (start + end) / 2;

            self.construct_internal(node * 2, start, mid);
            self.construct_internal(node * 2 + 1, mid + 1, end);

            self.data[node] =
                PermutationTree::merge(&self.data[node * 2], &self.data[node * 2 + 1]);
        }
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node].0 == 0 && self.lazy[node].1 == 0 {
            return;
        }

        if self.lazy[node].0 != 0 {
            // Update the min values
            self.data[node * 2].0 += self.lazy[node].0;
            self.data[node * 2 + 1].0 += self.lazy[node].0;

            if start != end {
                // Propagate the lazy updates to children
                self.lazy[node * 2].0 += self.lazy[node].0;
                self.lazy[node * 2 + 1].0 += self.lazy[node].0;
            }
        }

        if self.lazy[node].1 != 0 {
            // Update the counts if the min value matches
            if self.data[node * 2].0 == self.data[node].0 {
                self.val[node * 2] += self.data[node * 2].1 * self.lazy[node].1;

                if start != end {
                    self.lazy[node * 2].1 += self.lazy[node].1;
                }
            }

            if self.data[node * 2 + 1].0 == self.data[node].0 {
                self.val[node * 2 + 1] += self.data[node * 2 + 1].1 * self.lazy[node].1;

                if start != end {
                    self.lazy[node * 2 + 1].1 += self.lazy[node].1;
                }
            }
        }

        self.lazy[node].0 = 0;
        self.lazy[node].1 = 0;
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
        if start == node_start && node_end == end {
            self.data[node].0 += val;
            self.lazy[node].0 += val;
            return;
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;

        if end <= mid {
            self.update_internal(start, end, val, node * 2, node_start, mid);
        } else if start > mid {
            self.update_internal(start, end, val, node * 2 + 1, mid + 1, node_end);
        } else {
            self.update_internal(start, mid, val, node * 2, node_start, mid);
            self.update_internal(mid + 1, end, val, node * 2 + 1, mid + 1, node_end);
        }

        self.data[node] = PermutationTree::merge(&self.data[node * 2], &self.data[node * 2 + 1]);
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
        if start == node_start && node_end == end {
            return self.val[node];
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;

        if end <= mid {
            self.query_internal(start, end, node * 2, node_start, mid)
        } else if start > mid {
            self.query_internal(start, end, node * 2 + 1, mid + 1, node_end)
        } else {
            let left = self.query_internal(start, mid, node * 2, node_start, mid);
            let right = self.query_internal(mid + 1, end, node * 2 + 1, mid + 1, node_end);

            left + right
        }
    }
}

// Reference: https://codeforces.com/blog/entry/78898
// Reference: https://codeforces.com/blog/entry/60357
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n + 1];

    for i in 1..=n {
        nums[i] = scan.token::<i64>();
    }

    let q = scan.token::<usize>();
    let mut queries = vec![Vec::new(); n + 1];

    for i in 1..=q {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
        queries[r].push((l, i));
    }

    let mut tree = PermutationTree::new(n);
    tree.construct(1, n);

    let mut max = Vec::new(); // Stack to maintain decreasing sequence for max values
    let mut min = Vec::new(); // Stack to maintain increasing sequence for min values
    let mut ret = vec![0; q + 1];

    for i in 1..=n {
        // Update the tree for maximum values
        while !max.is_empty() && nums[i] > nums[*max.last().unwrap()] {
            let val = max.pop().unwrap();

            // Update the range from the last position to the current position
            tree.update(
                if max.is_empty() {
                    1
                } else {
                    *max.last().unwrap() + 1
                },
                val,
                nums[i] - nums[val],
            );
        }

        max.push(i);

        // Update the tree for minimum values
        while !min.is_empty() && nums[i] < nums[*min.last().unwrap()] {
            let val = min.pop().unwrap();

            // Update the range from the last position to the current position
            tree.update(
                if min.is_empty() {
                    1
                } else {
                    *min.last().unwrap() + 1
                },
                val,
                nums[val] - nums[i],
            );
        }

        min.push(i);

        // Increment the total count of ranges ending at position i
        tree.val[1] += tree.data[1].1;
        tree.lazy[1].1 += 1;

        for &(l, idx) in queries[i].iter() {
            ret[idx] = tree.query(l, i);
        }
    }

    for i in 1..=q {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}

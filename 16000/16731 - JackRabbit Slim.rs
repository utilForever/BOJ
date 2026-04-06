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
            val: self.val.max(other.val),
        }
    }
}

const NEG_INF: i64 = -1_000_000_000;

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
            data: vec![Node::new(NEG_INF); real_n * 4],
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

    fn query_first(&self, start: usize, val: i64) -> Option<usize> {
        self.query_first_internal(start, val, 1, 1, self.size)
    }

    fn query_first_internal(
        &self,
        start: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> Option<usize> {
        if node_end < start || self.data[node].val <= val {
            return None;
        }

        if node_start == node_end {
            return Some(node_start);
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_first_internal(start, val, node * 2, node_start, mid);
        let right = self.query_first_internal(start, val, node * 2 + 1, mid + 1, node_end);

        left.or(right)
    }

    fn query_last(&self, end: usize, val: i64) -> Option<usize> {
        self.query_last_internal(end, val, 1, 1, self.size)
    }

    fn query_last_internal(
        &self,
        end: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> Option<usize> {
        if end < node_start || self.data[node].val < val {
            return None;
        }

        if node_start == node_end {
            return Some(node_start);
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_last_internal(end, val, node * 2, node_start, mid);
        let right = self.query_last_internal(end, val, node * 2 + 1, mid + 1, node_end);

        right.or(left)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut carrots = vec![0; n + 1];

    for i in 1..=n {
        carrots[i] = scan.token::<i64>();
    }

    let mut tree_left = SegmentTree::new(n);
    let mut tree_right = SegmentTree::new(n);

    tree_left.update(1, NEG_INF);
    tree_right.update(1, NEG_INF);

    for i in 2..=n {
        tree_left.update(i, 2 * carrots[i] - carrots[i - 1]);
        tree_right.update(i, carrots[i] - 2 * carrots[i - 1]);
    }

    let mut ret = 0;

    for i in 1..=n {
        if i == 1 || i == n {
            ret += carrots[n] - carrots[1];
            continue;
        }

        let mut idx = i;
        let mut left = i - 1;
        let mut right = i + 1;
        let mut direction = if carrots[i] - carrots[left] >= carrots[right] - carrots[i] {
            1
        } else {
            -1
        };

        loop {
            if direction == 1 {
                let pos = match tree_right.query_first(idx + 2, -carrots[left]) {
                    Some(pos) => pos - 1,
                    None => n,
                };

                ret += carrots[pos] - carrots[idx];
                idx = pos;
                right = idx + 1;
            } else {
                let pos = match tree_left.query_last(idx - 1, carrots[right]) {
                    Some(pos) => pos,
                    None => 1,
                };

                ret += carrots[idx] - carrots[pos];
                idx = pos;
                left = idx - 1;
            }

            direction = -direction;

            if left < 1 || right > n {
                break;
            }
        }

        ret += if left >= 1 {
            carrots[idx] - carrots[1]
        } else {
            carrots[n] - carrots[idx]
        };
    }

    writeln!(out, "{ret}").unwrap();
}

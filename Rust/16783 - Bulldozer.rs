use io::Write;
use std::{cmp, io, str};

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
    left_max: i64,
    right_max: i64,
    sum: i64,
    total_max: i64,
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
            left_max: cmp::max(a.left_max, a.sum + b.left_max),
            right_max: cmp::max(b.right_max, b.sum + a.right_max),
            sum: a.sum + b.sum,
            total_max: *vec![a.total_max, b.total_max, a.right_max + b.left_max]
                .iter()
                .max()
                .unwrap(),
        }
    }

    pub fn update(&mut self, val: i64, weight: i64) {
        self.update_internal(val, weight, 1, 0, self.size - 1);
    }

    fn update_internal(
        &mut self,
        val: i64,
        weight: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if node_start == node_end {
            self.data[node].left_max = cmp::max(0, weight);
            self.data[node].right_max = cmp::max(0, weight);
            self.data[node].sum = weight;
            self.data[node].total_max = cmp::max(0, weight);

            return;
        }

        let mid = (node_start + node_end) / 2;

        if val <= mid as i64 {
            self.update_internal(val, weight, node * 2, node_start, mid);
        } else {
            self.update_internal(val, weight, node * 2 + 1, mid + 1, node_end);
        }

        self.data[node] = SegmentTree::merge(&self.data[node * 2], &self.data[node * 2 + 1]);
    }

    fn query(&mut self) -> i64 {
        self.data[1].total_max
    }
}

fn calculate_ccw(p1: (usize, usize, i64, i64), p2: (usize, usize, i64, i64)) -> i64 {
    p1.2 * p2.3 - p2.2 * p1.3
}

// Reference: https://justicehui.github.io/hard-algorithm/2022/03/30/rotate-sweep-line/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![(0, 0, 0); n];
    let mut indexes = vec![0; n];

    for i in 0..n {
        let (x, y, w) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        points[i] = (x, y, w);
    }

    points.sort_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));

    for i in 0..n {
        indexes[i] = i;
    }

    let mut lines = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            lines.push((i, j, points[j].0 - points[i].0, points[j].1 - points[i].1));
        }
    }

    lines.sort_by(|a, b| {
        let ret = a.2 * b.3 - a.3 * b.2;

        if ret == 0 {
            (a.0, a.1).cmp(&(b.0, b.1))
        } else {
            ret.cmp(&0).reverse()
        }
    });

    let mut segment_tree = SegmentTree::new(n);

    for i in 0..n {
        segment_tree.update(i as i64, points[i].2);
    }

    let mut ret = segment_tree.query();

    for i in 0..lines.len() {
        let (next_x, next_y) = (indexes[lines[i].0], indexes[lines[i].1]);

        points.swap(next_x, next_y);
        indexes.swap(lines[i].0, lines[i].1);

        segment_tree.update(next_x as i64, points[next_x].2);
        segment_tree.update(next_y as i64, points[next_y].2);

        if i + 1 < lines.len() && calculate_ccw(lines[i], lines[i + 1]) == 0 {
            continue;
        }

        ret = ret.max(segment_tree.query());
    }

    writeln!(out, "{}", ret).unwrap();
}

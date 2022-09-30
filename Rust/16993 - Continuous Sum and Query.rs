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

#[derive(Clone)]
struct Node {
    left_max: i64,
    mid_max: i64,
    right_max: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self {
            left_max: val,
            mid_max: val,
            right_max: val,
        }
    }

    fn merge(
        cumulative_sum: &Vec<i64>,
        node_left: &Node,
        node_right: &Node,
        left: usize,
        right: usize,
    ) -> Node {
        Node {
            left_max: node_left.left_max.max(
                node_right.left_max + cumulative_sum[(left + right) / 2] - cumulative_sum[left - 1],
            ),
            mid_max: node_left
                .mid_max
                .max(node_right.mid_max)
                .max(node_left.right_max + node_right.left_max),
            right_max: node_right.right_max.max(
                node_left.right_max + cumulative_sum[right] - cumulative_sum[(left + right) / 2],
            ),
        }
    }
}

fn update(
    tree: &mut Vec<Node>,
    cumulative_sum: &Vec<i64>,
    cur: usize,
    index: usize,
    val: i64,
    start: usize,
    end: usize,
) {
    if index > end || index < start {
        return;
    }

    if start == end {
        tree[cur] = Node::new(val);
        return;
    }

    if start != end {
        let mid = (start + end) / 2;
        update(tree, cumulative_sum, cur * 2, index, val, start, mid);
        update(tree, cumulative_sum, cur * 2 + 1, index, val, mid + 1, end);

        let left = tree[cur * 2].clone();
        let right = tree[cur * 2 + 1].clone();
        tree[cur] = Node::merge(cumulative_sum, &left, &right, start, end);
    }
}

fn query(
    tree: &Vec<Node>,
    cumulative_sum: &Vec<i64>,
    cur: usize,
    start: usize,
    end: usize,
    i: usize,
    j: usize,
) -> Node {
    if i > end || j < start {
        return Node::new(-1_000_000_000_000);
    }

    if i <= start && j >= end {
        return tree[cur].clone();
    }

    let mid = (start + end) / 2;
    let left = query(tree, cumulative_sum, cur * 2, start, mid, i, j);
    let right = query(tree, cumulative_sum, cur * 2 + 1, mid + 1, end, i, j);

    Node::merge(cumulative_sum, &left, &right, start, end)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut arr = vec![0; n + 1];
    let mut cumulative_sum = vec![0; n + 1];
    let mut tree = vec![Node::new(0); 4 * (n + 1)];

    for i in 1..=n {
        arr[i] = scan.token();
        cumulative_sum[i] = cumulative_sum[i - 1] + arr[i];

        update(&mut tree, &cumulative_sum, 1, i, arr[i], 1, n);
    }

    let m = scan.token::<i64>();

    for _ in 1..=m {
        let (i, j) = (scan.token::<usize>(), scan.token::<usize>());
        writeln!(
            out,
            "{}",
            query(&tree, &cumulative_sum, 1, 1, n, i, j).mid_max
        )
        .unwrap();
    }
}

use io::Write;
use std::io;

fn input_integers() -> Vec<i64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

#[derive(Clone, Debug)]
struct Node {
    val: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self { val }
    }

    fn merge(&mut self, other: &Self) -> Node {
        let mut ret = Node::new(1);
        ret.val = self.val * other.val;

        ret
    }
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
            data: vec![Node::new(1); real_n * 4],
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

        let mut left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();
        self.data[node] = left.merge(&right);
    }

    fn query(&mut self, start: usize, end: usize) -> Node {
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
            return Node { val: 1 };
        }

        if start <= node_start && node_end <= end {
            return self.data[node].clone();
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        let val = left.val * right.val;

        Node { val }
    }
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let nums = input_integers();

        if nums.is_empty() {
            break;
        }

        let (n, k) = (nums[0] as usize, nums[1] as usize);
        let mut tree = SegmentTree::new(n);
        let arr = input_integers();

        for i in 1..=n {
            tree.update(
                i,
                if arr[i - 1] > 0 {
                    1
                } else if arr[i - 1] < 0 {
                    -1
                } else {
                    0
                },
            );
        }

        for _ in 0..k {
            let mut s = String::new();
            io::stdin().read_line(&mut s).unwrap();
            s = s.trim().to_string();
            let s = s.split_whitespace().collect::<Vec<&str>>();

            if s[0] == "C" {
                let (i, mut v) = (s[1].parse::<usize>().unwrap(), s[2].parse::<i64>().unwrap());
                v = if v > 0 {
                    1
                } else if v < 0 {
                    -1
                } else {
                    0
                };

                tree.update(i, v);
            } else {
                let (i, j) = (
                    s[1].parse::<usize>().unwrap(),
                    s[2].parse::<usize>().unwrap(),
                );
                let ret = tree.query(i, j);

                write!(
                    out,
                    "{}",
                    if ret.val > 0 {
                        '+'
                    } else if ret.val < 0 {
                        '-'
                    } else {
                        '0'
                    }
                )
                .unwrap();
            }
        }

        writeln!(out).unwrap();
    }
}

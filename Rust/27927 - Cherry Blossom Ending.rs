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

fn query_left(
    tree: &Vec<usize>,
    start: usize,
    end: usize,
    node_start: usize,
    node_end: usize,
    node: usize,
) -> i64 {
    if node_start > end || node_end < start {
        return 0;
    }

    if start <= node_start && node_end <= end {
        return tree[node] as i64;
    }

    let mid = (node_start + node_end) / 2;
    let left = query_left(tree, start, end, node_start, mid, 2 * node);
    let right = query_left(tree, start, end, mid + 1, node_end, 2 * node + 1);

    left.max(right)
}

fn query_right(
    tree: &Vec<usize>,
    start: usize,
    end: usize,
    node_start: usize,
    node_end: usize,
    node: usize,
) -> i64 {
    if node_start > end || node_end < start {
        return i64::MAX;
    }

    if start <= node_start && node_end <= end {
        return tree[node] as i64;
    }

    let mid = (node_start + node_end) / 2;
    let left = query_right(tree, start, end, node_start, mid, 2 * node);
    let right = query_right(tree, start, end, mid + 1, node_end, 2 * node + 1);

    left.min(right)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, _) = (scan.token::<usize>(), scan.token::<usize>());
    let mut trees = vec![(0, 0); n + 1];
    let mut size = 1;

    while size < n {
        size *= 2;
    }

    let mut tree_left = vec![0; 2 * size];
    let mut tree_right = vec![0; 2 * size];

    for i in 1..=n {
        trees[i] = (scan.token::<usize>(), scan.token::<usize>());
        tree_left[size + i - 1] = trees[i].0;
        tree_right[size + i - 1] = trees[i].1;
    }

    for i in (1..size).rev() {
        tree_left[i] = tree_left[2 * i].max(tree_left[2 * i + 1]);
        tree_right[i] = tree_right[2 * i].min(tree_right[2 * i + 1]);
    }

    let mut left = 1;
    let mut right = n;
    let mut ret = (0, 0);

    while left <= right {
        let mid = (left + right) / 2;
        let mut values = Vec::new();
        let mut cnt = 0;

        for i in 1..=(n - mid + 1) {
            let val_left = query_left(&tree_left, i, i + mid - 1, 1, size, 1);
            let val_right = query_right(&tree_right, i, i + mid - 1, 1, size, 1);

            if left <= right {
                values.push((val_left, val_right));
            }
        }

        values.sort();

        let mut end = 0;

        for value in values {
            cnt += (value.1 - (value.0 - 1).max(end)).max(0);
            end = end.max(value.1);
        }

        if cnt > 0 {
            ret = (mid, cnt);
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    writeln!(out, "{} {}", ret.0, ret.1).unwrap();
}

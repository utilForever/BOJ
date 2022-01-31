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

fn update(tree: &mut Vec<i64>, cur: usize, index: usize, val: i64, start: usize, end: usize) {
    if index > end || index < start {
        return;
    }

    if start == end {
        tree[cur] = val;
        return;
    }

    if start != end {
        let mid = (start + end) / 2;
        update(tree, cur * 2, index, val, start, mid);
        update(tree, cur * 2 + 1, index, val, mid + 1, end);
    }

    tree[cur] = (tree[cur * 2] * tree[cur * 2 + 1]) % 1_000_000_007;
}

fn query(tree: &Vec<i64>, cur: usize, start: usize, end: usize, i: usize, j: usize) -> i64 {
    if i <= start && j >= end {
        return tree[cur];
    }

    let mut ret = 1;
    let mid = (start + end) / 2;

    if i <= mid {
        ret *= query(tree, cur * 2, start, mid, i, j) % 1_000_000_007;
    }
    if j > mid {
        ret *= query(tree, cur * 2 + 1, mid + 1, end, i, j) % 1_000_000_007;
    }

    ret %= 1_000_000_007;
    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k): (usize, usize, usize) = (scan.token(), scan.token(), scan.token());

    let mut arr = vec![1; n + 1];
    let mut tree = vec![1; 4 * (n + 1)];

    for i in 1..=n {
        arr[i] = scan.token();
        update(&mut tree, 1, i, arr[i], 1, n);
    }

    for _ in 1..=(m + k) {
        let (a, b, c): (usize, usize, usize) = (scan.token(), scan.token(), scan.token());

        if a == 1 {
            update(&mut tree, 1, b, c as i64, 1, n);
        } else {
            writeln!(out, "{}", query(&tree, 1, 1, n, b, c)).unwrap();
        }
    }
}

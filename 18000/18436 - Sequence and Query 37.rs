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
        tree[cur] = val % 2;
        return;
    }

    if start != end {
        let mid = (start + end) / 2;
        update(tree, cur * 2, index, val, start, mid);
        update(tree, cur * 2 + 1, index, val, mid + 1, end);
    }

    tree[cur] = tree[cur * 2] + tree[cur * 2 + 1];
}

fn query(tree: &Vec<i64>, cur: usize, start: usize, end: usize, i: usize, j: usize) -> i64 {
    if i > end || j < start {
        return 0;
    }

    if i <= start && j >= end {
        return tree[cur];
    }

    let mid = (start + end) / 2;

    let ret1 = query(tree, cur * 2, start, mid, i, j);
    let ret2 = query(tree, cur * 2 + 1, mid + 1, end, i, j);

    ret1 + ret2
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    let mut arr = vec![0; n + 1];
    let mut tree: Vec<i64> = vec![0; 4 * (n + 1)];

    for i in 1..=n {
        arr[i] = scan.token();
        update(&mut tree, 1, i, arr[i], 1, n);
    }

    let m = scan.token::<usize>();

    for _ in 1..=m {
        let command = scan.token::<usize>();

        if command == 1 {
            let (i, v) = (scan.token(), scan.token());
            update(&mut tree, 1, i, v, 1, n);
        } else if command == 2 {
            let (l, r) = (scan.token(), scan.token());
            let ret = r as i64 - l as i64 + 1 - query(&tree, 1, 1, n, l, r);
            writeln!(out, "{}", ret).unwrap();
        } else {
            let (l, r) = (scan.token(), scan.token());
            let ret = query(&tree, 1, 1, n, l, r);
            writeln!(out, "{}", ret).unwrap();
        }
    }
}

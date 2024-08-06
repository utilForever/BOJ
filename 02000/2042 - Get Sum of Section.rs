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

fn sum(tree: &Vec<i64>, x: usize) -> i64 {
    let mut sum = 0;
    let mut idx = x as i64;

    while idx > 0 {
        sum += tree[idx as usize];
        idx -= idx & -idx;
    }

    sum
}

fn sum_section(tree: &Vec<i64>, x: usize, y: usize) -> i64 {
    sum(tree, y) - sum(tree, x - 1)
}

fn update(tree: &mut Vec<i64>, x: usize, diff: i64) {
    let mut idx = x as i64;

    while idx < tree.len() as i64 {
        tree[idx as usize] += diff;
        idx += idx & -idx;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k): (usize, usize, usize) = (scan.token(), scan.token(), scan.token());

    let mut arr = vec![0; n + 1];
    let mut tree = vec![0; n + 1];

    for i in 1..=n {
        arr[i] = scan.token();
        update(&mut tree, i, arr[i]);
    }

    for _ in 1..=(m + k) {
        let (a, b, c): (usize, usize, i64) = (scan.token(), scan.token(), scan.token());

        if a == 1 {
            let diff = c - arr[b];
            arr[b] = c;

            update(&mut tree, b, diff);
        } else {
            writeln!(out, "{}", sum_section(&tree, b, c as usize)).unwrap();
        }
    }
}

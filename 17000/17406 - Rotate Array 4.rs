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

fn rotate(nums: &mut Vec<Vec<i64>>, r1: usize, c1: usize, r2: usize, c2: usize) {
    if r1 >= r2 || c1 >= c2 {
        return;
    }

    let mut data = Vec::with_capacity(2 * (r2 - r1 + c2 - c1));

    // Top
    for j in c1..c2 {
        data.push(nums[r1][j]);
    }

    // Right
    for i in r1..r2 {
        data.push(nums[i][c2]);
    }

    // Bottom
    for j in (c1 + 1..=c2).rev() {
        data.push(nums[r2][j]);
    }

    // Left
    for i in (r1 + 1..=r2).rev() {
        data.push(nums[i][c1]);
    }

    let len = data.len();
    let mut idx = 0;

    // New top
    for j in c1..c2 {
        nums[r1][j] = data[(idx + len - 1) % len];
        idx += 1;
    }

    // New right
    for i in r1..r2 {
        nums[i][c2] = data[(idx + len - 1) % len];
        idx += 1;
    }

    // New bottom
    for j in (c1 + 1..=c2).rev() {
        nums[r2][j] = data[(idx + len - 1) % len];
        idx += 1;
    }

    // New left
    for i in (r1 + 1..=r2).rev() {
        nums[i][c1] = data[(idx + len - 1) % len];
        idx += 1;
    }
}

fn process_backtrack(
    nums: &Vec<Vec<i64>>,
    operations: &Vec<(usize, usize, usize)>,
    visited: &mut Vec<bool>,
    order: &mut Vec<usize>,
    depth: usize,
    k: usize,
    ret: &mut i64,
) {
    if depth == k {
        let mut cloned_nums = nums.clone();

        for idx in order.iter() {
            let (r, c, s) = operations[*idx];

            for i in 1..=s {
                rotate(&mut cloned_nums, r - i, c - i, r + i, c + i);
            }
        }

        let val_min = cloned_nums
            .iter()
            .map(|row| row.iter().sum::<i64>())
            .min()
            .unwrap();
        *ret = (*ret).min(val_min);

        return;
    }

    for i in 0..k {
        if visited[i] {
            continue;
        }

        visited[i] = true;
        order.push(i);

        process_backtrack(nums, operations, visited, order, depth + 1, k, ret);

        order.pop();
        visited[i] = false;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut nums = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            nums[i][j] = scan.token::<i64>();
        }
    }

    let mut operations = vec![(0, 0, 0); k];

    for i in 0..k {
        operations[i] = (
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<usize>(),
        );
    }

    let mut visited = vec![false; k];
    let mut order = Vec::with_capacity(k);
    let mut ret = i64::MAX;

    process_backtrack(&nums, &operations, &mut visited, &mut order, 0, k, &mut ret);

    writeln!(out, "{ret}").unwrap();
}

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

fn init(arr: &Vec<usize>, tree: &mut Vec<usize>, node: usize, start: usize, end: usize) {
    if start == end {
        tree[node] = start;
    } else {
        init(arr, tree, node * 2, start, (start + end) / 2);
        init(arr, tree, node * 2 + 1, (start + end) / 2 + 1, end);

        if arr[tree[node * 2]] <= arr[tree[node * 2 + 1]] {
            tree[node] = tree[node * 2];
        } else {
            tree[node] = tree[node * 2 + 1];
        }
    }
}

fn query(
    arr: &Vec<usize>,
    tree: &Vec<usize>,
    node: usize,
    start: usize,
    end: usize,
    i: usize,
    j: usize,
) -> i64 {
    if i > end || j < start {
        return -1;
    }

    if i <= start && j >= end {
        return tree[node] as i64;
    }

    let left = query(arr, tree, node * 2, start, (start + end) / 2, i, j);
    let right = query(arr, tree, node * 2 + 1, (start + end) / 2 + 1, end, i, j);

    if left == -1 {
        return right;
    } else if right == -1 {
        return left;
    } else {
        if arr[left as usize] <= arr[right as usize] {
            return left;
        } else {
            return right;
        }
    }
}

fn get_largest(arr: &Vec<usize>, tree: &Vec<usize>, start: usize, end: usize) -> i64 {
    let m = query(arr, tree, 1, 0, arr.len() - 1, start, end);
    let mut area = ((end - start + 1) * arr[m as usize]) as i64;

    if start as i64 <= m - 1 {
        let temp = get_largest(arr, tree, start, (m - 1) as usize);

        if area < temp {
            area = temp;
        }
    }

    if end as i64 >= m + 1 {
        let temp = get_largest(arr, tree, (m + 1) as usize, end);

        if area < temp {
            area = temp;
        }
    }

    area
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token();
    let mut h = vec![0; n];

    for i in 0..n {
        h[i] = scan.token::<usize>();
    }

    let mut tree = vec![0; 4 * n];

    init(&h, &mut tree, 1, 0, n - 1);

    writeln!(out, "{}", get_largest(&h, &tree, 0, n - 1)).unwrap();
}

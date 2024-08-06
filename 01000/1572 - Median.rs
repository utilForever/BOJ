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

fn update(tree: &mut Vec<i64>, node: usize, start: usize, end: usize, i: usize, diff: i64) {
    if i < start || i > end {
        return;
    }

    if start == end {
        tree[node] += diff;
        return;
    }

    let mid = (start + end) / 2;

    update(tree, node * 2, start, mid, i, diff);
    update(tree, node * 2 + 1, mid + 1, end, i, diff);

    tree[node] = tree[node * 2] + tree[node * 2 + 1];
}

fn find_kth_value(tree: &Vec<i64>, node: usize, start: usize, end: usize, k: i64) -> usize {
    if start == end {
        start
    } else {
        let mid = (start + end) / 2;

        if k <= tree[node * 2] {
            find_kth_value(tree, node * 2, start, mid, k)
        } else {
            find_kth_value(tree, node * 2 + 1, mid + 1, end, k - tree[node * 2])
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut temperature = vec![0; 250_001];
    let mut tree = vec![0; 250_001 * 4];

    for i in 0..n {
        temperature[i] = scan.token::<i64>();
    }

    for i in 0..k - 1 {
        update(&mut tree, 1, 0, 65535, temperature[i] as usize, 1);
    }

    let mut sum_median = 0;

    for i in k - 1..n {
        update(&mut tree, 1, 0, 65535, temperature[i] as usize, 1);

        sum_median += find_kth_value(&tree, 1, 0, 65535, ((k + 1) / 2) as i64) as usize;

        update(&mut tree, 1, 0, 65535, temperature[i - k + 1] as usize, -1);
    }

    writeln!(out, "{}", sum_median).unwrap();
}

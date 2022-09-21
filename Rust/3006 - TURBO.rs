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

    let n = scan.token::<usize>();

    let mut arr = vec![0; n + 1];
    let mut tree = vec![0; n + 1];
    let mut pos = vec![0; n + 1];

    for i in 1..=n {
        arr[i] = scan.token::<i64>();
        pos[arr[i] as usize] = i;

        update(&mut tree, i, 1);
    }

    let mut left = 1;
    let mut right = n;

    for i in 1..=n {
        if i % 2 == 1 {
            update(&mut tree, pos[left], -1);

            writeln!(out, "{}", sum_section(&tree, 1, pos[left])).unwrap();

            left += 1;
        } else {
            update(&mut tree, pos[right], -1);

            writeln!(out, "{}", sum_section(&tree, pos[right], n)).unwrap();

            right -= 1;
        }
    }
}

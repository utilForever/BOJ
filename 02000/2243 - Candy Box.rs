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

    tree[node] += diff;

    if start != end {
        update(tree, node * 2, start, (start + end) / 2, i, diff);
        update(tree, node * 2 + 1, (start + end) / 2 + 1, end, i, diff);
    }
}

fn find_kth_candy(tree: &Vec<i64>, node: usize, start: usize, end: usize, k: i64) -> usize {
    if start == end {
        return start;
    } else {
        if k <= tree[2 * node] {
            return find_kth_candy(tree, node * 2, start, (start + end) / 2, k);
        } else {
            return find_kth_candy(
                tree,
                node * 2 + 1,
                (start + end) / 2 + 1,
                end,
                k - tree[2 * node],
            );
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n: usize = scan.token();

    let mut arr = vec![0; 1_000_001];
    let mut tree = vec![0; 4 * 1_000_001];

    for _ in 0..n {
        let a: usize = scan.token();

        if a == 1 {
            let b = scan.token();

            let candy_num = find_kth_candy(&tree, 1, 1, 1_000_001, b);
            writeln!(out, "{}", candy_num).unwrap();

            arr[candy_num] -= -1;

            update(&mut tree, 1, 1, 1_000_001, candy_num, -1);
        } else if a == 2 {
            let (b, c) = (scan.token(), scan.token());

            arr[b] += c;

            update(&mut tree, 1, 1, 1_000_001, b, c);
        }
    }
}

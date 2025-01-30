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

struct FenwickTree {
    data: Vec<usize>,
    size: usize,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            data: vec![0; n + 1],
            size: n,
        }
    }

    fn update(&mut self, mut idx: usize, val: isize) {
        while idx <= self.size {
            let tmp = self.data[idx] as isize + val;
            self.data[idx] = tmp as usize;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut idx: usize) -> usize {
        let mut sum = 0;

        while idx > 0 {
            sum += self.data[idx];
            idx -= idx & (!idx + 1);
        }

        sum
    }

    fn total(&self) -> usize {
        self.query(self.size)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut heights = vec![0; n];

    for i in 0..n {
        heights[i] = scan.token::<i32>();
    }

    let mut comp = heights.clone();
    comp.sort_unstable();
    comp.dedup();

    let to_idx = |x: i32| comp.binary_search(&x).unwrap() + 1;

    let size = comp.len();
    let mut fen_left = FenwickTree::new(size);
    let mut fen_right = FenwickTree::new(size);

    for i in 1..n {
        let idx = to_idx(heights[i]);
        fen_right.update(idx, 1);
    }

    let mut ret = 0;

    for i in 0..n {
        let total_left = fen_left.total();
        let total_right = fen_right.total();
        let idx = to_idx(heights[i]);

        let l = total_left - fen_left.query(idx);
        let r = total_right - fen_right.query(idx);

        if l > 2 * r || r > 2 * l {
            ret += 1;
        }

        if i < n - 1 {
            fen_left.update(idx, 1);
            let idx2 = to_idx(heights[i + 1]);
            fen_right.update(idx2, -1);
        }
    }

    writeln!(out, "{ret}").unwrap();
}

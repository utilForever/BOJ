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

struct FenwickTree {
    n: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            n,
            data: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.data[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut ret = 0;

        while idx > 0 {
            ret += self.data[idx];
            idx -= idx & (!idx + 1);
        }

        ret
    }

    fn query_range(&self, start: usize, end: usize) -> i64 {
        self.query(end) - self.query(start - 1)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut heights = vec![0; n + 1];

    for i in 1..=n {
        heights[i] = scan.token::<usize>();
    }

    let mut prev = vec![0; n + 1];
    let mut stack = Vec::new();

    for i in 1..=n {
        let height = heights[i];

        while let Some(&(top_height, _)) = stack.last() {
            if top_height > height {
                stack.pop();
            } else {
                break;
            }
        }

        if let Some(&mut (top_height, top_idx)) = stack.last_mut() {
            if top_height == height {
                prev[i] = top_idx;
                *stack.last_mut().unwrap() = (height, i);
                continue;
            }
        }

        prev[i] = 0;
        stack.push((height, i));
    }

    let mut bucket = vec![Vec::new(); n + 1];

    for i in 1..=n {
        bucket[prev[i]].push(i);
    }

    let q = scan.token::<usize>();
    let mut queries = Vec::with_capacity(q);

    for i in 0..q {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
        queries.push((l, r, i));
    }

    queries.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut fenwick = FenwickTree::new(n);

    for &idx in bucket[0].iter() {
        fenwick.update(idx, 1);
    }

    let mut threshold = 0;
    let mut ret = vec![0; q];

    for &(l, r, idx) in queries.iter() {
        while threshold + 1 < l {
            threshold += 1;

            for &pos in bucket[threshold].iter() {
                fenwick.update(pos, 1);
            }
        }

        ret[idx] = fenwick.query_range(l, r);
    }

    for i in 0..q {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}

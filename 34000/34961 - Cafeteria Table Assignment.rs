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
    data: Vec<i32>,
    cnt_bit: usize,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        let mut cnt_bit = 1;

        while cnt_bit <= n {
            cnt_bit <<= 1;
        }

        FenwickTree {
            n,
            data: vec![0; n + 1],
            cnt_bit: cnt_bit >> 1,
        }
    }

    fn update(&mut self, mut idx: usize, delta: i32) {
        while idx <= self.n {
            self.data[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, k: i32) -> (usize, i32) {
        let mut idx = 0;
        let mut bit = self.cnt_bit;
        let mut ret = 0;

        while bit != 0 {
            let next = idx + bit;

            if next <= self.n && ret + self.data[next] < k {
                ret += self.data[next];
                idx = next;
            }

            bit >>= 1;
        }

        (idx + 1, ret)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let q = scan.token::<i64>();
    let mut fenwick_tree = FenwickTree::new(200_000);
    let mut freq = vec![0; 200_001];
    let mut sum = 0;

    for _ in 0..q {
        let (t, x) = (scan.token::<i64>(), scan.token::<usize>());

        if t == 1 {
            fenwick_tree.update(x, 1);
            freq[x] += 1;
            sum += 1;
        } else {
            fenwick_tree.update(x, -1);
            freq[x] -= 1;
            sum -= 1;
        }

        if sum == 0 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let mut idx = 1;
        let mut ret = 0;

        while idx <= sum {
            let (val, less) = fenwick_tree.query(idx);
            let cnt = (less + freq[val] as i32 - idx) / val as i32 + 1;

            idx += cnt * val as i32;
            ret += cnt;
        }

        writeln!(out, "{ret}").unwrap();
    }
}

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

#[derive(Clone)]
struct FenwickTree {
    n: usize,
    bit: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            bit: vec![0; n + 1],
        }
    }

    fn add(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.bit[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn prefix_sum(&self, mut idx: usize) -> i64 {
        let mut ret = 0;

        while idx > 0 {
            ret += self.bit[idx];
            idx &= idx - 1;
        }

        ret
    }

    fn query_prefix_sum(&self, idx: usize) -> i64 {
        self.prefix_sum(idx) - self.prefix_sum(idx - 1)
    }

    fn query(&self, target: i64) -> usize {
        let mut idx = 0;
        let mut bit = 1;

        while bit <= self.n {
            bit <<= 1;
        }

        let mut sum = 0;

        while bit > 0 {
            let next = idx + bit;

            if next <= self.n && sum + self.bit[next] <= target {
                idx = next;
                sum += self.bit[next];
            }

            bit >>= 1;
        }

        idx
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut songs = Vec::with_capacity(n + q);
    let mut tree_cnt = FenwickTree::new(100_000);
    let mut tree_sum = FenwickTree::new(100_000);

    for _ in 0..n {
        let time = scan.token::<usize>();

        songs.push(time);
        tree_cnt.add(time, 1);
        tree_sum.add(time, time as i64);
    }

    for _ in 0..q {
        let command = scan.token::<i64>();

        match command {
            1 => {
                let (j, v) = (scan.token::<usize>(), scan.token::<usize>());
                let old = songs[j - 1];

                if old == v {
                    continue;
                }

                tree_cnt.add(old, -1);
                tree_sum.add(old, -(old as i64));

                songs[j - 1] = v;
                tree_cnt.add(v, 1);
                tree_sum.add(v, v as i64);
            }
            2 => {
                let mut t = scan.token::<i64>();
                let pos = tree_sum.query(t);
                let mut ret = tree_cnt.prefix_sum(pos) as i64;

                t -= tree_sum.prefix_sum(pos);

                if pos < 100_000 && t > 0 {
                    let t_next = (pos + 1) as i64;
                    let t_avail = tree_cnt.query_prefix_sum(pos + 1);
                    ret += std::cmp::min(t_avail, t / t_next);
                }

                writeln!(out, "{ret}").unwrap();
            }
            3 => {
                let v = scan.token::<usize>();

                songs.push(v);
                tree_cnt.add(v, 1);
                tree_sum.add(v, v as i64);
            }
            _ => unreachable!(),
        }
    }
}

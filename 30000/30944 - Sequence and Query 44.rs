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
        self.query(end) - if start > 1 { self.query(start - 1) } else { 0 }
    }
}

struct YoungTableaux {
    n: usize,
    n_sqrt: usize,
    table: Vec<Vec<usize>>,
    is_transposed: bool,
}

impl YoungTableaux {
    fn new(n: usize, is_transposed: bool) -> Self {
        let n_sqrt = (n as f64).sqrt() as usize + 1;

        Self {
            n,
            n_sqrt,
            table: vec![vec![0; n]; n_sqrt],
            is_transposed,
        }
    }

    fn insert(&mut self, mut num: usize) -> Option<usize> {
        let mut row = 1;
        let mut right = self.n;

        loop {
            if row >= self.n_sqrt {
                return None;
            }

            right = right.min(self.table[row][0] + 1);

            let mut left = 1;

            while left < right {
                let mid = (left + right) / 2;

                if !self.is_transposed ^ (self.table[row][mid] < num) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }

            std::mem::swap(&mut self.table[row][left], &mut num);
            self.table[row][0] = self.table[row][0].max(left);

            if num > 0 {
                row += 1;
                right = left;
            } else {
                if self.is_transposed {
                    return Some(row);
                } else if left >= self.n_sqrt {
                    return Some(left);
                }

                return None;
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![0; n + 1];
    let mut queries = vec![(0, 0); q + 1];
    let mut idxes = vec![Vec::new(); 50001];

    for i in 1..=n {
        nums[i] = scan.token::<usize>();
    }

    for i in 1..=q {
        let (m, k) = (scan.token::<usize>(), scan.token::<usize>());

        queries[i] = (m, k);
        idxes[m].push(i);
    }

    let mut young_tableaux1 = YoungTableaux::new(n, true);
    let mut young_tableaux2 = YoungTableaux::new(n, false);
    let mut fenwick_tree = FenwickTree::new(n);
    let mut ret = vec![0; q + 1];

    for i in 1..=n {
        let idx1 = young_tableaux1.insert(nums[i]);
        let idx2 = young_tableaux2.insert(nums[i]);

        if let Some(idx1) = idx1 {
            fenwick_tree.update(idx1, 1);
        }

        if let Some(idx2) = idx2 {
            fenwick_tree.update(idx2, 1);
        }

        for j in 0..idxes[i].len() {
            ret[idxes[i][j]] = fenwick_tree.query_range(1, queries[idxes[i][j]].1);
        }
    }

    for i in 1..=q {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}

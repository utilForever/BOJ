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

mod fenwick_tree {
    pub fn update(tree: &mut Vec<i64>, index: usize, value: i64) {
        let mut idx = index;

        while idx < tree.len() {
            tree[idx] += value;
            idx += idx & (!idx + 1);
        }
    }

    pub fn query(tree: &Vec<i64>, index: usize) -> i64 {
        let mut idx = index;
        let mut ret = 0;

        while idx > 0 {
            ret += tree[idx];
            idx -= idx & (!idx + 1);
        }

        ret
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut states = vec![Vec::new(); n + 1];

    for i in 1..=m {
        let state = scan.token::<usize>();
        states[state].push(i);
    }

    let mut goals = vec![0; n + 1];

    for i in 1..=n {
        goals[i] = scan.token::<i64>();
    }

    let q = scan.token::<usize>();
    let mut queries = vec![(0, 0, 0); q + 1];

    for i in 1..=q {
        queries[i] = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
    }

    let mut tree = vec![0; 300_001];
    let mut queries_mid = vec![Vec::new(); q + 1];
    let mut left = vec![1; n + 1];
    let mut right = vec![q; n + 1];

    loop {
        tree.fill(0);

        let mut should_check = false;

        for i in 1..=n {
            if left[i] <= right[i] {
                should_check = true;
                queries_mid[(left[i] + right[i]) / 2].push(i);
            }
        }

        if !should_check {
            break;
        }

        for i in 1..=q {
            let (l, r, a) = queries[i];

            if l <= r {
                fenwick_tree::update(&mut tree, l, a);
                fenwick_tree::update(&mut tree, r + 1, -a);
            } else {
                fenwick_tree::update(&mut tree, 1, a);
                fenwick_tree::update(&mut tree, r + 1, -a);
                fenwick_tree::update(&mut tree, l, a);
            }

            while let Some(candidate) = queries_mid[i].pop() {
                let mut sum = 0;

                for &state in states[candidate].iter() {
                    sum += fenwick_tree::query(&tree, state);

                    if sum >= goals[candidate] {
                        break;
                    }
                }

                if sum >= goals[candidate] {
                    right[candidate] = i - 1;
                } else {
                    left[candidate] = i + 1;
                }
            }
        }
    }

    for i in 1..=n {
        if left[i] == q + 1 {
            writeln!(out, "NIE").unwrap();
        } else {
            writeln!(out, "{}", left[i]).unwrap();
        }
    }
}

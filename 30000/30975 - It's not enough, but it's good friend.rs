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

fn process_tsp(
    matrix: &Vec<Vec<i64>>,
    prerequisites: &Vec<usize>,
    cost: &mut Vec<Vec<i64>>,
    n: usize,
    curr: usize,
    visited: usize,
) -> i64 {
    if cost[curr][visited] != 0 {
        return cost[curr][visited];
    }

    if visited == (1 << n) - 1 {
        return matrix[curr][n];
    }

    let mut cost_min = i64::MAX / 4;

    for i in 0..n {
        if visited & (1 << i) == 0
            && matrix[curr][i] != i64::MAX / 4
            && (prerequisites[i] == i || visited & (1 << prerequisites[i]) != 0)
        {
            let cost = process_tsp(matrix, prerequisites, cost, n, i, visited + (1 << i));
            cost_min = std::cmp::min(cost_min, cost + matrix[curr][i]);
        }
    }

    cost[curr][visited] = cost_min;
    cost_min
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut matrix = vec![vec![i64::MAX / 4; n + 1]; n + 1];
    let mut cost = vec![vec![0; 1 << (n + 1)]; n + 1];

    let mut prerequisites = vec![0; n];

    for i in 0..n {
        prerequisites[i] = scan.token::<usize>() - 1;
    }

    for _ in 0..m {
        let (u, v, w) = (
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<i64>(),
        );
        matrix[u][v] = matrix[u][v].min(w);
    }

    let ret = process_tsp(&matrix, &prerequisites, &mut cost, n, n, 0);

    writeln!(out, "{}", if ret == i64::MAX / 4 { -1 } else { ret }).unwrap();
}

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
    matrix: &Vec<Vec<f64>>,
    cost: &mut Vec<Vec<f64>>,
    n: usize,
    curr: usize,
    visited: usize,
) -> f64 {
    if cost[curr][visited] != 0.0 {
        return cost[curr][visited];
    }

    if visited == (1 << n) - 1 {
        if matrix[curr][0] != 0.0 {
            return matrix[curr][0];
        } else {
            return 1_000_000_000.0;
        }
    }

    let mut min_cost = 1_000_000_000.0_f64;

    for i in 0..n {
        if visited & (1 << i) == 0 && matrix[curr][i] != 0.0 {
            let cost = process_tsp(matrix, cost, n, i, visited + (1 << i));
            min_cost = min_cost.min(cost + matrix[curr][i]);
        }
    }

    cost[curr][visited] = min_cost;
    min_cost
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![(0, 0); n];
    let mut matrix = vec![vec![0.0; n]; n];
    let mut cost = vec![vec![0.0; 1 << n]; n];

    for i in 0..n {
        points[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    for i in 0..n {
        for j in 0..n {
            matrix[i][j] =
                ((points[i].0 - points[j].0) as f64).hypot((points[i].1 - points[j].1) as f64);
        }
    }

    writeln!(out, "{:.6}", process_tsp(&matrix, &mut cost, n, 0, 1)).unwrap();
}

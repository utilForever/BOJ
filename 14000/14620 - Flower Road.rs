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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn process_backtracking(
    garden: &Vec<Vec<i64>>,
    visited: &mut Vec<Vec<bool>>,
    depth: i64,
    cost: i64,
    cost_min: &mut i64,
) {
    if depth == 3 {
        *cost_min = std::cmp::min(*cost_min, cost);
        return;
    }

    let n = garden.len();

    for i in 1..n - 1 {
        for j in 1..n - 1 {
            if visited[i][j]
                || visited[i - 1][j]
                || visited[i + 1][j]
                || visited[i][j - 1]
                || visited[i][j + 1]
            {
                continue;
            }

            visited[i][j] = true;
            visited[i - 1][j] = true;
            visited[i + 1][j] = true;
            visited[i][j - 1] = true;
            visited[i][j + 1] = true;

            let sum = garden[i][j]
                + garden[i - 1][j]
                + garden[i + 1][j]
                + garden[i][j - 1]
                + garden[i][j + 1];

            process_backtracking(garden, visited, depth + 1, cost + sum, cost_min);

            visited[i][j] = false;
            visited[i - 1][j] = false;
            visited[i + 1][j] = false;
            visited[i][j - 1] = false;
            visited[i][j + 1] = false;
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut garden = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            garden[i][j] = scan.token::<i64>();
        }
    }

    let mut visited = vec![vec![false; n]; n];
    let mut cost_min = i64::MAX;

    process_backtracking(&garden, &mut visited, 0, 0, &mut cost_min);

    writeln!(out, "{cost_min}").unwrap();
}

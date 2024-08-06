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

fn process_dfs(
    heights: &Vec<i64>,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    index: usize,
) -> i64 {
    visited[index] = true;

    let mut sum = 0;

    for &next in graph[index].iter() {
        if visited[next] {
            continue;
        }

        sum += process_dfs(heights, graph, visited, next);
    }

    (sum + heights[index]).max(0)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, p) = (scan.token::<usize>(), scan.token::<usize>());
    let mut heights = vec![0; n + 1];

    for i in 1..=n {
        heights[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        let height = scan.token::<i64>();
        heights[i] = height - heights[i];
    }

    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());

        graph[u].push(v);
        graph[v].push(u);
    }

    let mut visited = vec![false; n + 1];

    writeln!(out, "{}", process_dfs(&heights, &graph, &mut visited, p)).unwrap();
}

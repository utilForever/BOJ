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
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    order_visit: &mut Vec<i64>,
    cnt: &mut i64,
    idx: usize,
) {
    visited[idx] = true;
    order_visit[idx] = *cnt;
    *cnt += 1;

    for &next in graph[idx].iter() {
        if !visited[next] {
            process_dfs(graph, visited, order_visit, cnt, next);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, r) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut graph = vec![Vec::new(); n + 1];
    let mut visited = vec![false; n + 1];
    let mut order_visit = vec![0; n + 1];
    let mut cnt = 1;

    for _ in 0..m {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    for i in 1..=n {
        graph[i].sort_by(|a, b| b.cmp(a));
    }

    process_dfs(&graph, &mut visited, &mut order_visit, &mut cnt, r);

    for i in 1..=n {
        writeln!(out, "{}", order_visit[i]).unwrap();
    }
}

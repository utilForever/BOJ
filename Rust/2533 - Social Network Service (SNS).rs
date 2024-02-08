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
    dp: &mut Vec<Vec<i64>>,
    visited: &mut Vec<bool>,
    curr: usize,
) {
    dp[curr][0] = 1;
    visited[curr] = true;

    for &next in graph[curr].iter() {
        if visited[next] {
            continue;
        }

        process_dfs(graph, dp, visited, next);

        dp[curr][0] += dp[next][1].min(dp[next][0]);
        dp[curr][1] += dp[next][0];
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];
    let mut visited = vec![false; n + 1];
    let mut dp = vec![vec![0; 2]; n + 1];

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    process_dfs(&graph, &mut dp, &mut visited, 1);

    writeln!(out, "{}", dp[1][0].min(dp[1][1])).unwrap();
}

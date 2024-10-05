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

fn process_dfs_for_1st_line(
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    path: &mut Vec<bool>,
    curr: usize,
    target: usize,
) -> bool {
    visited[curr] = true;

    if curr == target {
        path[curr] = true;
        return true;
    }

    for &next in graph[curr].iter() {
        if visited[next] {
            continue;
        }

        if process_dfs_for_1st_line(graph, visited, path, next, target) {
            path[curr] = true;
            return true;
        }
    }

    false
}

fn process_dfs_for_2nd_line(
    graph: &Vec<Vec<usize>>,
    path_1st_line: &Vec<bool>,
    visited: &mut Vec<bool>,
    cnt: &mut i64,
    curr: usize,
) {
    visited[curr] = true;
    *cnt += 1;

    for &next in graph[curr].iter() {
        if path_1st_line[next] {
            continue;
        }

        if visited[next] {
            continue;
        }

        process_dfs_for_2nd_line(graph, path_1st_line, visited, cnt, next);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut visited = vec![false; n + 1];
    let mut path_1st_line = vec![false; n + 1];

    process_dfs_for_1st_line(&graph, &mut visited, &mut path_1st_line, 1, n);

    let mut visited = vec![false; n + 1];
    let mut cnt_prefix_sum = 0;
    let mut ret = 0;

    for i in 1..=n {
        if path_1st_line[i] {
            continue;
        }

        if visited[i] {
            continue;
        }

        let mut cnt = 0;

        process_dfs_for_2nd_line(&graph, &path_1st_line, &mut visited, &mut cnt, i);

        ret += cnt * cnt_prefix_sum;
        cnt_prefix_sum += cnt;
    }

    writeln!(out, "{ret}").unwrap();
}

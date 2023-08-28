use io::Write;
use std::{collections::VecDeque, io, str};

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

fn process_bfs(
    graph: &Vec<Vec<usize>>,
    depths: &mut Vec<i64>,
    visited: &mut Vec<bool>,
    start: usize,
    end: usize,
    n: usize,
) {
    let mut tracking = vec![0; n + 1];

    depths[start] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();

        for &next in &graph[curr] {
            if depths[next] == -1 {
                depths[next] = depths[curr] + 1;
                tracking[next] = curr;

                queue.push_back(next);
            }
        }
    }

    let mut idx = end;

    while idx != 0 {
        visited[idx] = true;
        idx = tracking[idx];
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, s, e) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut graph = vec![Vec::new(); n + 1];
    let mut edges = vec![(0, 0); m];
    let mut depths = vec![-1; n + 1];
    let mut visited = vec![false; n + 1];

    for i in 0..m {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph[u].push(v);
        graph[v].push(u);
        edges[i] = (u, v);
    }

    process_bfs(&graph, &mut depths, &mut visited, s, e, n);

    if depths[e] == -1 {
        writeln!(out, "-1").unwrap();
        return;
    }

    for (u, v) in edges {
        if visited[u] && visited[v] {
            write!(out, "{} ", if depths[u] >= depths[v] { 1 } else { 0 }).unwrap();
        } else if visited[u] {
            write!(out, "1 ").unwrap();
        } else if visited[v] {
            write!(out, "0 ").unwrap();
        } else {
            write!(out, "0 ").unwrap();
        }
    }

    writeln!(out).unwrap();
}

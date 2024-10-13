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

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    stack: &mut Vec<usize>,
    top: &mut usize,
    node: usize,
) {
    visited[node] = true;

    for &next in graph[node].iter() {
        if visited[next] {
            continue;
        }

        process_dfs(graph, visited, stack, top, next);
    }

    stack[*top] = node;
    *top += 1;
}

fn process_dfs_rev(
    scc_group: &mut Vec<Vec<usize>>,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    node: usize,
) {
    visited[node] = true;

    let len = scc_group.len();
    scc_group[len - 1].push(node);

    for &next in graph[node].iter() {
        if visited[next] {
            continue;
        }

        process_dfs_rev(scc_group, graph, visited, next);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (v, e) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); v + 1];
    let mut graph_rev = vec![Vec::new(); v + 1];

    for _ in 0..e {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        graph[a].push(b);
        graph_rev[b].push(a);
    }

    let mut scc_group = Vec::new();
    let mut visited = vec![false; v + 1];
    let mut stack = vec![0; v + 1];
    let mut top = 0;

    for idx in 1..=v {
        if visited[idx] {
            continue;
        }

        process_dfs(&graph, &mut visited, &mut stack, &mut top, idx);
    }

    visited.fill(false);

    while top > 0 {
        let node = stack[top - 1];
        top -= 1;

        if visited[node] {
            continue;
        }

        scc_group.push(Vec::new());
        process_dfs_rev(&mut scc_group, &graph_rev, &mut visited, node);
    }

    for group in scc_group.iter_mut() {
        group.sort_unstable();
    }

    scc_group.sort_unstable();

    writeln!(out, "{}", scc_group.len()).unwrap();

    for group in scc_group.iter() {
        for node in group.iter() {
            write!(out, "{node} ").unwrap();
        }

        writeln!(out, "-1").unwrap();
    }
}

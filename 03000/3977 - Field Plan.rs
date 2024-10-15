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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
        let mut graph = vec![Vec::new(); n + 1];
        let mut graph_rev = vec![Vec::new(); n + 1];

        for _ in 0..m {
            let (v, w) = (scan.token::<usize>() + 1, scan.token::<usize>() + 1);
            graph[v].push(w);
            graph_rev[w].push(v);
        }

        let mut scc_group = Vec::new();
        let mut visited = vec![false; n + 1];
        let mut stack = vec![0; n + 1];
        let mut top = 0;

        for idx in 1..=n {
            if visited[idx] {
                continue;
            }

            process_dfs(&graph, &mut visited, &mut stack, &mut top, idx);
        }

        visited.fill(false);

        let mut scc_id = vec![0; n + 1];
        let mut scc_count = 0;

        while top > 0 {
            let node = stack[top - 1];
            top -= 1;

            if visited[node] {
                continue;
            }

            scc_group.push(Vec::new());
            process_dfs_rev(&mut scc_group, &graph_rev, &mut visited, node);

            for &member in scc_group[scc_count].iter() {
                scc_id[member] = scc_count;
            }

            scc_count += 1;
        }

        let mut in_degree = vec![0; scc_count];

        for curr in 1..=n {
            for &next in graph[curr].iter() {
                let pos_curr = scc_id[curr];
                let pos_next = scc_id[next];

                if pos_curr == pos_next {
                    continue;
                }

                in_degree[pos_next] += 1;
            }
        }

        let cnt = in_degree.iter().filter(|&&x| x == 0).count();

        if cnt == 1 {
            let pos = in_degree.iter().position(|&x| x == 0).unwrap();

            scc_group[pos].sort_unstable();

            for idx in scc_group[pos].iter() {
                writeln!(out, "{} ", idx - 1).unwrap();
            }
        } else {
            writeln!(out, "Confused").unwrap();
        }

        writeln!(out).unwrap();
    }
}

use io::Write;
use std::{collections::HashSet, io, str};

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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn process_dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, from: usize) -> (usize, usize) {
    let mut stack = vec![(from, 0)];
    let mut node_farthest = from;
    let mut dist_max = 0;

    visited[from] = true;

    while let Some((node, dist)) = stack.pop() {
        if dist > dist_max {
            dist_max = dist;
            node_farthest = node;
        }

        for &next in graph[node].iter() {
            if !visited[next] {
                visited[next] = true;
                stack.push((next, dist + 1));
            }
        }
    }

    (dist_max, node_farthest)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut edges1 = vec![Vec::new(); n + 1];
    let mut edges2 = vec![Vec::new(); n + 1];

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        edges1[u].push(v);
        edges1[v].push(u);
    }

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        edges2[u].push(v);
        edges2[v].push(u);
    }

    let mut graph = vec![Vec::new(); n + 1];

    for i in 1..=n {
        let set1 = edges1[i].iter().cloned().collect::<HashSet<_>>();
        let set2 = edges2[i].iter().cloned().collect::<HashSet<_>>();
        let common = set1.intersection(&set2).cloned().collect::<Vec<_>>();

        for &node in common.iter() {
            graph[i].push(node);
        }
    }

    let mut visited = vec![false; n + 1];
    let mut dist_max = 0;
    let (mut s, mut e) = (0, 0);

    for i in 1..=n {
        if visited[i] || graph[i].is_empty() {
            continue;
        }

        let a = process_dfs(&graph, &mut visited, i);
        let b = process_dfs(&graph, &mut vec![false; graph.len()], a.1);

        if b.0 > dist_max {
            dist_max = b.0;
            s = a.1;
            e = b.1;
        }
    }

    if dist_max == 0 {
        writeln!(out, "-1").unwrap();
        return;
    }

    writeln!(out, "{s} {e}").unwrap();
}

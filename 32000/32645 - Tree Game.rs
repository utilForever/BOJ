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
    ret: &mut Vec<bool>,
    node: usize,
    parent: usize,
) {
    let mut is_leaf = true;

    for &child in graph[node].iter() {
        if child == parent {
            continue;
        }

        is_leaf = false;
        visited[node] = true;

        process_dfs(graph, visited, ret, child, node);
    }

    ret[node] = false;

    if is_leaf {
        return;
    }

    for &child in graph[node].iter() {
        if child == parent {
            continue;
        }

        if !ret[child] {
            ret[node] = true;
        }
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
    let mut ret = vec![false; n + 1];

    process_dfs(&graph, &mut visited, &mut ret, 1, 0);

    for i in 1..=n {
        writeln!(out, "{}", if ret[i] { "donggggas" } else { "uppercut" }).unwrap();
    }
}

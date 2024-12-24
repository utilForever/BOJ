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
    pair_u: &mut [Option<usize>],
    pair_v: &mut [Option<usize>],
    dist: &mut [i64],
) -> bool {
    let len_u = pair_u.len();
    let mut queue = VecDeque::new();

    for u in 0..len_u {
        if pair_u[u].is_none() {
            dist[u] = 0;
            queue.push_back(u);
        } else {
            dist[u] = -1;
        }
    }

    let mut ret = false;

    while let Some(u) = queue.pop_front() {
        if dist[u] == -1 {
            continue;
        }

        for &v in graph[u].iter() {
            let u_match = pair_v[v];

            if let Some(u_match_idx) = u_match {
                if dist[u_match_idx] == -1 {
                    dist[u_match_idx] = dist[u] + 1;
                    queue.push_back(u_match_idx);
                }
            } else {
                ret = true;
            }
        }
    }

    ret
}

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    pair_u: &mut [Option<usize>],
    pair_v: &mut [Option<usize>],
    dist: &mut [i64],
    u: usize,
) -> bool {
    if u == usize::MAX {
        return true;
    }

    for &v in graph[u].iter() {
        let u_match = pair_v[v];

        if u_match.is_none()
            || (u_match.is_some()
                && dist[u_match.unwrap()] == dist[u] + 1
                && process_dfs(graph, pair_u, pair_v, dist, u_match.unwrap()))
        {
            pair_v[v] = Some(u);
            pair_u[u] = Some(v);

            return true;
        }
    }

    dist[u] = -1;

    false
}

fn hopkroft_karp(graph: &Vec<Vec<usize>>, len_u: usize, len_v: usize) -> usize {
    let mut pair_u = vec![None; len_u];
    let mut pair_v = vec![None; len_v];
    let mut dist = vec![0; len_u];
    let mut ret = 0;

    while process_bfs(&graph, &mut pair_u, &mut pair_v, &mut dist) {
        for u in 0..len_u {
            if pair_u[u].is_some() {
                continue;
            }

            if process_dfs(&graph, &mut pair_u, &mut pair_v, &mut dist, u) {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut graph = vec![Vec::new(); n];

    for _ in 0..m {
        let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
        graph[r - 1].push(c - 1);
    }

    let ret = hopkroft_karp(&graph, n, n);

    writeln!(out, "{ret}").unwrap();
}

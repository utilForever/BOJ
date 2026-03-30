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

const INF: i64 = i64::MAX / 4;

fn process_bellman_ford(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Option<Vec<i64>> {
    let n = graph.len() - 1;
    let mut ret = vec![INF; graph.len()];

    ret[from] = 0;

    for i in 1..=n {
        let mut updated = false;

        for u in 1..=n {
            if ret[u] == INF {
                continue;
            }

            for &(v, w) in graph[u].iter() {
                if ret[v] > ret[u] + w {
                    ret[v] = ret[u] + w;
                    updated = true;

                    if i == n {
                        return None;
                    }
                }
            }
        }

        if !updated {
            break;
        }
    }

    Some(ret)
}

fn process_bfs(graph: &Vec<Vec<usize>>, starts: &Vec<usize>) -> Vec<bool> {
    let n = graph.len() - 1;
    let mut visited = vec![false; n + 1];
    let mut queue = VecDeque::new();

    for &start in starts {
        visited[start] = true;
        queue.push_back(start);
    }

    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if visited[v] {
                continue;
            }

            visited[v] = true;
            queue.push_back(v);
        }
    }

    visited
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut costs_wait = vec![0; n + 1];
    let mut costs_ride = vec![0; n + 1];

    for i in 1..=n {
        let (t, p) = (scan.token::<i64>(), scan.token::<i64>());

        costs_wait[i] = p;
        costs_ride[i] = t + p;
    }

    let mut edges = Vec::with_capacity(m);
    let mut graph = vec![Vec::new(); n + 1];
    let mut graph_rev = vec![Vec::new(); n + 1];
    let mut costs_portal = Vec::with_capacity(m);

    for _ in 0..m {
        let (u, v, s) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        edges.push((u, v, s));
        graph[u].push(v);
        graph_rev[v].push(u);
        costs_portal.push(s);
    }

    let wait_min = *costs_wait[1..].iter().min().unwrap();
    let wait_max = *costs_wait[1..].iter().max().unwrap();

    let sources = (1..=n)
        .filter(|&i| costs_wait[i] == wait_min)
        .collect::<Vec<_>>();
    let targets = (1..=n)
        .filter(|&i| costs_wait[i] == wait_max)
        .collect::<Vec<_>>();

    let from_source = process_bfs(&graph, &sources);
    let to_dest = process_bfs(&graph_rev, &targets);
    let mut relavant = vec![false; n + 1];

    for i in 1..=n {
        relavant[i] = from_source[i] && to_dest[i];
    }

    if !targets.iter().any(|&x| relavant[x]) {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut edges2 = Vec::new();
    let mut graph2 = vec![Vec::new(); n + 2];

    for &(u, v, s) in edges.iter() {
        if relavant[u] && relavant[v] {
            graph2[u].push((v, s + costs_ride[v]));
            edges2.push((u, v));
        }
    }

    for &s in sources.iter() {
        if relavant[s] {
            graph2[n + 1].push((s, costs_ride[s]));
        }
    }

    let dist = match process_bellman_ford(&graph2, n + 1) {
        Some(d) => d,
        None => {
            writeln!(out, "-1").unwrap();
            return;
        }
    };

    let mut shortest_before_swap = INF;

    for &idx in targets.iter() {
        shortest_before_swap = shortest_before_swap.min(dist[idx]);
    }

    if shortest_before_swap == INF {
        writeln!(out, "-1").unwrap();
        return;
    }

    costs_portal.sort_unstable();

    let mut prefix_sum = vec![0; m + 1];

    for i in 0..m {
        prefix_sum[i + 1] = prefix_sum[i] + costs_portal[i];
    }

    for i in 1..=n {
        if !relavant[i] {
            continue;
        }

        let mut dp = vec![INF; n + 1];
        dp[i] = 0;

        for j in 1..=n.min(m) {
            let mut dp_next = vec![INF; n + 1];

            for &(u, v) in edges2.iter() {
                if dp[u] == INF {
                    continue;
                }

                dp_next[v] = dp_next[v].min(dp[u] + costs_ride[v]);
            }

            dp = dp_next;

            if dp[i] != INF && dp[i] + prefix_sum[j] < 0 {
                writeln!(out, "-1").unwrap();
                return;
            }
        }
    }

    let mut dp = vec![INF; n + 1];

    for &s in sources.iter() {
        if relavant[s] {
            dp[s] = costs_ride[s];
        }
    }

    let mut shortest_after_swap = INF;

    for &t in targets.iter() {
        shortest_after_swap = shortest_after_swap.min(dp[t]);
    }

    for i in 1..=(n - 1).min(m) {
        let mut dp_next = vec![INF; n + 1];

        for &(u, v) in edges2.iter() {
            if dp[u] == INF {
                continue;
            }

            dp_next[v] = dp_next[v].min(dp[u] + costs_ride[v]);
        }

        dp = dp_next;

        let mut curr = INF;

        for &t in targets.iter() {
            curr = curr.min(dp[t]);
        }

        if curr != INF {
            shortest_after_swap = shortest_after_swap.min(curr + prefix_sum[i]);
        }
    }

    writeln!(out, "{}", shortest_before_swap - shortest_after_swap).unwrap();
}

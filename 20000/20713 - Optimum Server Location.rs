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

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    rev: usize,
    capacity: i64,
}

impl Edge {
    fn new(to: usize, rev: usize, capacity: i64) -> Self {
        Self { to, rev, capacity }
    }
}

struct Dinic {
    graph: Vec<Vec<Edge>>,
    source: usize,
    sink: usize,
    check: Vec<i64>,
    done: Vec<usize>,
}

impl Dinic {
    fn new(n: usize, source: usize, sink: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n],
            source,
            sink,
            check: vec![-1; n],
            done: vec![0; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, capacity: i64) {
        let orig = self.graph[v].len();
        let dest = self.graph[u].len();

        self.graph[u].push(Edge::new(v, orig, capacity));
        self.graph[v].push(Edge::new(u, dest, 0));
    }

    fn process_bfs(&mut self) -> bool {
        self.check.fill(-1);

        let mut queue = VecDeque::new();

        queue.push_back(self.source);
        self.check[self.source] = 0;

        while let Some(u) = queue.pop_front() {
            for &e in self.graph[u].iter() {
                if e.capacity > 0 && self.check[e.to] < 0 {
                    queue.push_back(e.to);
                    self.check[e.to] = self.check[u] + 1;
                }
            }
        }

        self.check[self.sink] >= 0
    }

    fn process_dfs(&mut self, idx: usize, flow: i64) -> i64 {
        if idx == self.sink {
            return flow;
        }

        let n = self.graph[idx].len();

        while self.done[idx] < n {
            let edge = self.graph[idx][self.done[idx]];

            if edge.capacity > 0 && self.check[edge.to] == self.check[idx] + 1 {
                let flow_current = self.process_dfs(edge.to, flow.min(edge.capacity));

                if flow_current > 0 {
                    self.graph[idx][self.done[idx]].capacity -= flow_current;
                    self.graph[edge.to][edge.rev].capacity += flow_current;

                    return flow_current;
                }
            }

            self.done[idx] += 1;
        }

        0
    }

    fn get_flow(&mut self) -> i64 {
        let mut flow_total = 0;

        while self.process_bfs() {
            self.done.fill(0);

            loop {
                let flow_current = self.process_dfs(self.source, i64::MAX);

                if flow_current == 0 {
                    break;
                }

                flow_total += flow_current;
            }
        }

        flow_total
    }

    fn reachable_from_source(&self) -> Vec<bool> {
        let n = self.graph.len();
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();

        visited[self.source] = true;
        queue.push_back(self.source);

        while let Some(u) = queue.pop_front() {
            for &edge in self.graph[u].iter() {
                if edge.capacity > 0 && !visited[edge.to] {
                    visited[edge.to] = true;
                    queue.push_back(edge.to);
                }
            }
        }

        visited
    }
}

const INF: i64 = 1_000_000_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut locations = vec![0; m];

    for i in 0..m {
        locations[i] = scan.token::<i64>();
    }

    let mut throughputs_oondex_cdn = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            throughputs_oondex_cdn[i][j] = scan.token::<i64>();
        }
    }

    let mut throughts_oondex_oondex = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            throughts_oondex_oondex[i][j] = scan.token::<i64>();
        }
    }

    let mut positions = locations.clone();
    positions.sort_unstable();
    positions.dedup();

    let idx = |i: usize, j: usize| -> usize { 2 + i * (positions.len() - 1) + (j - 1) };

    let source = 0;
    let sink = 1;
    let mut maximum_flow = Dinic::new(n * (positions.len() - 1) + 2, source, sink);

    let mut base = 0;

    for i in 0..n {
        let mut costs = vec![0; positions.len()];

        for j in 0..positions.len() {
            let mut sum = 0;

            for k in 0..m {
                sum += throughputs_oondex_cdn[i][k] * (positions[j] - locations[k]).abs();
            }

            costs[j] = sum;
        }

        base += costs[0];

        for j in 1..positions.len() {
            let diff = costs[j] - costs[j - 1];
            let v = idx(i, j);

            if diff < 0 {
                maximum_flow.add_edge(source, v, -diff);
                base += diff;
            } else {
                maximum_flow.add_edge(v, sink, diff);
            }
        }

        for j in 2..positions.len() {
            maximum_flow.add_edge(idx(i, j), idx(i, j - 1), INF);
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            if throughts_oondex_oondex[i][j] == 0 {
                continue;
            }

            for k in 1..positions.len() {
                let diff = positions[k] - positions[k - 1];
                let capacity = throughts_oondex_oondex[i][j] * diff;

                if capacity == 0 {
                    continue;
                }

                let u = idx(i, k);
                let v = idx(j, k);

                maximum_flow.add_edge(u, v, capacity);
                maximum_flow.add_edge(v, u, capacity);
            }
        }
    }

    let flow = maximum_flow.get_flow();
    let ret = base + flow;
    let reach = maximum_flow.reachable_from_source();

    let mut labels = vec![positions[0]; n];

    for i in 0..n {
        let mut label = 0;

        for j in 1..positions.len() {
            if reach[idx(i, j)] {
                label = j;
            }
        }

        labels[i] = positions[label];
    }

    writeln!(out, "{ret}").unwrap();

    for i in 0..n {
        write!(out, "{} ", labels[i]).unwrap();
    }

    writeln!(out).unwrap();
}

use io::Write;
use std::{
    collections::{BinaryHeap, VecDeque},
    io, str,
};

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
}

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Vec<i64> {
    let mut ret = vec![INF; graph.len()];
    ret[from] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, from));

    while !queue.is_empty() {
        let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
        cost_curr *= -1;

        if ret[vertex_curr] < cost_curr {
            continue;
        }

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;

            cost_next += cost_curr;

            if ret[vertex_next] > cost_next {
                ret[vertex_next] = cost_next;
                queue.push((-cost_next, vertex_next));
            }
        }
    }

    ret
}

const INF: i64 = 1_000_000_000_000;
const CITY_A: usize = 95_050;
const CITY_B: usize = 104_729;

fn check(
    edges: &Vec<(usize, usize, i64)>,
    needs: &Vec<i64>,
    source: usize,
    sink: usize,
    n: usize,
    g: i64,
    x: i64,
) -> bool {
    let mut maximum_flow = Dinic::new(n + 2, source, sink);

    for (edge, &need) in edges.iter().zip(needs.iter()) {
        let capacity = if need <= x { 1 } else { INF };
        maximum_flow.add_edge(edge.0, edge.1, capacity);
        maximum_flow.add_edge(edge.1, edge.0, capacity);
    }

    maximum_flow.get_flow() <= g
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, g, e) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut graph = vec![Vec::new(); n + 2];
    let mut edges = Vec::with_capacity(e);

    for _ in 0..e {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            2 * scan.token::<i64>(),
        );
        let a = if a == CITY_A {
            n
        } else if a == CITY_B {
            n + 1
        } else {
            a
        };
        let b = if b == CITY_A {
            n
        } else if b == CITY_B {
            n + 1
        } else {
            b
        };

        graph[a].push((b, c));
        graph[b].push((a, c));
        edges.push((a, b, c));
    }

    let source = n;
    let sink = n + 1;

    let dist_source = process_dijkstra(&graph, source);
    let dist_sink = process_dijkstra(&graph, sink);

    if dist_source[sink] >= INF / 2 {
        if g == 0 {
            writeln!(out, "0.0").unwrap();
        } else {
            writeln!(out, "Impossible").unwrap();
        }

        return;
    }

    let needs = edges
        .iter()
        .map(|edge| {
            let (a, b) = (dist_source[edge.0], dist_source[edge.1]);
            let (c, d) = (dist_sink[edge.0], dist_sink[edge.1]);
            let w = edge.2;

            if a >= INF / 2 || b >= INF / 2 || c >= INF / 2 || d >= INF / 2 {
                return INF;
            }

            let mut ret = a.max(c);
            ret = ret.min(b.max(d));
            ret = ret.min(a.max(d).max((a + d + w) / 2));
            ret = ret.min(b.max(c).max((b + c + w) / 2));

            ret
        })
        .collect::<Vec<_>>();

    let mut candidates = needs
        .iter()
        .copied()
        .filter(|&x| x < INF)
        .collect::<Vec<_>>();
    candidates.sort_unstable();
    candidates.dedup();

    if candidates.is_empty() {
        writeln!(out, "Impossible").unwrap();
        return;
    }

    if !check(
        &edges,
        &needs,
        source,
        sink,
        n,
        g,
        *candidates.last().unwrap(),
    ) {
        writeln!(out, "Impossible").unwrap();
        return;
    }

    let mut left = 0;
    let mut right = candidates.len() - 1;

    while left < right {
        let mid = (left + right) / 2;

        if check(&edges, &needs, source, sink, n, g, candidates[mid]) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    writeln!(out, "{:.1}", candidates[left] as f64 / 2.0).unwrap();
}

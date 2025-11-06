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
}

const INF: i64 = 1_000_000_000_000;

fn parents_by_bfs(edges: &Vec<Vec<usize>>, root: usize) -> Vec<usize> {
    let n = edges.len();
    let mut parent = vec![n; n];
    let mut deque = VecDeque::new();

    parent[root] = root;
    deque.push_back(root);

    while let Some(u) = deque.pop_front() {
        for &v in edges[u].iter() {
            if parent[v] == n {
                parent[v] = u;
                deque.push_back(v);
            }
        }
    }

    parent
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut edges_a = vec![Vec::new(); n];
    let mut edges_b = vec![Vec::new(); n];
    let mut scores = vec![0; n];

    for _ in 0..n - 1 {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        edges_a[a].push(b);
        edges_a[b].push(a);
    }

    for _ in 0..n - 1 {
        let (c, d) = (scan.token::<usize>(), scan.token::<usize>());
        edges_b[c].push(d);
        edges_b[d].push(c);
    }

    for i in 0..n {
        scores[i] = scan.token::<i64>();
    }

    let mut ret = i64::MIN / 4;

    for i in 0..n {
        let parent_a = parents_by_bfs(&edges_a, i);
        let parent_b = parents_by_bfs(&edges_b, i);
        let source = n;
        let sink = n + 1;
        let mut maximum_flow = Dinic::new(n + 2, source, sink);

        let mut pos_sum = 0;

        for v in 0..n {
            if scores[v] >= 0 {
                maximum_flow.add_edge(source, v, scores[v]);
                pos_sum += scores[v];
            } else {
                maximum_flow.add_edge(v, sink, -scores[v]);
            }
        }

        maximum_flow.add_edge(source, i, INF);

        for j in 0..n {
            if i == j {
                continue;
            }

            maximum_flow.add_edge(j, parent_a[j], INF);
            maximum_flow.add_edge(j, parent_b[j], INF);
        }

        let flow = maximum_flow.get_flow();
        ret = ret.max(pos_sum - flow);
    }

    writeln!(out, "{}", ret.max(0)).unwrap();
}

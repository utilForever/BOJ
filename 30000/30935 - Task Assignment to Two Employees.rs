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
    capacity: i128,
}

impl Edge {
    fn new(to: usize, rev: usize, capacity: i128) -> Self {
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

    fn add_edge(&mut self, u: usize, v: usize, capacity: i128) {
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

    fn process_dfs(&mut self, idx: usize, flow: i128) -> i128 {
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

    fn get_flow(&mut self) -> i128 {
        let mut flow_total = 0;

        while self.process_bfs() {
            self.done.fill(0);

            loop {
                let flow_current = self.process_dfs(self.source, i128::MAX);

                if flow_current == 0 {
                    break;
                }

                flow_total += flow_current;
            }
        }

        flow_total
    }
}

const INF: i128 = 4_000_000_000_000_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, p0) = (scan.token::<usize>(), scan.token::<i128>());
    let mut s1 = vec![0; n];
    let mut s2 = vec![0; n];
    let mut v1 = vec![0; n];
    let mut v2 = vec![0; n];

    for i in 0..n {
        s1[i] = scan.token::<i128>();
    }

    for i in 0..n {
        s2[i] = scan.token::<i128>();
    }

    for i in 0..n {
        v1[i] = scan.token::<i128>();
    }

    for i in 0..n {
        v2[i] = scan.token::<i128>();
    }

    let mut pairs1 = Vec::new();
    let mut pairs2 = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            let w1 = {
                let a = s1[i] * v1[j];
                let b = s1[j] * v1[i];

                a.max(b)
            };

            if w1 > 0 {
                pairs1.push((i, j, w1));
            }

            let w2 = {
                let a = s2[i] * v2[j];
                let b = s2[j] * v2[i];

                a.max(b)
            };

            if w2 > 0 {
                pairs2.push((i, j, w2));
            }
        }
    }

    let pairs1_len = pairs1.len();
    let pairs2_len = pairs2.len();
    let source = n + pairs1_len + pairs2_len;
    let sink = n + pairs1_len + pairs2_len + 1;
    let mut maximum_flow = Dinic::new(n + pairs1_len + pairs2_len + 2, source, sink);

    let mut base = 0;

    for i in 0..n {
        let c1 = p0 * v1[i];
        let c2 = p0 * v2[i];

        base += c1 + c2;

        maximum_flow.add_edge(source, i, c1);
        maximum_flow.add_edge(i, sink, c2);
    }

    for (idx, &(a, b, w)) in pairs1.iter().enumerate() {
        let p = n + idx;

        base += w;

        maximum_flow.add_edge(source, p, w);
        maximum_flow.add_edge(p, a, INF);
        maximum_flow.add_edge(p, b, INF);
    }

    for (idx, &(a, b, w)) in pairs2.iter().enumerate() {
        let q = n + pairs1_len + idx;

        base += w;

        maximum_flow.add_edge(a, q, INF);
        maximum_flow.add_edge(b, q, INF);
        maximum_flow.add_edge(q, sink, w);
    }

    let flow = maximum_flow.get_flow();

    writeln!(out, "{}", base - flow).unwrap();
}

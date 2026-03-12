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

fn get_bit(x: i64, i: usize) -> i64 {
    ((x as usize >> i) & 1) as i64
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut edges = Vec::with_capacity(m);

    for _ in 0..m {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        edges.push((u, v));
    }

    let mut values = vec![0; n];

    for i in 0..n {
        values[i] = scan.token::<i64>();
    }

    let q = scan.token::<usize>();
    let mut idxes = vec![vec![usize::MAX; 16]; n];
    let mut variables = Vec::new();
    let mut constraints = Vec::with_capacity(q);

    for _ in 0..q {
        let (t, u, i, v, j) = (
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if idxes[u][i] == usize::MAX {
            idxes[u][i] = variables.len();
            variables.push((u, i));
        }

        if idxes[v][j] == usize::MAX {
            idxes[v][j] = variables.len();
            variables.push((v, j));
        }

        constraints.push((t, idxes[u][i], idxes[v][j]));
    }

    let k = variables.len();
    let mut graph = vec![Vec::new(); k];

    for (t, a, b) in constraints {
        graph[a].push((b, t));
        graph[b].push((a, t));
    }

    let mut components = vec![usize::MAX; k];
    let mut diffs = vec![0; k];
    let mut root_pre = Vec::new();

    for i in 0..k {
        if components[i] != usize::MAX {
            continue;
        }

        components[i] = root_pre.len();

        let mut stack = Vec::new();
        let mut root = -1;

        stack.push(i);

        while let Some(u) = stack.pop() {
            let (node, b) = variables[u];

            if values[node] != -1 {
                let bit = get_bit(values[node], b) ^ diffs[u];

                if root == -1 {
                    root = bit;
                } else if root != bit {
                    writeln!(out, "-1").unwrap();
                    return;
                }
            }

            for &(v, w) in graph[u].iter() {
                let diff_next = diffs[u] ^ w;

                if components[v] == usize::MAX {
                    components[v] = components[u];
                    diffs[v] = diff_next;
                    stack.push(v);
                } else if diffs[v] != diff_next {
                    writeln!(out, "-1").unwrap();
                    return;
                }
            }
        }

        root_pre.push(root);
    }

    let c = root_pre.len();
    let mut roots = vec![0; c];
    let mut ret = INF;

    'outer: for mask in 0..1usize << c {
        for i in 0..c {
            roots[i] = if root_pre[i] != -1 {
                root_pre[i]
            } else {
                ((mask >> i) & 1) as i64
            };
        }

        let mut flow_sum = 0;

        for b in 0..16 {
            let source = n;
            let sink = n + 1;
            let mut maximum_flow = Dinic::new(n + 2, source, sink);

            for &(u, v) in edges.iter() {
                maximum_flow.add_edge(u, v, 1);
                maximum_flow.add_edge(v, u, 1);
            }

            for u in 0..n {
                let mut bit = if values[u] == -1 {
                    -1
                } else {
                    get_bit(values[u], b)
                };
                let idx = idxes[u][b];

                if idx != usize::MAX {
                    let val = roots[components[idx]] ^ diffs[idx];

                    if bit == -1 {
                        bit = val;
                    } else if bit != val {
                        continue 'outer;
                    }
                }

                if bit == 1 {
                    maximum_flow.add_edge(source, u, INF);
                } else if bit == 0 {
                    maximum_flow.add_edge(u, sink, INF);
                }
            }

            flow_sum += maximum_flow.get_flow();

            if flow_sum >= ret {
                continue 'outer;
            }
        }

        ret = ret.min(flow_sum);
    }

    writeln!(out, "{}", if ret == INF { -1 } else { ret }).unwrap();
}

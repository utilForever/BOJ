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
const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut scores = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            scores[i][j] = scan.token::<i64>();
        }
    }

    let cnt_x = if n >= 3 { (n - 2) * (n - 2) } else { 0 };
    let source = 0;
    let sink = n * n + cnt_x + 1;
    let mut maximum_flow = Dinic::new(n * n + cnt_x + 2, source, sink);

    let idx_cell = |i: usize, j: usize| -> usize { i * n + j + 1 };
    let idx_x = |x: usize| -> usize { n * n + x + 1 };

    let mut constant = 0;
    let mut weights = vec![0; n * n + cnt_x + 2];

    for i in 0..n {
        for j in 0..n {
            let idx = idx_cell(i, j);

            if (i + j) % 2 == 0 {
                weights[idx] = scores[i][j];
            } else {
                weights[idx] = -scores[i][j];
                constant += scores[i][j];
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if (i + j) % 2 != 0 {
                continue;
            }

            let u = idx_cell(i, j);

            for (dy, dx) in DIRECTIONS.iter() {
                let y_next = i as isize + dy;
                let x_next = j as isize + dx;

                if y_next < 0 || y_next >= n as isize || x_next < 0 || x_next >= n as isize {
                    continue;
                }

                let v = idx_cell(y_next as usize, x_next as usize);
                maximum_flow.add_edge(u, v, INF);
            }
        }
    }

    if n >= 3 {
        let mut cnt = 0;

        for i in 1..n - 1 {
            for j in 1..n - 1 {
                let idx = idx_x(cnt);
                cnt += 1;

                let cells = [
                    idx_cell(i - 1, j - 1),
                    idx_cell(i - 1, j + 1),
                    idx_cell(i, j),
                    idx_cell(i + 1, j - 1),
                    idx_cell(i + 1, j + 1),
                ];

                if (i + j) % 2 == 0 {
                    weights[idx] = k;

                    for &cell in cells.iter() {
                        maximum_flow.add_edge(idx, cell, INF);
                    }
                } else {
                    weights[idx] = -k;
                    constant += k;

                    for &cell in cells.iter() {
                        maximum_flow.add_edge(cell, idx, INF);
                    }
                }
            }
        }
    }

    let mut sum = 0;

    for i in 1..sink {
        if weights[i] > 0 {
            maximum_flow.add_edge(source, i, weights[i]);
            sum += weights[i];
        } else if weights[i] < 0 {
            maximum_flow.add_edge(i, sink, -weights[i]);
        }
    }

    let flow = maximum_flow.get_flow();

    writeln!(out, "{}", sum - flow + constant).unwrap();
}

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut grid = vec![vec![' '; m]; n];

    for i in 0..n {
        let line = scan.line().trim().to_string();

        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    let mut hings = Vec::new();
    let mut hings_center = vec![Vec::new(); n * m];
    let mut hings_sides = vec![Vec::new(); n * m];

    if n >= 3 {
        for i in 1..n - 1 {
            for j in 0..m {
                if grid[i - 1][j] != '^' && grid[i][j] != '-' && grid[i + 1][j] != '^' {
                    let idx = hings.len();
                    let left = (i - 1) * m + j;
                    let center = i * m + j;
                    let right = (i + 1) * m + j;

                    hings.push((i + j) % 2);
                    hings_center[center].push(idx);
                    hings_sides[left].push(idx);
                    hings_sides[right].push(idx);
                }
            }
        }
    }

    if m >= 3 {
        for i in 0..n {
            for j in 1..m - 1 {
                if grid[i][j - 1] != '^' && grid[i][j] != '-' && grid[i][j + 1] != '^' {
                    let idx = hings.len();
                    let left = i * m + j - 1;
                    let center = i * m + j;
                    let right = i * m + j + 1;

                    hings.push((i + j) % 2);
                    hings_center[center].push(idx);
                    hings_sides[left].push(idx);
                    hings_sides[right].push(idx);
                }
            }
        }
    }

    if hings.is_empty() {
        writeln!(out, "0").unwrap();
        return;
    }

    let source = 0;
    let sink = hings.len() + 1;
    let mut maximum_flow = Dinic::new(hings.len() + 2, source, sink);

    for i in 0..hings.len() {
        if hings[i] == 0 {
            maximum_flow.add_edge(source, i + 1, 1);
        } else {
            maximum_flow.add_edge(i + 1, sink, 1);
        }
    }

    for i in 0..n * m {
        for &a in hings_sides[i].iter() {
            for &b in hings_center[i].iter() {
                let (left, right) = if hings[a] == 0 {
                    (a + 1, b + 1)
                } else {
                    (b + 1, a + 1)
                };

                maximum_flow.add_edge(left, right, 1);
            }
        }
    }

    let flow = maximum_flow.get_flow();

    writeln!(out, "{}", hings.len() as i64 - flow).unwrap();
}

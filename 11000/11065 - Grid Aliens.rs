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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut hydrophilics = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                let h = scan.token::<String>();
                let (part_int, part_frac) = h.split_once('.').unwrap();

                let part_int = part_int.parse::<i64>().unwrap() * 100;
                let part_frac = if part_frac.len() == 1 {
                    part_frac.parse::<i64>().unwrap() * 10
                } else {
                    part_frac.parse::<i64>().unwrap()
                };

                hydrophilics[i][j] = part_int + part_frac;
            }
        }

        let source = n * n;
        let sink = n * n + 1;
        let mut maximum_flow = Dinic::new(n * n + 2, source, sink);

        for i in 0..n {
            for j in 0..n {
                maximum_flow.add_edge(source, i * n + j, hydrophilics[i][j]);
                maximum_flow.add_edge(i * n + j, sink, 100 - hydrophilics[i][j]);
            }
        }

        for i in 0..n {
            for j in 0..n {
                if i + 1 < n {
                    let u = (i + 1) * n + j;
                    let v = i * n + j;
                    let w = 100 - (hydrophilics[i + 1][j] - hydrophilics[i][j]).abs();

                    maximum_flow.add_edge(u, v, w);
                    maximum_flow.add_edge(v, u, w);
                }

                if j + 1 < n {
                    let u = i * n + (j + 1);
                    let v = i * n + j;
                    let w = 100 - (hydrophilics[i][j + 1] - hydrophilics[i][j]).abs();

                    maximum_flow.add_edge(u, v, w);
                    maximum_flow.add_edge(v, u, w);
                }
            }
        }

        let total = (n * n * 100) as i64;
        let flow = maximum_flow.get_flow();
        let ret = total - flow;

        writeln!(out, "{}.{:02}", ret / 100, ret % 100).unwrap();
    }
}

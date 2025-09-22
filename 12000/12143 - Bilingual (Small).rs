use io::Write;
use std::{
    collections::{HashMap, VecDeque},
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

const INF: i64 = 1_000_000_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut sentences = Vec::with_capacity(n);

        for _ in 0..n {
            let line = scan.line().trim().to_string();
            sentences.push(
                line.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
            );
        }

        let mut sentences_mapped = Vec::with_capacity(n);
        let mut idxes = HashMap::new();
        let mut idx = 0;

        for sentence in sentences.iter() {
            let mut idxes_local = Vec::with_capacity(sentence.len());

            for word in sentence {
                let idx_local = *idxes.entry(word).or_insert_with(|| {
                    let idx_curr = idx;
                    idx += 1;
                    idx_curr
                });

                idxes_local.push(idx_local);
            }

            idxes_local.sort_unstable();
            idxes_local.dedup();
            sentences_mapped.push(idxes_local);
        }

        let source = n + 2 * idx;
        let sink = n + 2 * idx + 1;
        let mut maximum_flow = Dinic::new(n + 2 * idx + 2, source, sink);

        for j in 0..idx {
            maximum_flow.add_edge(2 * j, 2 * j + 1, 1);
        }

        for (j, words) in sentences_mapped.iter().enumerate() {
            for word in words.iter() {
                maximum_flow.add_edge(2 * word + 1, 2 * idx + j, INF);
                maximum_flow.add_edge(2 * idx + j, 2 * word, INF);
            }
        }

        for word in sentences_mapped[0].iter() {
            maximum_flow.add_edge(source, 2 * word, INF);
        }

        for word in sentences_mapped[1].iter() {
            maximum_flow.add_edge(2 * word + 1, sink, INF);
        }

        writeln!(out, "Case #{i}: {}", maximum_flow.get_flow()).unwrap();
    }
}

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

fn check(
    investments: &Vec<i64>,
    build_times: &Vec<i64>,
    profits: &Vec<i64>,
    plants_required: &Vec<Vec<usize>>,
    t: i64,
    n: usize,
    m: usize,
) -> i64 {
    let source = 0;
    let sink = n + m + 1;
    let mut maximum_flow = Dinic::new(n + m + 2, source, sink);
    let profit_sum = profits.iter().sum::<i64>();

    for i in 0..m {
        maximum_flow.add_edge(source, i + 1, profits[i]);
    }

    for i in 0..n {
        maximum_flow.add_edge(
            m + i + 1,
            sink,
            if build_times[i] <= t {
                investments[i]
            } else {
                INF
            },
        );
    }

    for i in 0..m {
        for &required in plants_required[i].iter() {
            maximum_flow.add_edge(i + 1, m + required + 1, INF);
        }
    }

    let flow = maximum_flow.get_flow();

    profit_sum - flow
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, l) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut investments = vec![0; n];
    let mut build_times = vec![0; n];

    for i in 0..n {
        investments[i] = scan.token::<i64>();
        build_times[i] = scan.token::<i64>();
    }

    let mut profits = vec![0; m];
    let mut plants_required = vec![Vec::new(); m];

    for i in 0..m {
        profits[i] = scan.token::<i64>();

        let k = scan.token::<usize>();
        let mut required = vec![0; k];

        for j in 0..k {
            required[j] = scan.token::<usize>() - 1;
        }

        plants_required[i] = required;
    }

    let mut times = build_times.clone();

    times.push(0);
    times.sort_unstable();
    times.dedup();

    let profit_max = check(
        &investments,
        &build_times,
        &profits,
        &plants_required,
        *times.last().unwrap(),
        n,
        m,
    );

    if profit_max < l {
        writeln!(out, "impossible").unwrap();
        return;
    }

    let mut left = 0;
    let mut right = times.len() - 1;

    while left < right {
        let mid = (left + right) / 2;

        if check(
            &investments,
            &build_times,
            &profits,
            &plants_required,
            times[mid],
            n,
            m,
        ) >= l
        {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    writeln!(
        out,
        "{} {}",
        times[left],
        check(
            &investments,
            &build_times,
            &profits,
            &plants_required,
            times[left],
            n,
            m
        )
    )
    .unwrap();
}

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

fn mask_last(bits: &mut Vec<u64>, size: usize) {
    let rem = size % 64;

    if rem != 0 {
        let mask = (1u64 << rem) - 1;
        let last = bits.len() - 1;

        bits[last] &= mask;
    }
}

fn shift_or(bits: &mut Vec<u64>, shift: usize) {
    if shift == 0 {
        return;
    }

    let (word, bit) = (shift / 64, shift % 64);
    let len = bits.len();

    for i in (0..len).rev() {
        let mut val = 0;

        if i >= word {
            val = bits[i - word] << bit;

            if bit != 0 && i > word {
                val |= bits[i - word - 1] >> (64 - bit);
            }
        }

        bits[i] |= val;
    }
}

fn min_strength_diff(deg: &[usize], m: usize) -> i64 {
    let total = 2 * m;
    let size_bits = total + 1;
    let words = (size_bits + 63) / 64;
    let start = deg[0];

    let mut dp = vec![0; words];
    dp[start / 64] |= 1 << (start % 64);

    mask_last(&mut dp, size_bits);

    for i in 2..deg.len() {
        if deg[i] == 0 {
            continue;
        }

        shift_or(&mut dp, deg[i]);
        mask_last(&mut dp, size_bits);
    }

    let mut ret = i64::MAX;

    for s in 0..=total {
        if ((dp[s / 64] >> (s % 64)) & 1) != 0 {
            ret = ret.min((s as i64 - m as i64).abs());
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
        let mut maximum_flow = Dinic::new(n, 0, 1);
        let mut degree = vec![0; n];

        for _ in 0..m {
            let (u, v) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);

            maximum_flow.add_edge(u, v, 1);
            maximum_flow.add_edge(v, u, 1);

            degree[u] += 1;
            degree[v] += 1;
        }

        let flow = maximum_flow.get_flow();

        writeln!(
            out,
            "{} {}",
            m - flow,
            min_strength_diff(&degree, m as usize)
        )
        .unwrap();
    }
}

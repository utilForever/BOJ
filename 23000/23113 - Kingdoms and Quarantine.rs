use io::Write;
use std::{cmp::Reverse, collections::BinaryHeap, io, str};

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

#[derive(Clone)]
struct Edge {
    u: usize,
    v: usize,
    capacity: i64,
    cost: i64,
    f: i64,
}

pub struct MinCostFlow {
    adj: Vec<Vec<usize>>,
    edges: Vec<Edge>,
    cnt_edge: usize,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct MinCostFlowResult {
    pub flow: i64,
    pub cost: i64,
}

impl Default for MinCostFlow {
    fn default() -> Self {
        Self::new()
    }
}

impl MinCostFlow {
    pub fn new() -> Self {
        Self {
            adj: vec![],
            edges: vec![],
            cnt_edge: 0,
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, capacity: i64, cost: i64, bidirectional: bool) {
        if self.adj.len() < u + 1 {
            self.adj.resize(u + 1, vec![]);
        }

        if self.adj.len() < v + 1 {
            self.adj.resize(v + 1, vec![]);
        }

        self.adj[u].push(self.edges.len());
        self.edges.push(Edge {
            u,
            v,
            capacity,
            cost,
            f: 0,
        });
        self.adj[v].push(self.edges.len());
        self.edges.push(Edge {
            u: v,
            v: u,
            capacity: if bidirectional { capacity } else { 0 },
            cost: -cost,
            f: 0,
        });
        self.cnt_edge += 1;
    }

    pub fn solve(&mut self, s: usize, t: usize) -> Option<MinCostFlowResult> {
        let bound = s.max(t);

        if bound >= self.adj.len() {
            self.adj.resize(bound + 1, vec![]);
        }

        let n = self.adj.len();

        let mut dist = vec![(i64::MAX, usize::MAX); n];
        dist[s] = (0, usize::MAX);

        for i in 0..n + 1 {
            let mut updated = false;

            for u in 0..n {
                if dist[u].0 == i64::MAX {
                    continue;
                }

                for &id in self.adj[u].iter() {
                    if self.edges[id].capacity - self.edges[id].f > 0 {
                        let v = self.edges[id].v;
                        let new_dist = dist[u].0 + self.edges[id].cost;

                        if new_dist < dist[v].0 {
                            dist[v] = (new_dist, id);
                            updated = true;
                        }
                    }
                }
            }

            if !updated {
                break;
            } else if i == n {
                return None;
            }
        }

        let mut ret = MinCostFlowResult::default();

        loop {
            let mut dist_new = vec![(i64::MAX, usize::MAX); n];
            dist_new[s] = (dist[s].0, usize::MAX);

            let mut priority_queue = BinaryHeap::new();
            priority_queue.push((Reverse(dist_new[s].0), s));

            while let Some(x) = priority_queue.pop() {
                if x.0 .0 != dist_new[x.1].0 {
                    continue;
                }

                let u = x.1;

                for &id in self.adj[u].iter() {
                    if self.edges[id].capacity - self.edges[id].f > 0 {
                        let v = self.edges[id].v;
                        let new_dist = x.0 .0 + (dist[u].0 + self.edges[id].cost - dist[v].0);

                        if new_dist < dist_new[v].0 {
                            dist_new[v] = (new_dist, id);
                            priority_queue.push((Reverse(dist_new[v].0), v));
                        }
                    }
                }
            }

            for u in 0..n {
                if dist_new[u].0 != i64::MAX {
                    dist_new[u].0 -= dist[s].0 - dist[u].0;
                }
            }

            let mut x = dist_new[t];

            if x.0 == i64::MAX {
                break;
            }

            let mut flow = i64::MAX;

            while x.1 != usize::MAX {
                let id = x.1;
                flow = flow.min(self.edges[id].capacity - self.edges[id].f);
                x = dist_new[self.edges[id].u];
            }

            x = dist_new[t];

            let mut cost = 0;

            while x.1 != usize::MAX {
                let id = x.1;

                self.edges[id].f += flow;
                self.edges[id ^ 1].f -= flow;

                cost += self.edges[id].cost;
                x = dist_new[self.edges[id].u];
            }

            ret.flow += flow;
            ret.cost += cost * flow;

            // Update s_dist
            dist = dist_new;
        }

        Some(ret)
    }
}

const INF: i64 = 1_000_000_000;

// Reference: https://github.com/boj-rs/basm-rs/blob/main/basm-std/src/graph/mcmf.rs
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n1, n2, m) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    let mut a = vec![0; m];
    let mut b = vec![0; m];
    let mut deg = vec![0; n1 + n2];

    for i in 0..m {
        a[i] = scan.token::<usize>() - 1;
        b[i] = scan.token::<usize>() - 1;

        deg[a[i]] ^= 1;
        deg[b[i]] ^= 1;
    }

    let flow_source = n1 + n2;
    let flow_sink = n1 + n2 + 1;
    let demand_source = n1 + n2 + 2;
    let demand_sink = n1 + n2 + 3;
    let mut min_cost_flow = MinCostFlow::new();
    let mut demand = vec![0; n1 + n2 + 4];

    for i in 0..n1 {
        min_cost_flow.add_edge(flow_source, i, 1, 0, false);
    }

    for i in 0..n2 {
        min_cost_flow.add_edge(n1 + i, flow_sink, 1, 0, false);
    }

    min_cost_flow.add_edge(flow_sink, flow_source, INF, 0, false);

    let mut edge_idx = vec![0; m];

    for i in 0..m {
        let (u, v) = if deg[a[i]] == deg[b[i]] {
            (a[i], b[i])
        } else {
            (b[i], a[i])
        };

        demand[u] -= 1;
        demand[v] += 1;

        let id = min_cost_flow.edges.len();

        min_cost_flow.add_edge(v, u, 1, 1, false);
        edge_idx[i] = id;
    }

    for i in 0..n1 + n2 + 2 {
        if demand[i] > 0 {
            min_cost_flow.add_edge(demand_source, i, demand[i], 0, false);
        } else if demand[i] < 0 {
            min_cost_flow.add_edge(i, demand_sink, -demand[i], 0, false);
        }
    }

    let _ = min_cost_flow.solve(demand_source, demand_sink).unwrap();

    let mut selected = vec![false; m];
    let mut parity = vec![0; n1 + n2];

    for i in 0..m {
        if min_cost_flow.edges[edge_idx[i]].f == 0 {
            selected[i] = true;
            parity[a[i]] ^= 1;
            parity[b[i]] ^= 1;
        }
    }

    let mut visited = selected.clone();
    let mut nums = Vec::new();

    loop {
        let mut chosen = None;

        for i in 0..m {
            if !visited[i] {
                continue;
            }

            let val = deg[a[i]] ^ deg[b[i]] ^ 1;

            if parity[a[i]] == val && parity[b[i]] == val {
                chosen = Some(i);
                break;
            }
        }
        if let Some(idx) = chosen {
            visited[idx] = false;
            parity[a[idx]] ^= 1;
            parity[b[idx]] ^= 1;
            nums.push(idx);
        } else {
            break;
        }
    }

    nums.reverse();

    writeln!(out, "{}", nums.len()).unwrap();

    if !nums.is_empty() {
        for val in nums.iter() {
            write!(out, "{} ", val + 1).unwrap();
        }

        writeln!(out).unwrap();
    }
}

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
}

#[derive(Clone)]
struct Edge {
    cost: i64,
    flow: i64,
    to: usize,
    rev: usize,
}

impl Edge {
    fn new(cost: i64, flow: i64, to: usize, rev: usize) -> Self {
        Self {
            cost,
            flow,
            to,
            rev,
        }
    }
}

#[allow(dead_code)]
struct PushRelabel {
    size: usize,
    source: usize,
    sink: usize,
    eps: i64,
    graph: Vec<Vec<Edge>>,
    is_active: Vec<bool>,
    curr: Vec<usize>,
    excess_flow: Vec<i64>,
    potential: Vec<i64>,
    hs: Vec<Vec<usize>>,
    co: Vec<i64>,
}

impl PushRelabel {
    fn new(size: usize, source: usize, sink: usize) -> Self {
        Self {
            size,
            source,
            sink,
            eps: 0,
            graph: vec![Vec::new(); size],
            is_active: vec![false; size],
            curr: vec![0; size],
            excess_flow: vec![0; size],
            potential: vec![0; size],
            hs: vec![Vec::new(); size * 2],
            co: vec![0; size * 2],
        }
    }
}

impl PushRelabel {
    fn add_edge(&mut self, a: usize, b: usize, mut cost: i64, capacity: i64) {
        assert!(capacity >= 0);
        assert!(a < self.size && b < self.size);

        if a == b {
            assert!(cost >= 0);
            return;
        }

        cost *= self.size as i64;
        self.eps = self.eps.max(cost.abs());

        let a_len = self.graph[a].len();
        let b_len = self.graph[b].len();

        self.graph[a].push(Edge::new(cost, capacity, b, b_len));
        self.graph[b].push(Edge::new(-cost, 0, a, a_len));
    }

    fn add_flow(&mut self, edge: &mut Edge, flow: i64) {
        let back = &mut self.graph[edge.to][edge.rev];

        if self.excess_flow[edge.to] == 0 && flow != 0 {
            self.hs[self.potential[edge.to] as usize].push(edge.to);
        }

        edge.flow -= flow;
        self.excess_flow[edge.to] += flow;

        back.flow += flow;
        self.excess_flow[back.to] -= flow;
    }

    #[allow(dead_code)]
    fn push(&mut self, edge: &mut Edge, mut amount: i64) {
        if edge.flow < amount {
            amount = edge.flow;
        }

        edge.flow -= amount;
        self.excess_flow[edge.to] += amount;

        self.graph[edge.to][edge.rev].flow += amount;
        self.excess_flow[self.graph[edge.to][edge.rev].to] -= amount;
    }

    #[allow(dead_code)]
    fn relabel(&mut self, vertex: usize) {
        let mut potential_new = -i64::MAX / 2;

        for i in 0..self.graph[vertex].len() {
            let edge = &self.graph[vertex][i];

            if edge.flow != 0 && potential_new < self.potential[edge.to] - edge.cost {
                potential_new = self.potential[edge.to] - edge.cost;
                self.curr[vertex] = i;
            }
        }

        self.potential[vertex] = potential_new - self.eps;
    }

    fn get_max_flow(&mut self) -> i64 {
        self.curr.fill(0);
        self.excess_flow.fill(0);
        self.potential.fill(0);
        self.hs.resize(self.size * 2, Vec::new());
        self.co.fill(0);

        self.potential[self.source] = self.size as i64;
        self.excess_flow[self.sink] = 1;
        self.co[0] = self.size as i64 - 1;

        for i in 0..self.graph[self.source].len() {
            let edge: *mut Edge = &mut self.graph[self.source][i];
            self.add_flow(unsafe { &mut *edge }, unsafe { &mut *edge }.flow);
        }

        if self.hs[0].is_empty() {
            return -self.excess_flow[self.source];
        }

        let mut index = 0_i64;

        while index >= 0 {
            let u = *self.hs[index as usize].last().unwrap();
            self.hs[index as usize].pop();

            while self.excess_flow[u] > 0 {
                if self.curr[u] == self.graph[u].len() {
                    self.potential[u] = 10_i64.pow(9);

                    for i in 0..self.graph[u].len() {
                        let edge = &mut self.graph[u][i];

                        if edge.flow != 0 && self.potential[u] > self.potential[edge.to] + 1 {
                            self.potential[u] = self.potential[edge.to] + 1;
                            self.curr[u] = i;
                        }
                    }

                    self.co[self.potential[u] as usize] += 1;
                    self.co[index as usize] -= 1;

                    if self.co[index as usize] == 0 && index < self.size as i64 {
                        for i in 0..self.size {
                            if index < self.potential[i] && self.potential[i] < self.size as i64 {
                                self.co[self.potential[i] as usize] -= 1;
                                self.potential[i] = self.size as i64 + 1;
                            }
                        }
                    }

                    index = self.potential[u];
                } else if self.graph[u][self.curr[u]].flow != 0
                    && self.potential[u] == self.potential[self.graph[u][self.curr[u]].to] + 1
                {
                    let edge: *mut Edge = &mut self.graph[u][self.curr[u]];
                    self.add_flow(
                        unsafe { &mut *edge },
                        self.excess_flow[u].min(unsafe { &mut *edge }.flow),
                    );
                } else {
                    self.curr[u] += 1;
                }
            }

            while index >= 0 && self.hs[index as usize].is_empty() {
                index -= 1;
            }
        }

        -self.excess_flow[self.source]
    }

    fn _get_min_cost_max_flow(&mut self) -> (i64, i64) {
        let mut ret_cost = 0;

        for i in 0..self.size {
            for edge in self.graph[i].iter() {
                ret_cost += edge.cost * edge.flow;
            }
        }

        let ret_flow = self.get_max_flow();

        self.is_active.fill(false);
        self.curr.fill(0);
        self.excess_flow.fill(0);
        self.potential.fill(0);

        let mut queue = VecDeque::new();

        while self.eps > 0 {
            self.curr.fill(0);

            for i in 0..self.size {
                for j in 0..self.graph[i].len() {
                    if self.potential[i] + self.graph[i][j].cost
                        - self.potential[self.graph[i][j].to]
                        < 0
                        && self.graph[i][j].flow != 0
                    {
                        let edge: *mut Edge = &mut self.graph[i][j];
                        self.push(unsafe { &mut *edge }, unsafe { &mut *edge }.flow);
                    }
                }
            }

            for i in 0..self.size {
                if self.excess_flow[i] > 0 {
                    queue.push_back(i);
                    self.is_active[i] = true;
                }
            }

            while !queue.is_empty() {
                let u = queue.pop_front().unwrap();
                self.is_active[u] = false;

                while self.excess_flow[u] > 0 {
                    if self.curr[u] == self.graph[u].len() {
                        self.relabel(u);
                    }

                    let index_max = self.graph[u].len();

                    while self.curr[u] < index_max {
                        let edge: *mut Edge = &mut self.graph[u][self.curr[u]];

                        if self.potential[u] + unsafe { &mut *edge }.cost
                            - self.potential[unsafe { &mut *edge }.to]
                            < 0
                        {
                            self.push(unsafe { &mut *edge }, self.excess_flow[u]);

                            if self.excess_flow[unsafe { &mut *edge }.to] > 0
                                && !self.is_active[unsafe { &mut *edge }.to]
                            {
                                queue.push_back(unsafe { &mut *edge }.to);
                                self.is_active[unsafe { &mut *edge }.to] = true;
                            }

                            if self.excess_flow[u] == 0 {
                                break;
                            }
                        }

                        self.curr[u] += 1;
                    }
                }
            }

            if self.eps > 1 && self.eps >> 2 == 0 {
                self.eps = 1 << 2;
            }

            self.eps >>= 2;
        }

        for i in 0..self.size {
            for edge in self.graph[i].iter() {
                ret_cost -= edge.cost * edge.flow;
            }
        }

        (ret_flow, ret_cost / 2 / self.size as i64)
    }
}

struct LRFlow {
    flow: PushRelabel,
    sum_lower: i64,
}

impl LRFlow {
    fn new(size: usize, source: usize, sink: usize) -> Self {
        Self {
            flow: PushRelabel::new(size, source, sink),
            sum_lower: 0,
        }
    }

    fn add_edge(&mut self, a: usize, b: usize, l: i64, r: i64) {
        // 1. Add edge from source_new to b with cappcity l
        // 2. Add edge from a to sink_new with capacity l
        // 3. Add edge from a to b with capacity r - l
        self.flow.add_edge(self.flow.source, b, 0, l);
        self.flow.add_edge(a, self.flow.sink, 0, l);
        self.flow.add_edge(a, b, 0, r - l);

        self.sum_lower += l;
    }

    fn max_flow(&mut self, a: usize, b: usize) {
        // 4. Add edge from sink_old to source_old with capacity INF
        self.flow.add_edge(b, a, 0, i64::MAX);
        assert!(self.sum_lower == self.flow.get_max_flow());
    }

    fn get_value(&self, a: usize, b: usize) -> i64 {
        for edge in self.flow.graph[a].iter() {
            if edge.to == b {
                return edge.flow;
            }
        }

        0
    }
}

// Reference: https://ryulstory.tistory.com/49
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut table = vec![vec![0.0; m + 1]; n + 1];
    let mut sum_row = vec![0.0; n + 1];
    let mut sum_col = vec![0.0; m + 1];

    for i in 1..=n {
        for j in 1..=m {
            table[i][j] = scan.token::<f64>();
        }

        sum_row[i] = scan.token::<f64>();
    }

    for j in 1..=m {
        sum_col[j] = scan.token::<f64>();
    }

    // NOTE: source_old = 0, sink_old = n + m + 1
    //       source_new = n + m + 2, sink_new = n + m + 3
    let mut lr_flow = LRFlow::new(n + m + 4, n + m + 2, n + m + 3);

    for i in 1..=n {
        lr_flow.add_edge(0, i, sum_row[i].floor() as i64, sum_row[i].ceil() as i64);
    }

    for i in 1..=m {
        lr_flow.add_edge(
            n + i,
            n + m + 1,
            sum_col[i].floor() as i64,
            sum_col[i].ceil() as i64,
        );
    }

    for i in 1..=n {
        for j in 1..=m {
            lr_flow.add_edge(
                i,
                n + j,
                table[i][j].floor() as i64,
                table[i][j].ceil() as i64,
            );
        }
    }

    lr_flow.max_flow(0, n + m + 1);

    for i in 1..=n {
        for j in 1..=m {
            write!(
                out,
                "{} ",
                table[i][j].ceil() as i64 - lr_flow.get_value(i, n + j)
            )
            .unwrap();
        }

        writeln!(
            out,
            "{}",
            sum_row[i].ceil() as i64 - lr_flow.get_value(0, i)
        )
        .unwrap();
    }

    for i in 1..=m {
        write!(
            out,
            "{} ",
            sum_col[i].ceil() as i64 - lr_flow.get_value(n + i, n + m + 1)
        )
        .unwrap();
    }
}

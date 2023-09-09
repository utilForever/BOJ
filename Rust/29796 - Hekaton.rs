use io::Write;
use std::{cell::RefCell, collections::VecDeque, io, rc::Rc, str};

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

struct Edge {
    dest: Option<Rc<RefCell<Edge>>>,
    to: usize,
    capacity: i64,
}

impl Edge {
    fn new(to: usize, capacity: i64) -> Self {
        Self {
            dest: None,
            to,
            capacity,
        }
    }
}

struct MaximumFlow {
    graph: Vec<Vec<Rc<RefCell<Edge>>>>,
    source: usize,
    sink: usize,
    check: Vec<i64>,
    done: Vec<usize>,
}

impl MaximumFlow {
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
        let orig = Rc::new(RefCell::new(Edge::new(v, capacity)));
        let dest = Rc::new(RefCell::new(Edge::new(u, 0)));

        orig.as_ref().borrow_mut().dest = Some(dest.clone());
        dest.as_ref().borrow_mut().dest = Some(orig.clone());

        self.graph[u].push(orig);
        self.graph[v].push(dest);
    }

    fn process_bfs(&mut self) -> bool {
        self.check.fill(-1);

        let mut queue = VecDeque::new();
        queue.push_back(self.source);
        self.check[self.source] = 0;

        while !queue.is_empty() {
            let val = queue.pop_front().unwrap();

            for i in 0..self.graph[val].len() {
                let edge = self.graph[val][i].borrow();

                if edge.capacity <= 0 || self.check[edge.to] != -1 {
                    continue;
                }

                queue.push_back(edge.to);
                self.check[edge.to] = self.check[val] + 1;
            }
        }

        self.check[self.sink] != -1
    }

    fn process_dfs(&mut self, idx: usize, flow: i64) -> i64 {
        if idx == self.sink || flow == 0 {
            return flow;
        }

        while self.done[idx] < self.graph[idx].len() {
            let (to, capacity) = {
                let edge = self.graph[idx][self.done[idx]].borrow();
                (edge.to, edge.capacity)
            };

            if self.check[to] <= self.check[idx] {
                self.done[idx] += 1;
                continue;
            }

            let flow_current = self.process_dfs(to, capacity.min(flow));

            if flow_current > 0 {
                let edge = &mut self.graph[idx][self.done[idx]].borrow_mut();
                edge.capacity -= flow_current;
                unsafe {
                    (*edge.dest.as_ref().unwrap().as_ref().as_ptr()).capacity += flow_current;
                }

                return flow_current;
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

fn is_satisfy(
    characters: &Vec<i64>,
    edges: &Vec<(usize, usize)>,
    weaknesses: &Vec<bool>,
    n: usize,
    r: usize,
    target: i64,
) -> bool {
    let mut maximum_flow = MaximumFlow::new(2 * n + 2, 0, 2 * n + 1);

    for i in 0..n {
        maximum_flow.add_edge(0, i + 1, characters[i]);
        maximum_flow.add_edge(i + 1, n + i + 1, i64::MAX);
        maximum_flow.add_edge(n + i + 1, 2 * n + 1, if weaknesses[i] { target } else { 0 });
    }

    for (u, v) in edges.iter() {
        maximum_flow.add_edge(u + 1, n + v + 1, i64::MAX);
        maximum_flow.add_edge(v + 1, n + u + 1, i64::MAX);
    }

    maximum_flow.get_flow() == target * r as i64
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut characters = vec![0; n];

    for i in 0..n {
        characters[i] = scan.token::<i64>();
    }

    let mut edges = vec![(0, 0); m];

    for i in 0..m {
        edges[i] = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
    }

    let r = scan.token::<usize>();
    let mut weaknesses = vec![false; n];

    for _ in 0..r {
        let b = scan.token::<usize>() - 1;
        weaknesses[b] = true;
    }

    let mut left = 0;
    let mut right = 10_000_001;

    while left < right {
        let mid = (left + right + 1) / 2;

        if is_satisfy(&characters, &edges, &weaknesses, n, r, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    writeln!(out, "{left}").unwrap();
}

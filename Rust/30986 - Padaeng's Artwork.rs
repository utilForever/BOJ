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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let prefix = [[1, 6, 7, 8], [1, 6, 9, 10], [1, 7, 9, 11], [1, 8, 10, 11]];
    let mut maximum_flow = MaximumFlow::new(18, 0, 17);
    let mut capacities = [[0; 18]; 18];

    for i in 1..=16 {
        if i % 10 >= 2 && i % 10 <= 5 {
            capacities[0][i] = 1;
        } else {
            capacities[i][17] = 1;
        }
    }

    for i in 0..4 {
        for j in 0..4 {
            capacities[i + 2][prefix[i][j]] = 1;
            capacities[15 - i][17 - prefix[i][j]] = 1;
        }
    }

    let n = scan.token::<i64>();

    for _ in 0..n {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

        if a % 10 >= 2 && a % 10 <= 5 {
            capacities[a][b] = 0;
        } else {
            capacities[b][a] = 0;
        }
    }

    for i in 0..=17 {
        for j in 0..=17 {
            if capacities[i][j] == 1 {
                maximum_flow.add_edge(i, j, 1);
            }
        }
    }

    writeln!(out, "{}", 16 - maximum_flow.get_flow()).unwrap();
}

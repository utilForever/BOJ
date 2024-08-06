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

    let dx = [0, 0, 1, -1];
    let dy = [1, -1, 0, 0];

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut city = vec![vec!['0'; m]; n];

    for i in 0..n {
        let s = scan.token::<String>();
        let mut j = 0;

        for ch in s.chars() {
            city[i][j] = ch;
            j += 1;
        }
    }

    let mut x_start = 0;
    let mut x_end = 0;
    let mut y_start = 0;
    let mut y_end = 0;

    for i in 0..n {
        for j in 0..m {
            if city[i][j] == 'K' {
                y_start = i;
                x_start = j;
            } else if city[i][j] == 'H' {
                y_end = i;
                x_end = j;
            }
        }
    }

    let mut maximum_flow = MaximumFlow::new(
        (n * m) * 2,
        (y_start * m + x_start) * 2 + 1,
        (y_end * m + x_end) * 2,
    );

    for i in 0..n {
        for j in 0..m {
            if city[i][j] == '#' {
                continue;
            }

            maximum_flow.add_edge((i * m + j) * 2, (i * m + j) * 2 + 1, 1);

            for k in 0..4 {
                let y_new = i as i64 + dy[k];
                let x_new = j as i64 + dx[k];

                if y_new < 0 || y_new >= n as i64 || x_new < 0 || x_new >= m as i64 {
                    continue;
                }

                let y_new = y_new as usize;
                let x_new = x_new as usize;

                if city[y_new][x_new] != '#' {
                    maximum_flow.add_edge((i * m + j) * 2 + 1, (y_new * m + x_new) * 2, 1_000_000);
                }
            }
        }
    }

    let mut ret = maximum_flow.get_flow();

    if ret >= 1_000_000 {
        ret = -1;
    }

    writeln!(out, "{ret}").unwrap();
}

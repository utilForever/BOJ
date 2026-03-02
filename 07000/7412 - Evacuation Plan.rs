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

struct Edge {
    dest: Option<Rc<RefCell<Edge>>>,
    to: usize,
    capacity: i64,
    cost: i64,
}

impl Edge {
    fn new(to: usize, capacity: i64, cost: i64) -> Self {
        Self {
            dest: None,
            to,
            capacity,
            cost,
        }
    }
}

struct MCMF {
    graph: Vec<Vec<Rc<RefCell<Edge>>>>,
    from: Vec<(i64, i64)>,
    check: Vec<bool>,
    distance: Vec<i64>,
    source: usize,
    sink: usize,
}

impl MCMF {
    fn new(n: usize, source: usize, sink: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n as usize],
            from: vec![(-1, -1); n as usize],
            check: vec![false; n as usize],
            distance: vec![0; n as usize],
            source,
            sink,
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, capacity: i64, cost: i64) {
        let orig = Rc::new(RefCell::new(Edge::new(v, capacity, cost)));
        let dest = Rc::new(RefCell::new(Edge::new(u, 0, -cost)));

        orig.as_ref().borrow_mut().dest = Some(dest.clone());
        dest.as_ref().borrow_mut().dest = Some(orig.clone());

        self.graph[u].push(orig);
        self.graph[v].push(dest);
    }

    fn process_spfa(&mut self, total_flow: &mut i64, total_cost: &mut i64) -> bool {
        self.from.fill((-1, -1));
        self.check.fill(false);
        self.distance.fill(std::i64::MAX);

        self.distance[self.source] = 0;

        let mut queue = VecDeque::new();
        queue.push_back(self.source);

        while !queue.is_empty() {
            let val = queue.pop_front().unwrap();
            self.check[val] = false;

            for i in 0..self.graph[val].len() {
                let edge = &self.graph[val][i];
                let edge_to = edge.as_ref().borrow().to as usize;

                if edge.as_ref().borrow().capacity > 0
                    && self.distance[val] + edge.as_ref().borrow().cost < self.distance[edge_to]
                {
                    self.distance[edge_to] = self.distance[val] + edge.as_ref().borrow().cost;
                    self.from[edge_to] = (val as i64, i as i64);

                    if !self.check[edge_to] {
                        self.check[edge_to] = true;
                        queue.push_back(edge_to);
                    }
                }
            }
        }

        if self.distance[self.sink] == std::i64::MAX {
            return false;
        }

        let mut x = self.sink;
        let mut capacity = self.graph[self.from[x].0 as usize][self.from[x].1 as usize]
            .as_ref()
            .borrow()
            .capacity;

        while self.from[x].0 != -1 {
            if capacity
                > self.graph[self.from[x].0 as usize][self.from[x].1 as usize]
                    .as_ref()
                    .borrow()
                    .capacity
            {
                capacity = self.graph[self.from[x].0 as usize][self.from[x].1 as usize]
                    .as_ref()
                    .borrow()
                    .capacity;
            }

            x = self.from[x].0 as usize;
        }

        x = self.sink;

        while self.from[x].0 != -1 {
            let edge =
                &mut self.graph[self.from[x].0 as usize][self.from[x].1 as usize].borrow_mut();
            edge.capacity -= capacity;
            unsafe {
                (*edge.dest.as_ref().unwrap().as_ref().as_ptr()).capacity += capacity;
            }

            x = self.from[x].0 as usize;
        }

        *total_flow += capacity;
        *total_cost += capacity * self.distance[self.sink];

        true
    }

    fn get_flow(&mut self) -> (i64, i64) {
        let mut total_flow = 0;
        let mut total_cost = 0;

        while self.process_spfa(&mut total_flow, &mut total_cost) {}
        (total_flow, total_cost)
    }
}

const INF: i64 = 1_000_000_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut buildings = vec![(0, 0, 0); n];
    let mut shelters = vec![(0, 0, 0); m];
    let mut plans = vec![vec![0; m]; n];
    let mut times = vec![vec![0; m]; n];
    let mut cost_council = 0;

    for i in 0..n {
        buildings[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    for i in 0..m {
        shelters[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    for i in 0..n {
        for j in 0..m {
            plans[i][j] = scan.token::<i64>();
        }
    }

    for i in 0..n {
        for j in 0..m {
            times[i][j] =
                (buildings[i].0 - shelters[j].0).abs() + (buildings[i].1 - shelters[j].1).abs() + 1;
            cost_council += plans[i][j] * times[i][j];
        }
    }

    let source = 0;
    let sink = n + m + 1;
    let mut mcmf = MCMF::new(n + m + 2, source, sink);

    for i in 0..n {
        mcmf.add_edge(source, i + 1, buildings[i].2, 0);
    }

    for i in 0..m {
        mcmf.add_edge(n + 1 + i, sink, shelters[i].2, 0);
    }

    let mut idx_edge = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            idx_edge[i][j] = mcmf.graph[i + 1].len();
            mcmf.add_edge(i + 1, n + 1 + j, INF, times[i][j]);
        }
    }

    let (_, cost) = mcmf.get_flow();

    if cost == cost_council {
        writeln!(out, "OPTIMAL").unwrap();
        return;
    }

    writeln!(out, "SUBOPTIMAL").unwrap();

    for i in 0..n {
        for j in 0..m {
            let capacity = mcmf.graph[i + 1][idx_edge[i][j]].borrow().capacity;
            write!(out, "{} ", INF - capacity).unwrap();
        }

        writeln!(out).unwrap();
    }
}

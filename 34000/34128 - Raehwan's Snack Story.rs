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

    let n = scan.token::<usize>();
    let (a, b, c) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut preferences = [0; 8];

    for _ in 0..n {
        let (sa, sb, sc) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        preferences[(sa | sb << 1 | sc << 2) as usize] += 1;
    }

    let mut ret = preferences[0];

    for i in 1..8 {
        let req_a = if i & 1 != 0 { a } else { 0 };
        let req_b = if i & 2 != 0 { b } else { 0 };
        let req_c = if i & 4 != 0 { c } else { 0 };

        let mut cnt_exclusive_like = 0;

        for j in 1..8 {
            if (!i & j) == 0 {
                cnt_exclusive_like += preferences[j];
            }
        }

        let mut id_preferences = [None; 8];
        let mut id = 1;

        for j in 1..8 {
            if preferences[j] > 0 && (i & j) != 0 {
                id_preferences[j] = Some(id);
                id += 1;
            }
        }

        let id_a = id;
        let id_b = id + 1;
        let id_c = id + 2;
        let id_sink = id + 3;

        let mut mcmf = MCMF::new(id_sink + 1, 0, id_sink);

        for j in 1..8 {
            if let Some(pid) = id_preferences[j] {
                let is_exclusive_like = (!i & j) == 0;
                let cost = if is_exclusive_like { 1 } else { 0 };

                mcmf.add_edge(0, pid, preferences[j], cost);

                if j & 1 != 0 && req_a > 0 {
                    mcmf.add_edge(pid, id_a, INF, 0);
                }

                if j & 2 != 0 && req_b > 0 {
                    mcmf.add_edge(pid, id_b, INF, 0);
                }

                if j & 4 != 0 && req_c > 0 {
                    mcmf.add_edge(pid, id_c, INF, 0);
                }
            }
        }

        if req_a > 0 {
            mcmf.add_edge(id_a, id_sink, req_a, 0);
        }

        if req_b > 0 {
            mcmf.add_edge(id_b, id_sink, req_b, 0);
        }

        if req_c > 0 {
            mcmf.add_edge(id_c, id_sink, req_c, 0);
        }

        let (flow, cost) = mcmf.get_flow();

        if flow == req_a + req_b + req_c {
            let val = preferences[0] + cnt_exclusive_like - cost;
            ret = ret.max(val);
        }
    }

    writeln!(out, "{ret}").unwrap();
}

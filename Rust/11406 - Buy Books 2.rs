use std::{cell::RefCell, collections::VecDeque, io, rc::Rc};

fn input_integers() -> Vec<i64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
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

struct MCMF {
    graph: Vec<Vec<Rc<RefCell<Edge>>>>,
    from: Vec<(i64, i64)>,
    check: Vec<usize>,
    step: usize,
    source: usize,
    sink: usize,
}

impl MCMF {
    fn new(n: usize, source: usize, sink: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n as usize],
            from: vec![(-1, -1); n as usize],
            check: vec![0; n as usize],
            step: 0,
            source,
            sink,
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

    fn add_edge_from_source(&mut self, v: usize, capacity: i64) {
        self.add_edge(self.source, v, capacity);
    }

    fn add_edge_to_sink(&mut self, u: usize, capacity: i64) {
        self.add_edge(u, self.sink, capacity);
    }

    fn process_dfs(&mut self) -> i64 {
        let mut queue = VecDeque::new();
        queue.push_back(self.source);

        self.check[self.source] = self.step;

        while !queue.is_empty() {
            let val = queue.pop_front().unwrap();

            for i in 0..self.graph[val].len() {
                if self.graph[val][i].borrow().capacity > 0
                    && self.check[self.graph[val][i].borrow().to] != self.step
                {
                    queue.push_back(self.graph[val][i].borrow().to);
                    self.check[self.graph[val][i].borrow().to] = self.step;
                    self.from[self.graph[val][i].borrow().to] = (val as i64, i as i64);
                }
            }
        }

        if self.check[self.sink] != self.step {
            return 0;
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

        capacity
    }

    fn get_flow(&mut self) -> i64 {
        let mut total_flow = 0;

        loop {
            self.step += 1;

            let res = self.process_dfs();

            if res == 0 {
                break;
            }

            total_flow += res;
        }

        total_flow
    }
}

fn main() {
    let nums = input_integers();
    let (n, m) = (nums[0] as usize, nums[1] as usize);

    let mut mcmf = MCMF::new(n + m + 2, n + m, n + m + 1);

    let a = input_integers();

    for i in 0..n {
        mcmf.add_edge_to_sink(m + i, a[i]);
    }

    let b = input_integers();

    for i in 0..m {
        mcmf.add_edge_from_source(i, b[i]);
    }

    for i in 0..m {
        let c = input_integers();

        for j in 0..n {
            mcmf.add_edge(i, j + m, c[j]);
        }
    }

    let total_flow = mcmf.get_flow();
    println!("{}", total_flow);
}

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

    fn add_edge_from_source(&mut self, v: usize, capacity: i64, cost: i64) {
        self.add_edge(self.source, v, capacity, cost);
    }

    fn add_edge_to_sink(&mut self, u: usize, capacity: i64, cost: i64) {
        self.add_edge(u, self.sink, capacity, cost);
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

fn main() {
    let nums = input_integers();
    let (n, m) = (nums[0] as usize, nums[1] as usize);

    let mut mcmf = MCMF::new(n + m + 2, n + m, n + m + 1);

    for i in 0..n {
        mcmf.add_edge_from_source(i, 1, 0);
    }

    for i in 0..m {
        mcmf.add_edge_to_sink(i + n, 1, 0);
    }

    for i in 0..n {
        let nums = input_integers();
        let num_works = nums[0] as usize;

        for j in 0..num_works {
            let (work, salary) = (nums[j * 2 + 1], nums[j * 2 + 2]);
            mcmf.add_edge(i, work as usize + n - 1, 1, -salary);
        }
    }

    let (total_flow, total_cost) = mcmf.get_flow();
    println!("{}", total_flow);
    println!("{}", -total_cost);
}

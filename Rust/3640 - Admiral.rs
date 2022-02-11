use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    io,
    rc::Rc,
};

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
    potential: Vec<i64>,
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
            potential: vec![0; n as usize],
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

    fn process_spfa(&mut self) {
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

        for i in 0..self.potential.len() {
            self.potential[i] = self.distance[i];
        }
    }

    fn process_dijkstra(&mut self, total_flow: &mut i64, total_cost: &mut i64) -> bool {
        self.check.fill(false);
        self.distance.fill(std::i64::MAX);
        self.from.fill((-1, -1));

        self.distance[self.source] = 0;

        let mut priority_queue: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        priority_queue.push(Reverse((0, self.source)));

        while !priority_queue.is_empty() {
            let val = priority_queue.pop().unwrap().0.1;

            if self.check[val] {
                continue;
            }

            self.check[val] = true;

            if val == self.sink {
                continue;
            }

            for i in 0..self.graph[val].len() {
                let edge = &self.graph[val][i];
                let edge_to = edge.as_ref().borrow().to as usize;

                if edge.as_ref().borrow().capacity > 0
                    && self.distance[edge_to]
                        > self.distance[val] + edge.as_ref().borrow().cost - self.potential[edge_to]
                            + self.potential[val]
                {
                    self.distance[edge_to] = self.distance[val] + edge.as_ref().borrow().cost
                        - self.potential[edge_to]
                        + self.potential[val];
                    self.from[edge_to] = (val as i64, i as i64);
                    priority_queue.push(Reverse((self.distance[edge_to], edge_to)));
                }
            }
        }

        if self.distance[self.sink] == std::i64::MAX {
            return false;
        }

        for i in 0..self.potential.len() {
            if self.distance[i] != std::i64::MAX {
                self.potential[i] += self.distance[i];
            }
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

            *total_cost += edge.cost * capacity;
            x = self.from[x].0 as usize;
        }

        *total_flow += capacity;

        true
    }

    fn get_flow(&mut self) -> (i64, i64) {
        let mut total_flow = 0;
        let mut total_cost = 0;

        self.process_spfa();
        while self.process_dijkstra(&mut total_flow, &mut total_cost) {}
        (total_flow, total_cost)
    }
}

fn main() {
    loop {
        let nums = input_integers();
        if nums.is_empty() {
            break;
        }

        let (v, e) = (nums[0] as usize, nums[1] as usize);
        let mut mcmf = MCMF::new(2 * v + 2, 2 * v, 2 * v + 1);

        for _ in 0..e {
            let nums = input_integers();
            let (a, b, c) = (nums[0] as usize, nums[1] as usize, nums[2]);

            mcmf.add_edge(2 * (a - 1) + 1, 2 * (b - 1), 1, c);
        }

        mcmf.add_edge_from_source(0, 2, 0);
        mcmf.add_edge_to_sink(2 * (v - 1) + 1, 2, 0);

        for i in 0..v {
            if i == 0 || i == v - 1 {
                mcmf.add_edge(2 * i, 2 * i + 1, 2, 0);
            } else {
                mcmf.add_edge(2 * i, 2 * i + 1, 1, 0);
            }
        }

        let (_, total_cost) = mcmf.get_flow();
        println!("{}", total_cost);
    }
}

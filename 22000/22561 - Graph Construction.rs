use io::Write;
use std::{io, str};

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

pub struct GroupBy<'a, T, P> {
    slice: &'a [T],
    predicate: P,
}

pub trait GroupByTrait<T> {
    fn group_by<P>(&self, p: P) -> GroupBy<T, P>
    where
        P: FnMut(&T, &T) -> bool;
}

impl<T> GroupByTrait<T> for [T] {
    fn group_by<P>(&self, p: P) -> GroupBy<T, P>
    where
        P: FnMut(&T, &T) -> bool,
    {
        GroupBy {
            slice: self,
            predicate: p,
        }
    }
}

impl<'a, T, P> Iterator for GroupBy<'a, T, P>
where
    P: FnMut(&T, &T) -> bool,
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None;
        }

        let mut next = 1;

        while next != self.slice.len() && (self.predicate)(&self.slice[next - 1], &self.slice[next])
        {
            next += 1;
        }

        let ret = &self.slice[0..next];

        self.slice = &self.slice[next..];

        Some(ret)
    }
}

pub trait DsuNodeTrait: Copy + Clone + Default {
    fn join(lhs: &Self, rhs: &Self) -> Self;
}

pub struct DsuEvent<T: DsuNodeTrait> {
    idx: usize,
    size: usize,
    parent: usize,
    node: T,
}

pub struct DsuWithRollback<T: DsuNodeTrait> {
    size: Vec<usize>,
    parent: Vec<usize>,
    events: Vec<DsuEvent<T>>,
    nodes: Vec<T>,
}

impl<T: DsuNodeTrait> DsuWithRollback<T> {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect::<Vec<_>>();
        let nodes = vec![T::default(); n];

        Self {
            size: vec![1; n],
            parent,
            events: Vec::new(),
            nodes,
        }
    }

    pub fn len(&self) -> usize {
        self.parent.len()
    }

    pub fn get(&self, mut v: usize) -> usize {
        while self.parent[v] != v {
            v = self.parent[v];
        }

        v
    }

    pub fn get_current_time(&self) -> usize {
        self.events.len()
    }

    pub fn get_node(&self, mut v: usize) -> &T {
        v = self.get(v);
        &self.nodes[v]
    }

    pub fn set_node(&mut self, v: usize, node: T) {
        self.save(v);
        self.nodes[v] = node;
    }

    pub fn rollback(&mut self, time: usize) {
        while self.events.len() != time {
            let event = self.events.pop().unwrap();

            self.size[event.idx] = event.size;
            self.parent[event.idx] = event.parent;
            self.nodes[event.idx] = event.node;
        }
    }

    pub fn save(&mut self, idx: usize) {
        self.events.push(DsuEvent {
            idx,
            size: self.size[idx],
            parent: self.parent[idx],
            node: self.nodes[idx],
        });
    }

    pub fn unite(&mut self, mut v: usize, mut u: usize) {
        v = self.get(v);
        u = self.get(u);

        if v == u {
            return;
        }

        let (smaller, larger) = if self.size[u] < self.size[v] {
            (u, v)
        } else {
            (v, u)
        };

        self.save(smaller);
        self.save(larger);

        self.size[larger] += self.size[smaller];
        self.parent[smaller] = larger;
        self.nodes[larger] = T::join(&self.nodes[smaller], &self.nodes[larger]);
    }
}

#[derive(Clone, Copy, Debug)]
struct EdgeChange {
    time: usize,
    from: usize,
    to: usize,
    delta: i64,
}

#[derive(Clone, Copy, Debug)]
struct EdgePresent {
    from: usize,
    to: usize,
    time_from_inclusive: usize,
    time_to_exclusive: usize,
}

pub struct DynamicConnectivityOffline<T: DsuNodeTrait, Q: Copy + Default> {
    dsu: DsuWithRollback<T>,
    queries: Vec<Q>,
    events: Vec<EdgeChange>,
}

impl<T: DsuNodeTrait, Q: Copy + Default> DynamicConnectivityOffline<T, Q> {
    pub fn new(n: usize) -> Self {
        Self {
            dsu: DsuWithRollback::new(n),
            queries: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn get_dsu_mut(&mut self) -> &mut DsuWithRollback<T> {
        &mut self.dsu
    }

    pub fn add_query(&mut self, query: Q) {
        self.queries.push(query);
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.events.push(EdgeChange {
            time: self.queries.len(),
            from,
            to,
            delta: 1,
        });
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) {
        self.events.push(EdgeChange {
            time: self.queries.len(),
            from,
            to,
            delta: -1,
        });
    }

    fn generate_edge_events(&mut self) -> Vec<EdgePresent> {
        let mut ret = Vec::new();

        self.events
            .sort_by_key(|e| (e.from, e.to, e.time, -e.delta));

        for group in self
            .events
            .group_by(|e1, e2| e1.from == e2.from && e1.to == e2.to)
        {
            let mut balance = 0;
            let mut cur_start = 0;

            for ev in group.iter() {
                balance += ev.delta;

                if balance == 1 && ev.delta == 1 {
                    cur_start = ev.time;
                }

                if balance == 0 {
                    ret.push(EdgePresent {
                        from: ev.from,
                        to: ev.to,
                        time_from_inclusive: cur_start,
                        time_to_exclusive: ev.time,
                    })
                }
            }

            if balance > 0 {
                ret.push(EdgePresent {
                    from: group[0].from,
                    to: group[0].to,
                    time_from_inclusive: cur_start,
                    time_to_exclusive: self.queries.len(),
                })
            }
        }

        ret
    }

    pub fn run<F>(&mut self, callback: &mut F)
    where
        F: FnMut(&Q, &DsuWithRollback<T>),
    {
        let events = self.generate_edge_events();
        self.run_rec(0, self.queries.len(), events, callback);
    }

    fn run_rec<F>(&mut self, left: usize, right: usize, events: Vec<EdgePresent>, callback: &mut F)
    where
        F: FnMut(&Q, &DsuWithRollback<T>),
    {
        let mut ev_left = vec![];
        let mut ev_right = vec![];

        let mid = (left + right) / 2;
        let cur_time = self.dsu.get_current_time();

        for ev in events.iter() {
            if ev.time_from_inclusive <= left && ev.time_to_exclusive >= right {
                self.dsu.unite(ev.from, ev.to);
            } else {
                if ev.time_from_inclusive < mid {
                    ev_left.push(*ev);
                }

                if ev.time_to_exclusive > mid {
                    ev_right.push(*ev);
                }
            }
        }

        if left + 1 == right {
            callback(&self.queries[left], &self.dsu);
        } else {
            self.run_rec(left, mid, ev_left, callback);
            self.run_rec(mid, right, ev_right, callback);
        }

        self.dsu.rollback(cur_time);
    }
}

impl DsuNodeTrait for () {
    fn join(_: &(), _: &()) -> () {
        ()
    }
}

// Reference: https://cp-algorithms.com/data_structures/deleting_in_log_n.html
// Reference: https://koosaga.com/121
// Reference: https://stonejjun.tistory.com/171
// Reference: https://github.com/bminaiev/rust-contests/blob/main/algo_lib/src/graph/dynamic_connectivity_offline.rs
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut odc = DynamicConnectivityOffline::<(), (usize, usize)>::new(n);
    let mut cnt_query_3 = 0;

    for _ in 0..m {
        let (command, a, b) = (
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if command == 3 {
            cnt_query_3 += 1;
        }

        match command {
            1 => odc.add_edge(a, b),
            2 => odc.remove_edge(a, b),
            3 => odc.add_query((a, b)),
            _ => unreachable!(),
        }
    }

    if cnt_query_3 == 0 {
        return;
    }

    odc.run(&mut |query, dsu| {
        let parent_a = dsu.get(query.0);
        let parent_b = dsu.get(query.1);

        writeln!(out, "{}", if parent_a == parent_b { "YES" } else { "NO" }).unwrap();
    });
}

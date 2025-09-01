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

#[derive(Clone, Copy, Debug)]
struct Interval(i64, i64);

fn ceil_div(a: i64, b: i64) -> i64 {
    if a >= 0 {
        (a + b - 1) / b
    } else {
        -((-a) / b)
    }
}

fn floor_div(a: i64, b: i64) -> i64 {
    if a >= 0 {
        a / b
    } else {
        -((-a + b - 1) / b)
    }
}

fn merge_intervals(mut intervals: Vec<Interval>) -> Vec<Interval> {
    if intervals.is_empty() {
        return Vec::new();
    }

    intervals.sort_by_key(|interval| (interval.0, interval.1));

    let mut ret = Vec::with_capacity(intervals.len());
    let mut curr = intervals[0];

    for &interval in intervals.iter().skip(1) {
        if curr.1 >= interval.0 {
            curr.1 = curr.1.max(interval.1);
        } else {
            ret.push(curr);
            curr = interval;
        }
    }

    ret.push(curr);
    ret
}

fn dead_intervals_1d(mut x0: i64, mut dx: i64, t: i64) -> Vec<Interval> {
    let rem = 4 * t;
    let mut ret: Vec<Interval> = Vec::new();

    if dx < 0 {
        x0 = -x0;
        dx = -dx;
    }

    x0 = ((x0 % rem) + rem) % rem;

    if dx == 0 {
        if t <= x0 && x0 <= 3 * t {
            ret.push(Interval(1, t + 1));
        }

        return ret;
    }

    let lower = -(x0.abs() / rem);
    let upper = (dx.abs() * t + x0.abs()) / rem + 2;

    for k in lower..=upper {
        let bound_left = t - x0 + k * rem;
        let bound_right = 3 * t - x0 + k * rem;

        let mut t_start = ceil_div(bound_left, dx);
        let mut t_end = floor_div(bound_right, dx);

        t_start = t_start.max(1);
        t_end = t_end.min(t);

        if t_start <= t_end {
            ret.push(Interval(t_start, t_end + 1));
        }
    }

    merge_intervals(ret)
}

fn intersect_intervals(a: &[Interval], b: &[Interval]) -> Vec<Interval> {
    let mut i = 0;
    let mut j = 0;
    let mut ret = Vec::new();

    while i < a.len() && j < b.len() {
        let start = a[i].0.max(b[j].0);
        let end = a[i].1.min(b[j].1);

        if start < end {
            ret.push(Interval(start, end));
        }

        if a[i].1 < b[j].1 {
            i += 1;
        } else {
            j += 1;
        }
    }

    ret
}

fn invert_to_alive_intervals(dead: &[Interval], t: i64) -> Vec<Interval> {
    let mut alive = Vec::new();
    let mut curr = 1;

    for &Interval(s, e) in dead {
        if curr < s {
            alive.push(Interval(curr, s));
        }

        curr = curr.max(e);
    }

    if curr <= t {
        alive.push(Interval(curr, t + 1));
    }

    alive
}

fn alive_intervals(
    angular_velocity1: i64,
    angular_velocity2: i64,
    phase1: i64,
    phase2: i64,
    t: i64,
) -> Vec<Interval> {
    let x0 = 4 * (phase1 - phase2);
    let y0 = 4 * (phase1 + phase2);
    let dx = 4 * (angular_velocity1 - angular_velocity2);
    let dy = 4 * (angular_velocity1 + angular_velocity2);

    let intervals_dead_x = dead_intervals_1d(x0, dx, t);
    let intervals_dead_y = dead_intervals_1d(y0, dy, t);

    let intervals_dead_total = intersect_intervals(&intervals_dead_x, &intervals_dead_y);
    let intervals_alive = invert_to_alive_intervals(&intervals_dead_total, t);

    intervals_alive
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, t) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut angular_velocities = vec![vec![0; m]; n];
    let mut phases = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            angular_velocities[i][j] = scan.token::<i64>();
        }
    }

    for i in 0..n {
        for j in 0..m {
            phases[i][j] = scan.token::<i64>();
        }
    }

    let mut event_add: Vec<Vec<(usize, usize)>> = vec![Vec::new(); t as usize];
    let mut event_rem: Vec<Vec<(usize, usize)>> = vec![Vec::new(); t as usize];

    for i in 0..n {
        for j in 0..m {
            let u = (i * m + j) as usize;

            if j + 1 < m {
                let v = (i * m + (j + 1)) as usize;
                let intervals = alive_intervals(
                    angular_velocities[i][j],
                    angular_velocities[i][j + 1],
                    phases[i][j],
                    phases[i][j + 1],
                    t,
                );

                let (u, v) = if u < v { (u, v) } else { (v, u) };

                for Interval(l, r) in intervals {
                    let start = (l - 1) as usize;
                    let end = (r - 1) as usize;

                    if start < t as usize {
                        event_add[start].push((u, v));
                    }

                    if end < t as usize {
                        event_rem[end].push((u, v));
                    }
                }
            }

            if i + 1 < n {
                let v = (i + 1) * m + j;
                let intervals = alive_intervals(
                    angular_velocities[i][j],
                    angular_velocities[i + 1][j],
                    phases[i][j],
                    phases[i + 1][j],
                    t,
                );

                let (u, v) = if u < v { (u, v) } else { (v, u) };

                for Interval(l, r) in intervals {
                    let start = (l - 1) as usize;
                    let end = (r - 1) as usize;

                    if start < t as usize {
                        event_add[start].push((u, v));
                    }

                    if end < t as usize {
                        event_rem[end].push((u, v));
                    }
                }
            }
        }
    }

    let mut odc = DynamicConnectivityOffline::<(), usize>::new(n * m);

    for i in 0..t as usize {
        for &(u, v) in event_add[i].iter() {
            odc.add_edge(u, v);
        }

        for &(u, v) in event_rem[i].iter() {
            odc.remove_edge(u, v);
        }

        odc.add_query(i);
    }

    let mut ret = vec![0; t as usize];
    let mut callback = |q: &usize, dsu: &DsuWithRollback<()>| {
        let unions = dsu.get_current_time() / 2;
        let comps = n * m - unions;

        ret[*q] = comps;
    };

    odc.run(&mut callback);

    for i in 0..t as usize {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}

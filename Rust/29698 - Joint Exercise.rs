use io::Write;
use std::{
    borrow::Borrow,
    collections::{btree_map, BTreeMap, BTreeSet, HashMap},
    io, str,
};

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

#[derive(Debug, Clone)]
pub struct MultiSet<T> {
    freq: BTreeMap<T, usize>,
    len: usize,
}

pub struct Iter<'a, T> {
    iter: btree_map::Iter<'a, T, usize>,
    front: Option<&'a T>,
    front_to_dispatch: usize,
    back: Option<&'a T>,
    back_to_dispatch: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_to_dispatch == 0 {
            if let Some((k, &v)) = self.iter.next() {
                self.front = Some(k);
                self.front_to_dispatch = v;
            } else if self.back_to_dispatch > 0 {
                self.back_to_dispatch -= 1;
                return self.back;
            }
        }

        if self.front_to_dispatch > 0 {
            self.front_to_dispatch -= 1;
            return self.front;
        }

        None
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back_to_dispatch == 0 {
            if let Some((k, &v)) = self.iter.next_back() {
                self.back = Some(k);
                self.back_to_dispatch = v;
            } else if self.front_to_dispatch > 0 {
                self.front_to_dispatch -= 1;
                return self.front;
            }
        }

        if self.back_to_dispatch > 0 {
            self.back_to_dispatch -= 1;
            return self.back;
        }

        None
    }
}

impl<T: Ord> Default for MultiSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MultiSet<T> {
    pub fn new() -> Self
    where
        T: Ord,
    {
        Self {
            freq: BTreeMap::new(),
            len: 0,
        }
    }

    pub fn insert(&mut self, val: T)
    where
        T: Ord,
    {
        *self.freq.entry(val).or_insert(0) += 1;
        self.len += 1;
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.freq.is_empty()
    }

    pub fn contains<Q: ?Sized>(&self, val: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.freq.contains_key(val)
    }

    pub fn remove<Q: ?Sized>(&mut self, val: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        if self.contains(val) {
            *self.freq.get_mut(val).unwrap() -= 1;

            if self.freq[val] == 0 {
                self.freq.remove(val);
            }

            self.len -= 1;
            return true;
        }

        false
    }

    pub fn iter(&self) -> Iter<T>
    where
        T: Ord,
    {
        Iter {
            iter: self.freq.iter(),
            front: None,
            front_to_dispatch: 0,
            back: None,
            back_to_dispatch: 0,
        }
    }
}

fn process_dfs(
    parent: &mut Vec<usize>,
    costs_sum: &mut Vec<i64>,
    costs_sum_distinct: &mut Vec<BTreeSet<i64>>,
    costs_sum_group: &mut Vec<MultiSet<i64>>,
    rank_group: &mut MultiSet<i64>,
    dfs: &mut BTreeSet<usize>,
    dissatisfactions: &HashMap<usize, HashMap<usize, i64>>,
    k: i64,
    idx_node: usize,
    idx_parent: usize,
) {
    let mut stack = vec![(idx_node, idx_parent)];

    while let Some((curr_node, curr_parent)) = stack.pop() {
        process_union(
            parent,
            costs_sum,
            costs_sum_distinct,
            costs_sum_group,
            rank_group,
            curr_parent,
            curr_node,
        );

        let mut flag = false;
        let mut idx_next = 0;

        while idx_next < dfs.len() {
            let node_next = *dfs.iter().nth(idx_next).unwrap();
            idx_next += 1;

            if dissatisfactions
                .get(&(curr_node))
                .and_then(|inner_map| inner_map.get(&(node_next)))
                .unwrap_or(&0)
                >= &k
            {
                continue;
            } else {
                dfs.remove(&node_next);
                stack.push((node_next, curr_parent));
                flag = true;
                break;
            }
        }

        if !dfs.is_empty() && flag {
            stack.push((curr_node, curr_parent));
            continue;
        }
    }
}

fn find(parent: &mut Vec<usize>, node: usize) -> usize {
    if parent[node] == node {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn process_union(
    parent: &mut Vec<usize>,
    costs_sum: &mut Vec<i64>,
    costs_sum_distinct: &mut Vec<BTreeSet<i64>>,
    costs_sum_group: &mut Vec<MultiSet<i64>>,
    rank_group: &mut MultiSet<i64>,
    mut a: usize,
    mut b: usize,
) {
    a = find(parent, a);
    b = find(parent, b);

    if a == b {
        return;
    }

    if costs_sum_group[a].len() < costs_sum_group[b].len() {
        std::mem::swap(&mut a, &mut b);
    }

    parent[b] = a;

    rank_group.remove(&costs_sum[a]);
    rank_group.remove(&costs_sum[b]);

    costs_sum[a] += costs_sum[b];
    rank_group.insert(costs_sum[a]);

    let costs = costs_sum_group[b].iter().cloned().collect::<Vec<_>>();

    for cost in costs.iter() {
        costs_sum_group[a].insert(*cost);
    }

    let costs = costs_sum_distinct[b].iter().cloned().collect::<Vec<_>>();

    for cost in costs.iter() {
        costs_sum_distinct[a].insert(*cost);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut costs = vec![0; n];
    let mut costs_sum = vec![0; n];
    let mut costs_sum_distinct = vec![BTreeSet::new(); n];
    let mut costs_sum_group = vec![MultiSet::new(); n];
    let mut rank_group = MultiSet::new();
    let mut dissatisfactions: HashMap<usize, HashMap<usize, i64>> = HashMap::new();
    let mut parent = vec![0; n];

    for i in 0..n {
        costs[i] = scan.token::<i64>();
        costs_sum[i] += costs[i];
        parent[i] = i;
    }

    let q = scan.token::<usize>();
    let mut edges = vec![Vec::new(); q];
    let mut query_type = vec![0; q];
    let mut query3_params = vec![(0, 0); q];
    let mut ret = vec![0; q];

    for i in 0..q {
        query_type[i] = scan.token::<i64>();

        if query_type[i] == 1 {
            let mut soldiers = vec![0; 2 * n];
            let mut need_bus = vec![0; n];

            for j in 0..2 * n {
                soldiers[j] = scan.token::<usize>() - 1;
                need_bus[soldiers[j]] = 1;
            }

            let mut can_plan = true;

            for j in 1..2 * n {
                if dissatisfactions
                    .get(&(soldiers[j]))
                    .and_then(|inner_map| inner_map.get(&(soldiers[j - 1])))
                    .unwrap_or(&0)
                    >= &k
                {
                    can_plan = false;
                    break;
                }
            }

            if !can_plan {
                ret[i] = -1;
                continue;
            }

            for j in 1..2 * n {
                if soldiers[j] == soldiers[j - 1] {
                    continue;
                }

                dissatisfactions
                    .entry(soldiers[j])
                    .or_insert(HashMap::new())
                    .entry(soldiers[j - 1])
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
                dissatisfactions
                    .entry(soldiers[j - 1])
                    .or_insert(HashMap::new())
                    .entry(soldiers[j])
                    .and_modify(|x| *x += 1)
                    .or_insert(1);

                if dissatisfactions[&(soldiers[j])][&(soldiers[j - 1])] >= k {
                    edges[i].push((soldiers[j], soldiers[j - 1]));
                }
            }

            for j in 0..n {
                ret[i] += need_bus[j] * costs[j];
            }
        } else if query_type[i] == 2 {
            let (s, t, x) = (
                scan.token::<usize>() - 1,
                scan.token::<usize>() - 1,
                scan.token::<i64>(),
            );

            if dissatisfactions
                .get(&s)
                .and_then(|inner_map| inner_map.get(&t))
                .unwrap_or(&0)
                >= &k
            {
                continue;
            }

            dissatisfactions
                .entry(s)
                .or_insert(HashMap::new())
                .entry(t)
                .and_modify(|val| *val += x)
                .or_insert(x);
            dissatisfactions
                .entry(t)
                .or_insert(HashMap::new())
                .entry(s)
                .and_modify(|val| *val += x)
                .or_insert(x);

            if dissatisfactions[&s][&t] >= k {
                edges[i].push((s, t));
            }
        } else {
            let (s, x) = (scan.token::<usize>() - 1, scan.token::<i64>());
            query3_params[i] = (s, x);

            costs[s] += x;
            costs_sum[s] += x;
        }
    }

    for i in 0..n {
        costs_sum_distinct[i].insert(costs_sum[i]);
        costs_sum_group[i].insert(costs_sum[i]);
        rank_group.insert(costs_sum[i]);
    }

    let mut dfs = BTreeSet::new();

    for i in 0..n {
        dfs.insert(i);
    }

    while !dfs.is_empty() {
        let idx = *dfs.iter().next().unwrap();
        dfs.remove(&idx);

        process_dfs(
            &mut parent,
            &mut costs_sum,
            &mut costs_sum_distinct,
            &mut costs_sum_group,
            &mut rank_group,
            &mut dfs,
            &dissatisfactions,
            k,
            idx,
            idx,
        );
    }

    for i in (0..q).rev() {
        if query_type[i] == 2 {
            ret[i] = rank_group.iter().last().unwrap().clone();
        } else if query_type[i] == 3 {
            let (s, x) = query3_params[i];
            let node_parent = find(&mut parent, s);
            ret[i] = costs_sum_distinct[node_parent].len() as i64;

            rank_group.remove(&costs_sum[node_parent]);
            costs_sum[node_parent] -= x;
            rank_group.insert(costs_sum[node_parent]);

            costs_sum_group[node_parent].remove(&costs[s]);

            if !costs_sum_group[node_parent].contains(&costs[s]) {
                costs_sum_distinct[node_parent].remove(&costs[s]);
            }

            costs[s] -= x;
            costs_sum_group[node_parent].insert(costs[s]);
            costs_sum_distinct[node_parent].insert(costs[s]);
        }

        for edge in edges[i].iter() {
            process_union(
                &mut parent,
                &mut costs_sum,
                &mut costs_sum_distinct,
                &mut costs_sum_group,
                &mut rank_group,
                edge.0,
                edge.1,
            );
        }
    }

    for i in 0..q {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}

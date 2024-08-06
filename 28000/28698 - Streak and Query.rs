use io::Write;
use std::borrow::Borrow;
use std::collections::{btree_map, BTreeMap};
use std::{cmp::Ordering, io, str};
use Ordering::Less;

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

pub trait Ext {
    type Item;

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    fn lower_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;
}

impl<T> Ext for [T] {
    type Item = T;

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.lower_bound_by(|y| y.cmp(x))
    }

    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();

        if size == 0 {
            return 0;
        }

        let mut base = 0usize;

        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Less { mid } else { base };
            size -= half;
        }

        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp == Less) as usize
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

#[derive(Clone, Debug)]
struct Node {
    times: (i64, i64),
    len: i64,
    left: i64,
    right: i64,
    val: i64,
}

impl Node {
    fn new() -> Self {
        Self {
            times: (0, 0),
            len: 0,
            left: 0,
            right: 0,
            val: 0,
        }
    }

    fn merge(&mut self, other: &Self) -> Node {
        let mut ret = Node::new();

        ret.times = (self.times.0, other.times.1);
        ret.len = self.len + other.len;
        ret.left = if self.left == self.len && self.times.1 == other.times.0 - 1 {
            self.left + other.left
        } else {
            self.left
        };
        ret.right = if other.right == other.len && self.times.1 == other.times.0 - 1 {
            self.right + other.right
        } else {
            other.right
        };
        ret.val = self
            .val
            .max(other.val)
            .max(if self.times.1 == other.times.0 - 1 {
                self.right + other.left
            } else {
                0
            });

        ret
    }
}

#[derive(Clone)]
struct SegmentTree {
    pub data: Vec<Node>,
}

impl SegmentTree {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn init(n: usize) -> Self {
        Self {
            data: vec![Node::new(); n * 4],
        }
    }

    pub fn update_init(
        &mut self,
        node: usize,
        node_start: usize,
        node_end: usize,
        data: &Vec<i64>,
    ) {
        if node_start == node_end {
            self.data[node].times = (data[node_start], data[node_start]);
            self.data[node].len = 1;
            self.data[node].left = 0;
            self.data[node].right = 0;
            self.data[node].val = 0;
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_init(node * 2, node_start, mid, data);
        self.update_init(node * 2 + 1, mid + 1, node_end, data);

        let mut left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();
        self.data[node] = left.merge(&right);
    }

    pub fn update(
        &mut self,
        index: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if index > node_end || index < node_start {
            return;
        }

        if node_start == node_end {
            self.data[node].left = val;
            self.data[node].right = val;
            self.data[node].val = val;
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update(index, val, node * 2, node_start, mid);
        self.update(index, val, node * 2 + 1, mid + 1, node_end);

        let mut left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();
        self.data[node] = left.merge(&right);
    }
}

struct FenwickTree {
    data: Vec<i64>,
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        Self { data: vec![0; n] }
    }

    pub fn update(&mut self, idx: usize, num: i64) {
        let mut idx = idx as i64 + 1;

        while idx < 100_001 {
            self.data[idx as usize] += num;
            idx += idx & -idx;
        }
    }

    pub fn query(&mut self, idx: usize) -> i64 {
        let mut idx = idx as i64 + 1;
        let mut ret = 0;

        while idx > 0 {
            ret += self.data[idx as usize];
            idx -= idx & -idx;
        }

        ret
    }
}

#[derive(Clone, Copy)]
enum Query {
    SubmitSolution(usize, usize, i64),
    ChangeDate,
    RejudgeSubmit(usize),
    PrintStreak(usize),
}

#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
struct User {
    idx: usize,
    problem: usize,
}

impl User {
    fn new(idx: usize, problem: usize) -> Self {
        Self { idx, problem }
    }
}

fn process_history(
    segment_trees: &mut Vec<SegmentTree>,
    fenwick_tree: &mut FenwickTree,
    users: &Vec<User>,
    times: &Vec<Vec<i64>>,
    counts: &mut Vec<Vec<i64>>,
    multi_sets: &mut Vec<MultiSet<i64>>,
    idx: usize,
    time: i64,
    kind: i64,
) {
    let idx_user = users[idx].idx;
    let val_prev = segment_trees[idx_user].data[1].val;

    if !multi_sets[idx].is_empty() {
        let time_prev = multi_sets[idx].iter().next().unwrap();
        let idx_time = times[idx_user].lower_bound(&time_prev);
        counts[idx_user][idx_time] -= 1;

        if counts[idx_user][idx_time] == 0 {
            segment_trees[idx_user].update(idx_time, 0, 1, 0, times[idx_user].len() - 1);
        }
    }

    if kind == 1 {
        multi_sets[idx].insert(time);
    } else {
        multi_sets[idx].remove(&time);
    }

    if !multi_sets[idx].is_empty() {
        let time_curr = multi_sets[idx].iter().next().unwrap();
        let idx_time = times[idx_user].lower_bound(&time_curr);
        counts[idx_user][idx_time] += 1;

        if counts[idx_user][idx_time] != 0 {
            segment_trees[idx_user].update(idx_time, 1, 1, 0, times[idx_user].len() - 1);
        }
    }

    let val_curr = segment_trees[idx_user].data[1].val;
    fenwick_tree.update(val_prev as usize, -1);
    fenwick_tree.update(val_curr as usize, 1);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut queries = vec![Query::ChangeDate; 100_001];
    let mut users = Vec::new();
    let mut times = vec![Vec::new(); 100_001];
    let mut counts = vec![Vec::new(); 100_001];
    let mut time_curr = 0;

    // Input and preprocess
    for i in 1..=q {
        let command = scan.token::<i64>();

        if command == 1 {
            let (u, p, c) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            queries[i] = Query::SubmitSolution(u, p, c);
            users.push(User::new(u, p));
            times[u].push(time_curr);
        } else if command == 2 {
            queries[i] = Query::ChangeDate;
            time_curr += 1;
        } else if command == 3 {
            let idx = scan.token::<usize>();
            queries[i] = Query::RejudgeSubmit(idx);
        } else {
            let k = scan.token::<usize>();
            queries[i] = Query::PrintStreak(k);
        }
    }

    for i in 1..=n {
        counts[i] = vec![0; times[i].len()];
    }

    time_curr = 0;

    // Make segment trees and fenwick tree
    let mut segment_trees = vec![SegmentTree::new(); 100_001];
    let mut fenwick_tree = FenwickTree::new(100_001);

    for i in 1..=n {
        if times[i].is_empty() {
            continue;
        }

        times[i].dedup();

        segment_trees[i] = SegmentTree::init(times[i].len());
        segment_trees[i].update_init(1, 0, times[i].len() - 1, &times[i]);
    }

    fenwick_tree.update(0, n as i64);

    // Process users and related problems
    users.sort();
    users.dedup();

    for i in 1..=q {
        if let Query::SubmitSolution(u, p, c) = queries[i].clone() {
            let u_new = users.lower_bound(&User::new(u, p));
            let p_new = c;
            let c_new = time_curr;

            queries[i] = Query::SubmitSolution(u_new, p_new as usize, c_new);
        } else if matches!(queries[i], Query::ChangeDate) {
            time_curr += 1;
        }
    }

    time_curr = 0;

    // Process history
    let mut multi_sets = vec![MultiSet::new(); 100_001];

    // Process queries
    for i in 1..=q {
        match queries[i] {
            Query::SubmitSolution(u, p, _) => {
                if p == 1 {
                    process_history(
                        &mut segment_trees,
                        &mut fenwick_tree,
                        &users,
                        &times,
                        &mut counts,
                        &mut multi_sets,
                        u,
                        time_curr,
                        1,
                    );
                }
            }
            Query::ChangeDate => {
                time_curr += 1;
            }
            Query::RejudgeSubmit(idx) => {
                if let Query::SubmitSolution(u, p, c) = queries[idx].clone() {
                    if p == 1 {
                        process_history(
                            &mut segment_trees,
                            &mut fenwick_tree,
                            &users,
                            &times,
                            &mut counts,
                            &mut multi_sets,
                            u,
                            c,
                            0,
                        );
                        queries[idx] = Query::SubmitSolution(u, 0, c);
                    } else {
                        process_history(
                            &mut segment_trees,
                            &mut fenwick_tree,
                            &users,
                            &times,
                            &mut counts,
                            &mut multi_sets,
                            u,
                            c,
                            1,
                        );
                        queries[idx] = Query::SubmitSolution(u, 1, c);
                    }
                }
            }
            Query::PrintStreak(k) => {
                let mut left = 0;
                let mut right = q;

                while left != right {
                    let mid = (left + right) / 2;

                    if fenwick_tree.query(mid) >= n as i64 - k as i64 + 1 {
                        right = mid;
                    } else {
                        left = mid + 1;
                    }
                }

                writeln!(out, "{left}").unwrap();
            }
        }
    }
}

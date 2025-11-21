use io::Write;
use std::{
    collections::BTreeSet,
    io,
    ops::Bound::{Excluded, Unbounded},
    str,
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

#[derive(Debug, Clone, Default)]
struct Node {
    val: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self { val }
    }

    fn merge(&self, other: &Self) -> Node {
        Node {
            val: self.val + other.val,
        }
    }
}

struct SegmentTree {
    size: usize,
    data: Vec<Node>,
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![Node::new(0); real_n * 4],
        }
    }

    pub fn update(&mut self, index: usize, val: i64) {
        self.update_internal(index, val, 1, 1, self.size);
    }

    fn update_internal(
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
            self.data[node].val += val;
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(index, val, node * 2, node_start, mid);
        self.update_internal(index, val, node * 2 + 1, mid + 1, node_end);

        let left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();
        self.data[node] = left.merge(&right);
    }

    fn query(&self, start: usize, end: usize) -> Node {
        self.query_internal(start, end, 1, 1, self.size)
    }

    fn query_internal(
        &self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> Node {
        if end < node_start || node_end < start {
            return Node::new(0);
        }

        if start <= node_start && node_end <= end {
            return self.data[node].clone();
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        Node::merge(&left, &right)
    }
}

struct UnionFind {
    parent: Vec<usize>,
    offset: Vec<i64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![0; n],
            offset: vec![0; n],
        }
    }

    fn init(&mut self) {
        for i in 0..self.parent.len() {
            self.parent[i] = i;
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        let p = self.find(self.parent[x]);

        if p != self.parent[x] {
            self.offset[x] += self.offset[self.parent[x]];
        }

        self.parent[x] = p;
        p
    }
}

fn lower_bound(vec: &Vec<i64>, n: usize, x: i64) -> usize {
    let mut left = 1;
    let mut right = n;
    let mut ret = n + 1;

    while left <= right {
        let mid = (left + right) / 2;

        if vec[mid] >= x {
            ret = mid;

            if mid == 0 {
                break;
            }

            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    ret
}

fn compute_pseudo_critical_time(
    tree: &SegmentTree,
    pesudo_offset_forest: &mut UnionFind,
    deadlines: &Vec<i64>,
    idx: usize,
    t: i64,
) -> i64 {
    let root = pesudo_offset_forest.find(idx);

    let mut ret = deadlines[idx];
    ret -= pesudo_offset_forest.offset[idx];

    if idx != root {
        ret -= pesudo_offset_forest.offset[root];
    }

    ret -= tree.query(1, idx).val * t;

    ret
}

fn update_q(
    fractional_offset_tree: &mut BTreeSet<(i64, usize)>,
    pesudo_offset_forest: &mut UnionFind,
    a: i64,
    b: i64,
    t: i64,
) {
    let (a, b) = (a % t, b % t);

    if a < b {
        let mut affected = Vec::new();

        loop {
            let next = fractional_offset_tree
                .range((Excluded((a, usize::MAX)), Unbounded))
                .next()
                .cloned();

            match next {
                Some((q, idx)) if q <= b => {
                    fractional_offset_tree.remove(&(q, idx));
                    affected.push((q, idx));
                }
                _ => break,
            }
        }

        if !affected.is_empty() {
            let root = affected[0].1;

            pesudo_offset_forest.offset[root] += affected[0].0 - a;

            for k in 1..affected.len() {
                pesudo_offset_forest.parent[affected[k].1] = root;
                pesudo_offset_forest.offset[affected[k].1] +=
                    affected[k].0 - a - pesudo_offset_forest.offset[root];
            }

            fractional_offset_tree.insert((a, root));
        }
    } else {
        let mut affected = Vec::new();

        loop {
            let next = fractional_offset_tree
                .range((Excluded((a, usize::MAX)), Unbounded))
                .next()
                .cloned();

            match next {
                Some(p) => {
                    fractional_offset_tree.remove(&p);
                    affected.push(p);
                }
                None => break,
            }
        }

        if !affected.is_empty() {
            let root = affected[0].1;

            pesudo_offset_forest.offset[root] += affected[0].0 - a;

            for k in 1..affected.len() {
                pesudo_offset_forest.parent[affected[k].1] = root;
                pesudo_offset_forest.offset[affected[k].1] +=
                    affected[k].0 - a - pesudo_offset_forest.offset[root];
            }

            fractional_offset_tree.insert((a, root));
        }

        let mut affected = Vec::new();

        loop {
            if let Some(&(q, idx)) = fractional_offset_tree.iter().next() {
                if q <= b {
                    fractional_offset_tree.remove(&(q, idx));
                    affected.push((q, idx));
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if !affected.is_empty() {
            let root = affected[0].1;

            pesudo_offset_forest.offset[root] += affected[0].0 - a + t;

            for k in 1..affected.len() {
                pesudo_offset_forest.parent[affected[k].1] = root;
                pesudo_offset_forest.offset[affected[k].1] +=
                    affected[k].0 - a + t - pesudo_offset_forest.offset[root];
            }

            fractional_offset_tree.insert((a, root));
        }
    }
}

// Reference: Scheduling Unit-time Tasks with Release Time and Deadlines (Garey et al., 1981)
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, t) = (scan.token::<usize>(), scan.token::<i64>());
    let mut photographs = vec![(0, 0); n + 1];
    let mut deadlines = vec![0; n + 1];

    for i in 1..=n {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());

        photographs[i] = (a, b);
        deadlines[i] = b;
    }

    photographs[1..=n].sort_unstable_by(|a, b| a.0.cmp(&b.0));
    deadlines[1..=n].sort_unstable();

    let mut tree = SegmentTree::new(n + 1);
    let mut forbidden_regions = Vec::new();
    let mut fractional_offset_tree = BTreeSet::new();
    let mut pesudo_offset_forest = UnionFind::new(n + 1);
    let mut relevant_deadlines = BTreeSet::new();
    let mut deadline_next = n + 1;

    pesudo_offset_forest.init();

    for i in (1..=n).rev() {
        let (a, b) = photographs[i];
        let idx = lower_bound(&deadlines, n, b);

        tree.update(idx, 1);

        if deadline_next > idx {
            while deadline_next > idx {
                deadline_next -= 1;

                let q = deadlines[deadline_next] % t;
                fractional_offset_tree.insert((q, deadline_next));

                if relevant_deadlines.is_empty() {
                    relevant_deadlines.insert(deadline_next);
                } else {
                    let first = *relevant_deadlines.iter().next().unwrap();
                    let c_prime = compute_pseudo_critical_time(
                        &tree,
                        &mut pesudo_offset_forest,
                        &deadlines,
                        first,
                        t,
                    );

                    if deadlines[deadline_next] - t < c_prime {
                        relevant_deadlines.insert(deadline_next);
                    }
                }
            }
        } else {
            if let Some(&idx_relevant) = relevant_deadlines.range(idx..).next() {
                let c_prime = compute_pseudo_critical_time(
                    &tree,
                    &mut pesudo_offset_forest,
                    &deadlines,
                    idx_relevant,
                    t,
                );

                loop {
                    let opt_prev = relevant_deadlines.range(..idx).next_back().copied();

                    if let Some(idx) = opt_prev {
                        if compute_pseudo_critical_time(
                            &tree,
                            &mut pesudo_offset_forest,
                            &deadlines,
                            idx,
                            t,
                        ) >= c_prime
                        {
                            relevant_deadlines.remove(&idx);
                            continue;
                        }
                    }

                    break;
                }
            }
        }

        if relevant_deadlines.is_empty() {
            writeln!(out, "no").unwrap();
            return;
        }

        let first = *relevant_deadlines.iter().next().unwrap();
        let c_min_prime =
            compute_pseudo_critical_time(&tree, &mut pesudo_offset_forest, &deadlines, first, t);

        if c_min_prime < a {
            writeln!(out, "no").unwrap();
            return;
        }

        if c_min_prime < a + t {
            let endpoint_left = c_min_prime - t;
            let mut endpoint_right = a - 1;

            if let Some(&(left_prev, _)) = forbidden_regions.last() {
                endpoint_right = endpoint_right.min(left_prev);
            }

            if endpoint_left < endpoint_right {
                update_q(
                    &mut fractional_offset_tree,
                    &mut pesudo_offset_forest,
                    endpoint_left,
                    endpoint_right,
                    t,
                );
                forbidden_regions.push((endpoint_left, endpoint_right));
            }
        }
    }

    writeln!(out, "yes").unwrap();
}

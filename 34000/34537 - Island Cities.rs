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

struct UnionFind {
    parent: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind { parent: vec![0; n] }
    }

    fn init(&mut self) {
        for i in 0..self.parent.len() {
            self.parent[i] = i;
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            self.parent[root_y] = root_x;
        }
    }
}

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
    w: i64,
    inc: i64,
    cost: i64,
}

impl Edge {
    fn new(u: usize, v: usize, w: i64, inc: i64, cost: i64) -> Self {
        Self { u, v, w, inc, cost }
    }
}

#[inline]
fn calculate_cost(edge: &Edge, x: i64, budget: i64) -> i64 {
    if x <= edge.w {
        return 0;
    }

    let need = x - edge.w;
    let k = if need <= 0 {
        0
    } else {
        (need + edge.inc - 1) / edge.inc
    };
    let cost = k * edge.cost;

    if cost > budget {
        budget + 1
    } else {
        cost
    }
}

fn check(edges: &Vec<Edge>, x: i64, budget: i64, n: usize) -> bool {
    let mut candidate = Vec::with_capacity(edges.len());

    for (idx, edge) in edges.iter().enumerate() {
        let cost = calculate_cost(edge, x, budget);

        if cost <= budget {
            candidate.push((cost, idx));
        }
    }

    candidate.sort_unstable_by_key(|&(c, _)| c);

    let mut uf = UnionFind::new(n);
    uf.init();

    let mut used = 0;
    let mut sum = 0;

    for (cost, idx) in candidate {
        let edge = edges[idx];
        let root_u = uf.find(edge.u);
        let root_v = uf.find(edge.v);

        if root_u != root_v {
            uf.union(root_u, root_v);
            sum += cost;
            used += 1;

            if sum > budget {
                return false;
            }

            if used == n - 1 {
                return true;
            }
        }
    }

    false
}

fn build_mst(edges: &Vec<Edge>, x: i64, budget: i64, n: usize) -> Vec<usize> {
    let mut candidate = Vec::with_capacity(edges.len());

    for (idx, edge) in edges.iter().enumerate() {
        let cost = calculate_cost(edge, x, budget);

        if cost <= budget {
            candidate.push((cost, idx));
        }
    }

    candidate.sort_unstable_by_key(|&(c, _)| c);

    let mut uf = UnionFind::new(n);
    uf.init();

    let mut used = Vec::with_capacity(n - 1);

    for (_, idx) in candidate {
        let edge = edges[idx];
        let root_u = uf.find(edge.u);
        let root_v = uf.find(edge.v);

        if root_u != root_v {
            uf.union(root_u, root_v);
            used.push(idx);

            if used.len() == n - 1 {
                break;
            }
        }
    }

    used
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, b) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut edges = Vec::with_capacity(m);
    let mut weight_max = 0;
    let mut inc_max = 0;
    let mut cost_min = i64::MAX;

    for _ in 0..m {
        let (u, v, w, x, y) = (
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        edges.push(Edge::new(u, v, w, x, y));

        weight_max = weight_max.max(w);
        inc_max = inc_max.max(x);
        cost_min = cost_min.min(y);
    }

    let mut left = 0;
    let mut right = weight_max + (b / cost_min) * inc_max + 1;

    while left + 1 < right {
        let mid = (left + right) / 2;

        if check(&edges, mid, b, n) {
            left = mid;
        } else {
            right = mid;
        }
    }

    let used_idx = build_mst(&edges, left, b, n);
    let mut ret = vec![0; m];

    for idx in used_idx {
        let edge = edges[idx];

        if left > edge.w {
            let need = left - edge.w;

            ret[idx] = if need <= 0 {
                0
            } else {
                (need + edge.inc - 1) / edge.inc
            };
        }
    }

    writeln!(out, "{left}").unwrap();

    for val in ret {
        writeln!(out, "{val} ").unwrap();
    }
}

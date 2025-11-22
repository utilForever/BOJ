use io::Write;
use std::{cmp::Reverse, collections::BinaryHeap, io, str};

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
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![0; n + 1],
            size: vec![1; n + 1],
        }
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

    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }

        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];

        true
    }
}

#[derive(Clone, Copy)]
enum IntervalKind {
    Diagonal,
    Boundary,
}

#[derive(Clone, Copy)]
struct IntervalEdge {
    l: usize,
    r: usize,
    id: usize,
    kind: IntervalKind,
}

impl IntervalEdge {
    fn new(l: usize, r: usize, id: usize, kind: IntervalKind) -> Self {
        IntervalEdge { l, r, id, kind }
    }
}

#[derive(Clone, Copy)]
struct ActiveChord {
    id: usize,
    r: usize,
}

impl ActiveChord {
    fn new(id: usize, r: usize) -> Self {
        ActiveChord { id, r }
    }
}

struct GomoryHuTree {
    graph: Vec<Vec<(usize, i64)>>,
}

impl GomoryHuTree {
    fn new(n: usize) -> Self {
        GomoryHuTree {
            graph: vec![Vec::new(); n + 1],
        }
    }

    fn from_outer_planar(n: usize, edges_input: &Vec<(usize, usize, i64)>) -> Self {
        let m = edges_input.len();
        let mut capacity = vec![0_i64; m + 2];
        let mut edges = vec![(0_usize, 0_usize); m + 2];
        let mut items = Vec::with_capacity(m);

        for (idx, &(mut u, mut v, w)) in edges_input.iter().enumerate() {
            if u > v {
                std::mem::swap(&mut u, &mut v);
            }

            capacity[idx + 1] = w;
            edges[idx + 1] = (u, v);

            let (l, r, kind) = if u + 1 == v {
                (u, u + 1, IntervalKind::Boundary)
            } else if u == 1 && v == n {
                (n, n + 1, IntervalKind::Boundary)
            } else {
                (u, v, IntervalKind::Diagonal)
            };

            items.push(IntervalEdge::new(l, r, idx + 1, kind));
        }

        items.sort_unstable_by(|a, b| {
            if a.l != b.l {
                a.l.cmp(&b.l)
            } else {
                match (a.kind, b.kind) {
                    (IntervalKind::Diagonal, IntervalKind::Boundary) => std::cmp::Ordering::Less,
                    (IntervalKind::Boundary, IntervalKind::Diagonal) => std::cmp::Ordering::Greater,
                    (IntervalKind::Diagonal, IntervalKind::Diagonal) => b.r.cmp(&a.r),
                    (IntervalKind::Boundary, IntervalKind::Boundary) => a.r.cmp(&b.r),
                }
            }
        });

        let root = m + 1;
        let mut parent = vec![root; m + 2];
        let mut stack: Vec<ActiveChord> = Vec::new();

        for item in items.into_iter() {
            while let Some(top) = stack.last() {
                if top.r <= item.l {
                    stack.pop();
                } else {
                    break;
                }
            }

            match item.kind {
                IntervalKind::Diagonal => {
                    let p = stack.last().map(|e| e.id).unwrap_or(root);

                    parent[item.id] = p;
                    stack.push(ActiveChord::new(item.id, item.r));
                }
                IntervalKind::Boundary => {
                    let mut p = root;

                    for elem in stack.iter().rev() {
                        if elem.r >= item.r {
                            p = elem.id;
                            break;
                        }
                    }

                    parent[item.id] = p;
                }
            }
        }

        let mut graph = vec![Vec::<(usize, i64)>::new(); m + 2];

        for i in 1..=m {
            let p = parent[i];
            graph[p].push((i, capacity[i]));
            graph[i].push((p, capacity[i]));
        }

        let mut costs = vec![0; m + 2];
        let mut visited = vec![false; m + 2];
        let mut heap: BinaryHeap<Reverse<(i64, usize, usize)>> = BinaryHeap::new();

        for i in 1..=m + 1 {
            if graph[i].len() == 1 {
                let (next, w) = graph[i][0];
                heap.push(Reverse((w, i, next)));
                visited[i] = true;
            }
        }

        let add_weight =
            |parent: &Vec<usize>, costs: &mut Vec<i64>, mut u: usize, mut v: usize, w: i64| {
                if parent[v] == u {
                    std::mem::swap(&mut u, &mut v);
                }

                if w == i64::MIN {
                    costs[u] = i64::MIN;
                } else if costs[u] != i64::MIN {
                    costs[u] += w;
                }
            };

        while let Some(Reverse((w, u, v))) = heap.pop() {
            if visited[v] {
                add_weight(&parent, &mut costs, u, v, w);
                continue;
            }

            visited[v] = true;
            add_weight(&parent, &mut costs, u, v, i64::MIN);

            for &(next, cost) in graph[v].iter() {
                if next == u {
                    continue;
                }

                if visited[next] {
                    add_weight(&parent, &mut costs, v, next, w);
                } else {
                    heap.push(Reverse((w + cost, v, next)));
                }
            }
        }

        let mut tree = GomoryHuTree::new(n);

        for i in 1..=m {
            if costs[i] >= 0 {
                let (u, v) = edges[i];
                tree.graph[u].push((v, costs[i]));
                tree.graph[v].push((u, costs[i]));
            }
        }

        tree
    }
}

const MOD: i64 = 998_244_353;

// Reference: https://en.wikipedia.org/wiki/Outerplanar_graph
// Reference: https://en.wikipedia.org/wiki/Gomory%E2%80%93Hu_tree
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut edges = vec![(0, 0, 0); m];

    for i in 0..m {
        edges[i] = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
    }

    let gomory_hu_tree = GomoryHuTree::from_outer_planar(n, &edges);
    let mut edges_new = Vec::new();

    for curr in 1..=n {
        for &(next, w) in gomory_hu_tree.graph[curr].iter() {
            edges_new.push((curr, next, w));
        }
    }

    edges_new.sort_by(|a, b| b.2.cmp(&a.2));

    let mut union_find = UnionFind::new(n);
    union_find.init();

    let mut ret = 0;

    for &(u, v, w) in edges_new.iter() {
        let root_u = union_find.find(u);
        let root_v = union_find.find(v);

        if root_u == root_v {
            continue;
        }

        let size_u = union_find.size[root_u];
        let size_v = union_find.size[root_v];
        let val = (((w * size_u as i64) % MOD) * size_v as i64) % MOD;

        ret = (ret + val) % MOD;
        union_find.union(u, v);
    }

    writeln!(out, "{ret}").unwrap();
}

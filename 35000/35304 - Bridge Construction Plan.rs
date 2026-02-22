use io::Write;
use std::{
    collections::{BTreeSet, VecDeque},
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
    root: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![0; n + 1],
            size: vec![1; n + 1],
            root: vec![0; n + 1],
        }
    }

    fn init(&mut self) {
        for i in 0..self.parent.len() {
            self.parent[i] = i;
            self.root[i] = i;
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

fn build_cartesian_tree(
    graph: &Vec<Vec<usize>>,
    weights: &Vec<i64>,
    n: usize,
) -> Vec<(usize, usize)> {
    let mut order = (1..=n).collect::<Vec<_>>();
    order.sort_unstable_by_key(|&v| weights[v]);

    let mut union_find = UnionFind::new(n);
    let mut active = vec![false; n + 1];
    let mut edges = Vec::with_capacity(n - 1);

    union_find.init();

    for v in order {
        active[v] = true;

        let mut reps = Vec::new();

        for &u in graph[v].iter() {
            if active[u] {
                reps.push(union_find.find(u));
            }
        }

        for r in reps {
            edges.push((v, union_find.root[r]));
            union_find.union(v, r);

            let root_new = union_find.find(v);
            union_find.root[root_new] = v;
        }
    }

    edges
}

struct LCA {
    up: Vec<Vec<usize>>,
    depth: Vec<usize>,
    log: usize,
}

impl LCA {
    fn new(graph: &Vec<Vec<usize>>, n: usize, root: usize) -> Self {
        let mut log = 1;
        while (1usize << log) <= n {
            log += 1;
        }

        let mut up = vec![vec![0; n + 1]; log];
        let mut depth = vec![0; n + 1];

        let mut stack = Vec::new();
        stack.push((root, 0));

        while let Some((u, p)) = stack.pop() {
            up[0][u] = p;

            for &v in graph[u].iter() {
                if v == p {
                    continue;
                }

                depth[v] = depth[u] + 1;
                stack.push((v, u));
            }
        }

        for k in 1..log {
            for v in 1..=n {
                let mid = up[k - 1][v];
                up[k][v] = if mid == 0 { 0 } else { up[k - 1][mid] };
            }
        }

        Self { up, depth, log }
    }

    fn lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] < self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }

        let mut diff = self.depth[a] - self.depth[b];
        let mut k = 0;

        while diff > 0 {
            if (diff & 1) == 1 {
                a = self.up[k][a];
            }

            diff >>= 1;
            k += 1;
        }

        if a == b {
            return a;
        }

        for k in (0..self.log).rev() {
            if self.up[k][a] != self.up[k][b] {
                a = self.up[k][a];
                b = self.up[k][b];
            }
        }

        self.up[0][a]
    }
}

fn remove_degree2(graph: &Vec<Vec<usize>>, n: usize) -> (Vec<BTreeSet<usize>>, Vec<usize>) {
    let mut graph_removed = vec![BTreeSet::new(); n + 1];

    for v in 1..=n {
        for &u in graph[v].iter() {
            graph_removed[v].insert(u);
        }
    }

    let mut queue = VecDeque::new();
    let mut alive = vec![true; n + 1];

    for v in 1..=n {
        if graph_removed[v].len() == 2 {
            queue.push_back(v);
        }
    }

    while let Some(v) = queue.pop_front() {
        if !alive[v] || graph_removed[v].len() != 2 {
            continue;
        }

        let mut iter = graph_removed[v].iter();
        let a = *iter.next().unwrap();
        let b = *iter.next().unwrap();

        graph_removed[a].remove(&v);
        graph_removed[b].remove(&v);
        graph_removed[v].clear();
        alive[v] = false;

        if !graph_removed[a].contains(&b) {
            graph_removed[a].insert(b);
            graph_removed[b].insert(a);
        }

        if alive[a] && graph_removed[a].len() == 2 {
            queue.push_back(a);
        }

        if alive[b] && graph_removed[b].len() == 2 {
            queue.push_back(b);
        }
    }

    let nodes_alive = (1..=n).filter(|&v| alive[v]).collect::<Vec<_>>();

    (graph_removed, nodes_alive)
}

fn centroids(graph: &Vec<BTreeSet<usize>>, nodes_alive: &Vec<usize>, n: usize) -> Vec<usize> {
    let m = nodes_alive.len();
    let mut alive = vec![false; n + 1];

    for &v in nodes_alive.iter() {
        alive[v] = true;
    }

    let mut degree = vec![0; n + 1];
    let mut queue = VecDeque::new();

    for &v in nodes_alive.iter() {
        degree[v] = graph[v].iter().filter(|&&u| alive[u]).count();

        if degree[v] <= 1 {
            queue.push_back(v);
        }
    }

    let mut remain = m;
    let mut alive = alive.clone();

    while remain > 2 {
        let layer = queue.len();

        for _ in 0..layer {
            let v = queue.pop_front().unwrap();

            if !alive[v] {
                continue;
            }

            alive[v] = false;
            remain -= 1;

            for &u in graph[v].iter() {
                if alive[u] {
                    degree[u] -= 1;

                    if degree[u] == 1 {
                        queue.push_back(u);
                    }
                }
            }
        }
    }

    let mut ret = Vec::new();

    for &v in nodes_alive.iter() {
        if alive[v] {
            ret.push(v);
        }
    }

    ret
}

fn process_bfs(graph: &Vec<Vec<usize>>, n: usize, from: usize) -> Vec<i32> {
    let mut queue = VecDeque::new();
    let mut dist = vec![-1; n + 1];

    queue.push_back(from);
    dist[from] = 0;

    while let Some(v) = queue.pop_front() {
        for &u in graph[v].iter() {
            if dist[u] == -1 {
                dist[u] = dist[v] + 1;
                queue.push_back(u);
            }
        }
    }

    dist
}

fn next_on_path(graph: &Vec<Vec<usize>>, n: usize, from: usize, to: usize) -> usize {
    if from == to {
        return from;
    }

    let mut queue = VecDeque::new();
    let mut parent = vec![0; n + 1];

    queue.push_back(from);
    parent[from] = from;

    while let Some(v) = queue.pop_front() {
        if v == to {
            break;
        }

        for &u in graph[v].iter() {
            if parent[u] == 0 {
                parent[u] = v;
                queue.push_back(u);
            }
        }
    }

    let mut ret = to;

    while parent[ret] != from {
        ret = parent[ret];
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<i64>();
    let t = scan.token::<i64>();

    for _ in 0..t {
        if s == 1 {
            let n = scan.token::<usize>();
            let mut resources = vec![0; n + 1];

            for i in 1..=n {
                resources[i] = scan.token::<i64>();
            }

            let mut graph = vec![Vec::new(); n + 1];

            for _ in 0..n - 1 {
                let (u, v) = (scan.token::<usize>(), scan.token::<usize>());

                graph[u].push(v);
                graph[v].push(u);
            }

            let edges = build_cartesian_tree(&graph, &resources, n);
            let mut graph = vec![Vec::new(); n + 1];

            for &(u, v) in edges.iter() {
                graph[u].push(v);
                graph[v].push(u);
            }

            let mut root = 1;

            for i in 2..=n {
                if resources[i] > resources[root] {
                    root = i;
                }
            }

            let (graph_removed, nodes_alive) = remove_degree2(&graph, n);
            let (swap_a, swap_b) = if nodes_alive.len() == 2 {
                (0, 0)
            } else {
                let cents = centroids(&graph_removed, &nodes_alive, n);

                if cents.len() == 1 {
                    (root, cents[0])
                } else {
                    let c1 = cents[0];
                    let c2 = cents[1];
                    let dist = process_bfs(&graph, n, root);

                    if dist[c1] >= dist[c2] {
                        (root, c1)
                    } else {
                        (root, c2)
                    }
                }
            };

            for (u, v) in edges {
                let u = if u == swap_a {
                    swap_b
                } else if u == swap_b {
                    swap_a
                } else {
                    u
                };
                let v = if v == swap_a {
                    swap_b
                } else if v == swap_b {
                    swap_a
                } else {
                    v
                };

                writeln!(out, "{u} {v}").unwrap();
            }

            out.flush().unwrap();
        } else {
            let n = scan.token::<usize>();
            let mut edges = Vec::with_capacity(n - 1);
            let mut graph = vec![Vec::new(); n + 1];

            for _ in 0..n - 1 {
                let (u, v) = (scan.token::<usize>(), scan.token::<usize>());

                edges.push((u, v));
                graph[u].push(v);
                graph[v].push(u);
            }

            let (graph_removed, nodes_alive) = remove_degree2(&graph, n);

            let root;
            let swap;

            if nodes_alive.len() == 2 {
                let mut leaves = Vec::new();

                for v in 1..=n {
                    if graph[v].len() == 1 {
                        leaves.push(v);
                    }
                }

                writeln!(out, "? {} {}", leaves[0], leaves[1]).unwrap();
                out.flush().unwrap();

                root = scan.token::<usize>();
                swap = root;
            } else {
                let cents = centroids(&graph_removed, &nodes_alive, n);

                if cents.len() == 1 {
                    root = cents[0];

                    let a = graph[root][0];
                    let b = graph[root][1];
                    let c = graph[root][2];

                    writeln!(out, "? {a} {b}").unwrap();
                    out.flush().unwrap();

                    let q = scan.token::<usize>();

                    swap = if q == root {
                        root
                    } else if q != a && q != b {
                        q
                    } else if q == a {
                        writeln!(out, "? {b} {c}").unwrap();
                        out.flush().unwrap();

                        scan.token::<usize>()
                    } else {
                        writeln!(out, "? {a} {c}").unwrap();
                        out.flush().unwrap();

                        scan.token::<usize>()
                    }
                } else {
                    writeln!(out, "? {} {}", cents[0], cents[1]).unwrap();
                    out.flush().unwrap();

                    root = scan.token::<usize>();

                    let other = if root == cents[0] { cents[1] } else { cents[0] };
                    let next = next_on_path(&graph, n, root, other);
                    let mut neighbors = Vec::new();

                    for &u in graph[root].iter() {
                        if u != next {
                            neighbors.push(u);
                        }
                    }

                    writeln!(out, "? {} {}", neighbors[0], neighbors[1]).unwrap();
                    out.flush().unwrap();

                    swap = scan.token::<usize>();
                }
            }

            let map = |x: usize| -> usize {
                if x == root {
                    swap
                } else if x == swap {
                    root
                } else {
                    x
                }
            };

            let mut graph = vec![Vec::new(); n + 1];

            for &(u, v) in edges.iter() {
                let (u, v) = (map(u), map(v));

                graph[u].push(v);
                graph[v].push(u);
            }

            let lca = LCA::new(&graph, n, root);

            writeln!(out, "!").unwrap();

            for i in 1..=n {
                for j in 1..=n {
                    let v = lca.lca(i, j);
                    write!(out, "{v} ").unwrap();
                }

                writeln!(out).unwrap();
            }

            out.flush().unwrap();
        }
    }
}

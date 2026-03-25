use io::Write;
use std::{collections::BinaryHeap, io, str};

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

struct LCA {
    up: Vec<Vec<usize>>,
    depth: Vec<usize>,
    log: usize,
}

impl LCA {
    fn new(graph: &Vec<Vec<(usize, i64)>>, n: usize, root: usize) -> Self {
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

            for &(v, _) in graph[u].iter() {
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

const INF: i64 = i64::MAX / 4;

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Vec<i64> {
    let mut ret = vec![INF; graph.len()];
    ret[from] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, from));

    while !queue.is_empty() {
        let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
        cost_curr *= -1;

        if ret[vertex_curr] < cost_curr {
            continue;
        }

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;

            cost_next += cost_curr;

            if ret[vertex_next] > cost_next {
                ret[vertex_next] = cost_next;
                queue.push((-cost_next, vertex_next));
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n + 1];
    let mut tree = vec![Vec::new(); n + 1];
    let mut nodes_extra = Vec::new();

    let mut union_find = UnionFind::new(n);
    union_find.init();

    for _ in 0..m {
        let (u, v, w) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        graph[u].push((v, w));
        graph[v].push((u, w));

        if union_find.union(u, v) {
            tree[u].push((v, w));
            tree[v].push((u, w));
        } else {
            nodes_extra.push(u);
            nodes_extra.push(v);
        }
    }

    let lca = LCA::new(&tree, n, 1);
    let mut dist_root = vec![0; n + 1];
    let mut stack = Vec::new();

    stack.push((1, 0));

    while let Some((u, parent)) = stack.pop() {
        for &(v, w) in tree[u].iter() {
            if v == parent {
                continue;
            }

            dist_root[v] = dist_root[u] + w;
            stack.push((v, u));
        }
    }

    let dists = nodes_extra
        .iter()
        .map(|&node| process_dijkstra(&graph, node))
        .collect::<Vec<_>>();

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (s, e) = (scan.token::<usize>(), scan.token::<usize>());

        let center = lca.lca(s, e);
        let mut ret = dist_root[s] + dist_root[e] - 2 * dist_root[center];

        for dist in dists.iter() {
            ret = ret.min(dist[s] + dist[e]);
        }

        writeln!(out, "{ret}").unwrap();
    }
}

use io::Write;
use std::{collections::VecDeque, io, str};

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
struct Edge {
    to: usize,
    rev: usize,
    capacity: i64,
}

impl Edge {
    fn new(to: usize, rev: usize, capacity: i64) -> Self {
        Self { to, rev, capacity }
    }
}

struct Dinic {
    graph: Vec<Vec<Edge>>,
    source: usize,
    sink: usize,
    check: Vec<i64>,
    done: Vec<usize>,
}

impl Dinic {
    fn new(n: usize, source: usize, sink: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n],
            source,
            sink,
            check: vec![-1; n],
            done: vec![0; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, capacity: i64) {
        let orig = self.graph[v].len();
        let dest = self.graph[u].len();

        self.graph[u].push(Edge::new(v, orig, capacity));
        self.graph[v].push(Edge::new(u, dest, 0));
    }

    fn process_bfs(&mut self) -> bool {
        self.check.fill(-1);

        let mut queue = VecDeque::new();

        queue.push_back(self.source);
        self.check[self.source] = 0;

        while let Some(u) = queue.pop_front() {
            for &e in self.graph[u].iter() {
                if e.capacity > 0 && self.check[e.to] < 0 {
                    queue.push_back(e.to);
                    self.check[e.to] = self.check[u] + 1;
                }
            }
        }

        self.check[self.sink] >= 0
    }

    fn process_dfs(&mut self, idx: usize, flow: i64) -> i64 {
        if idx == self.sink {
            return flow;
        }

        let n = self.graph[idx].len();

        while self.done[idx] < n {
            let edge = self.graph[idx][self.done[idx]];

            if edge.capacity > 0 && self.check[edge.to] == self.check[idx] + 1 {
                let flow_current = self.process_dfs(edge.to, flow.min(edge.capacity));

                if flow_current > 0 {
                    self.graph[idx][self.done[idx]].capacity -= flow_current;
                    self.graph[edge.to][edge.rev].capacity += flow_current;

                    return flow_current;
                }
            }

            self.done[idx] += 1;
        }

        0
    }

    fn get_flow(&mut self) -> i64 {
        let mut flow_total = 0;

        while self.process_bfs() {
            self.done.fill(0);

            loop {
                let flow_current = self.process_dfs(self.source, i64::MAX);

                if flow_current == 0 {
                    break;
                }

                flow_total += flow_current;
            }
        }

        flow_total
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
    let mut dungeon = vec![vec![' '; 2 * w + 1]; 2 * h + 1];

    for i in 0..2 * h + 1 {
        let line = scan.token::<String>();

        for (c, j) in line.chars().enumerate() {
            dungeon[i][c] = j;
        }
    }

    let mut is_room = vec![false; h * w];
    let mut parity = vec![0; h * w];
    let mut cnt_even = 0;
    let mut cnt_odd = 0;

    for i in 0..h {
        for j in 0..w {
            if dungeon[2 * i + 1][2 * j + 1] == '.' {
                let p = (i + j) % 2;

                is_room[i * w + j] = true;
                parity[i * w + j] = p;

                if p == 0 {
                    cnt_even += 1;
                } else {
                    cnt_odd += 1;
                }
            }
        }
    }

    if cnt_even == cnt_odd {
        writeln!(out, "No").unwrap();
        return;
    }

    let parity_right = if cnt_even > cnt_odd { 0 } else { 1 };
    let mut is_left = vec![false; h * w];
    let mut is_right = vec![false; h * w];
    let mut vertices_left = Vec::new();
    let mut vertices_right = Vec::new();

    for i in 0..h * w {
        if !is_room[i] {
            continue;
        }

        if parity[i] == parity_right {
            is_right[i] = true;
            vertices_right.push(i);
        } else {
            is_left[i] = true;
            vertices_left.push(i);
        }
    }

    let mut graph = vec![Vec::new(); h * w];
    let mut edges = Vec::new();

    for i in 0..h {
        for j in 0..w {
            let curr = i * w + j;

            if !is_room[curr] {
                continue;
            }

            if j + 1 < w {
                let next = i * w + (j + 1);

                if is_room[next] && dungeon[2 * i + 1][2 * j + 2] == '.' {
                    graph[curr].push(next);
                    graph[next].push(curr);
                    edges.push((curr, next));
                }
            }

            if i + 1 < h {
                let next = (i + 1) * w + j;

                if is_room[next] && dungeon[2 * i + 2][2 * j + 1] == '.' {
                    graph[curr].push(next);
                    graph[next].push(curr);
                    edges.push((curr, next));
                }
            }
        }
    }

    let source = h * w;
    let sink = h * w + 1;
    let mut maximum_flow = Dinic::new(h * w + 2, source, sink);

    for &u in vertices_left.iter() {
        maximum_flow.add_edge(source, u, 1);
    }

    for &v in vertices_right.iter() {
        maximum_flow.add_edge(v, sink, 1);
    }

    for &(a, b) in edges.iter() {
        if is_left[a] && is_right[b] {
            maximum_flow.add_edge(a, b, 1);
        } else if is_right[a] && is_left[b] {
            maximum_flow.add_edge(b, a, 1);
        }
    }

    let flow = maximum_flow.get_flow();

    if flow != vertices_left.len() as i64 {
        writeln!(out, "No").unwrap();
        return;
    }

    let mut mate = vec![usize::MAX; h * w];

    for &u in vertices_left.iter() {
        for edge in maximum_flow.graph[u].iter() {
            let v = edge.to;

            if v < h * w && is_right[v] && maximum_flow.graph[v][edge.rev].capacity > 0 {
                mate[u] = v;
                mate[v] = u;
                break;
            }
        }
    }

    let mut vertices_start = Vec::new();

    for &v in vertices_right.iter() {
        if mate[v] == usize::MAX {
            vertices_start.push(v);
        }
    }

    let mut union_find = UnionFind::new(h * w);
    let mut visited = vec![false; h * w];
    let mut edges_ret = Vec::new();

    union_find.init();

    for start in vertices_start {
        if visited[start] {
            continue;
        }

        visited[start] = true;

        let mut stack = Vec::new();
        stack.push(start);

        while let Some(u) = stack.pop() {
            if is_left[u] {
                if mate[u] != usize::MAX && !visited[mate[u]] {
                    union_find.union(u, mate[u]);
                    visited[mate[u]] = true;
                    edges_ret.push((u, mate[u]));
                    stack.push(mate[u]);
                }
            } else if is_right[u] {
                for &v in graph[u].iter() {
                    if !is_left[v] || visited[v] || mate[u] == v {
                        continue;
                    }

                    union_find.union(u, v);
                    visited[v] = true;
                    edges_ret.push((u, v));
                    stack.push(v);
                }
            }
        }
    }

    for i in 0..h * w {
        if is_room[i] && !visited[i] {
            writeln!(out, "No").unwrap();
            return;
        }
    }

    for &(u, v) in edges.iter() {
        if union_find.find(u) != union_find.find(v) {
            union_find.union(u, v);
            edges_ret.push((u, v));
        }
    }

    let mut ret = vec![vec!['#'; 2 * w + 1]; 2 * h + 1];

    for i in 0..h {
        for j in 0..w {
            if is_room[i * w + j] {
                ret[2 * i + 1][2 * j + 1] = '.';
            }
        }
    }

    for &(u, v) in edges_ret.iter() {
        let (uy, ux) = (u / w, u % w);
        let (vy, vx) = (v / w, v % w);

        ret[uy + vy + 1][ux + vx + 1] = '.';
    }

    writeln!(out, "Yes").unwrap();

    for i in 0..2 * h + 1 {
        for j in 0..2 * w + 1 {
            write!(out, "{}", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}

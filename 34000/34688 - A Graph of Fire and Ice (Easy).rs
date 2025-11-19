use io::Write;
use std::{cmp::Reverse, io, str};

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

#[derive(Debug, Default, Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
    w: i64,
}

impl Edge {
    fn new(u: usize, v: usize, w: i64) -> Self {
        Self { u, v, w }
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

struct Work {
    visited: Vec<i64>,
    stamp: i64,
    parent: Vec<usize>,
    depth: Vec<i32>,
    time_in: Vec<usize>,
    odd: Vec<i32>,
    even: Vec<i32>,
}

impl Work {
    fn new(n: usize) -> Self {
        Self {
            visited: vec![0; n + 1],
            stamp: 0,
            parent: vec![0; n + 1],
            depth: vec![0; n + 1],
            time_in: vec![0; n + 1],
            odd: vec![0; n + 1],
            even: vec![0; n + 1],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Frame {
    u: usize,
    idx: usize,
}

impl Frame {
    fn new(u: usize, idx: usize) -> Self {
        Self { u, idx }
    }
}

fn check(graph: &Vec<Vec<(usize, i64)>>, work: &mut Work, threshold: i64, n: usize) -> bool {
    work.stamp += 1;

    let mut cnt_components_odd = 0;
    let mut time = 0;
    let mut stack = Vec::new();
    let mut order = Vec::new();

    for i in 1..=n {
        if work.visited[i] == work.stamp {
            continue;
        }

        time += 1;

        work.visited[i] = work.stamp;
        work.parent[i] = 0;
        work.depth[i] = 0;
        work.time_in[i] = time;
        work.odd[i] = 0;
        work.even[i] = 0;

        let mut cnt_component_odd = 0;

        stack.clear();
        order.clear();
        stack.push(Frame::new(i, 0));

        while let Some(top) = stack.last_mut() {
            if top.idx < graph[top.u].len() {
                let (v, w) = graph[top.u][top.idx];

                top.idx += 1;

                if w <= threshold {
                    continue;
                }

                if v == work.parent[top.u] {
                    continue;
                }

                if work.visited[v] != work.stamp {
                    time += 1;

                    work.visited[v] = work.stamp;
                    work.parent[v] = top.u;
                    work.depth[v] = work.depth[top.u] + 1;
                    work.time_in[v] = time;
                    work.odd[v] = 0;
                    work.even[v] = 0;

                    stack.push(Frame::new(v, 0));
                } else {
                    if work.time_in[v] < work.time_in[top.u] {
                        if (work.depth[top.u] ^ work.depth[v]) & 1 == 0 {
                            cnt_component_odd += 1;
                            work.odd[top.u] += 1;
                            work.odd[v] -= 1;
                        } else {
                            work.even[top.u] += 1;
                            work.even[v] -= 1;
                        }
                    }
                }
            } else {
                order.push(top.u);
                stack.pop();
            }
        }

        for &u in order.iter() {
            if work.parent[u] != 0 {
                let p = work.parent[u];

                work.odd[p] += work.odd[u];
                work.even[p] += work.even[u];
            }
        }

        if cnt_component_odd > 0 {
            cnt_components_odd += 1;

            if cnt_components_odd > 1 {
                return false;
            }

            if cnt_component_odd >= 2 {
                let mut flag = false;

                for &u in order.iter() {
                    if work.parent[u] != 0 && work.odd[u] == cnt_component_odd && work.even[u] == 0
                    {
                        flag = true;
                        break;
                    }
                }

                if !flag {
                    return false;
                }
            }
        }
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n + 1];
    let mut edges = vec![Edge::default(); m];

    for i in 0..m {
        let (u, v, r) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        graph[u].push((v, r));
        graph[v].push((u, r));
        edges[i] = Edge::new(u, v, r);
    }

    let mut idx = (0..m).collect::<Vec<_>>();
    idx.sort_unstable_by_key(|&i| Reverse(edges[i].w));

    let mut union_find = UnionFind::new(n);
    union_find.init();

    let mut cnt_component = n;
    let mut threshold = 0;

    for i in idx {
        let edge = edges[i];

        if union_find.union(edge.u, edge.v) {
            threshold = edge.w;
            cnt_component -= 1;

            if cnt_component == 1 {
                break;
            }
        }
    }

    let mut work = Work::new(n);

    if !check(&graph, &mut work, threshold, n) {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut left = 0;
    let mut right = threshold;

    while left < right {
        let mid = (left + right) / 2;

        if check(&graph, &mut work, mid, n) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    writeln!(out, "{left}").unwrap();
}

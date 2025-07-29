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

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
    t: usize,
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![0; n],
            size: vec![0; n],
            components: n - 1,
        }
    }

    fn init(&mut self) {
        for i in 0..self.parent.len() {
            self.parent[i] = i;
            self.size[i] = 1;
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }

        x
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
        self.size[root_y] = 0;
        self.components -= 1;

        true
    }
}

fn check(edges: &Vec<Edge>, idx: usize, n: usize) -> bool {
    let mut union_find = UnionFind::new(n + 1);

    union_find.init();

    for &edge in edges[..idx].iter() {
        if !union_find.union(edge.u, edge.v) {
            return false;
        }
    }

    if union_find.components == 1 {
        return true;
    }

    for &edge in edges[idx..].iter() {
        union_find.union(edge.u, edge.v);

        if union_find.components == 1 {
            return true;
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut edges = Vec::with_capacity(m);
    let mut days_open = vec![false; n + 1];

    for _ in 0..m {
        let (u, v, t) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        edges.push(Edge { u, v, t });

        if t <= n {
            days_open[t] = true;
        }
    }

    edges.sort_by(|a, b| a.t.cmp(&b.t));

    let mut left = 1;
    let mut right = 1;
    let mut ret = 1;

    while right < n && days_open[right] {
        right += 1;
    }

    while left <= right {
        let mid = (left + right) / 2;
        let idx = edges.partition_point(|e| e.t < mid);

        if check(&edges, idx, n) {
            ret = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}

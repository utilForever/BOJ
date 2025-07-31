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
    w: f64,
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![0; n],
            size: vec![0; n],
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

        true
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut points = vec![(0, 0); n];

    for i in 0..n {
        points[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut dists = vec![vec![f64::MAX; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let dx = (points[i].0 - points[j].0) as f64;
            let dy = (points[i].1 - points[j].1) as f64;
            dists[i][j] = (dx * dx + dy * dy).sqrt();
        }
    }

    for _ in 0..m {
        let (x, y) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        dists[x][y] = 0.0;
        dists[y][x] = 0.0;
    }

    let mut edges = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in i + 1..n {
            edges.push(Edge {
                u: i,
                v: j,
                w: dists[i][j],
            });
        }
    }

    edges.sort_by(|a, b| a.w.partial_cmp(&b.w).unwrap());

    let mut union_find = UnionFind::new(n);
    union_find.init();

    let mut ret = 0.0;

    for edge in edges {
        let (u, v, w) = (edge.u, edge.v, edge.w);

        if union_find.union(u, v) {
            ret += w;
        }
    }

    writeln!(out, "{ret:.2}").unwrap();
}

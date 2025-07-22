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

#[derive(Clone)]
struct Edge {
    u: usize,
    v: usize,
    w: f64,
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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (s, p) = (scan.token::<usize>(), scan.token::<usize>());
        let mut points = vec![(0.0, 0.0); p];

        for i in 0..p {
            points[i] = (scan.token::<f64>(), scan.token::<f64>());
        }

        let mut edges = Vec::with_capacity(p * (p - 1) / 2);

        for i in 0..p {
            for j in i + 1..p {
                let dx = points[i].0 - points[j].0;
                let dy = points[i].1 - points[j].1;
                let dist = (dx * dx + dy * dy).sqrt();

                edges.push(Edge {
                    u: i,
                    v: j,
                    w: dist,
                });
            }
        }

        edges.sort_by(|a, b| a.w.partial_cmp(&b.w).unwrap());

        let mut union_find = UnionFind::new(p);
        union_find.init();

        let mut picked = 0;
        let mut ret = 0.0;

        for edge in edges {
            if union_find.find(edge.u) != union_find.find(edge.v) {
                union_find.union(edge.u, edge.v);
                picked += 1;
                ret = edge.w;

                if picked == p - s {
                    break;
                }
            }
        }

        writeln!(out, "{:.2}", ret).unwrap();
    }
}

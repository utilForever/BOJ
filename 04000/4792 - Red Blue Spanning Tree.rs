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
    flag: usize,
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

fn process_kruskal(n: usize, edges: &[Edge], blue_first: bool) -> usize {
    let mut union_find = UnionFind::new(n);
    union_find.init();

    let mut ret = 0;

    if blue_first {
        for edge in edges.iter().filter(|e| e.flag == 1) {
            if union_find.union(edge.u, edge.v) {
                ret += 1;
            }
        }

        for edge in edges.iter().filter(|e| e.flag == 0) {
            union_find.union(edge.u, edge.v);
        }
    } else {
        for edge in edges.iter().filter(|e| e.flag == 0) {
            union_find.union(edge.u, edge.v);
        }

        for edge in edges.iter().filter(|e| e.flag == 1) {
            if union_find.union(edge.u, edge.v) {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let (n, m, k) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if n == 0 && m == 0 && k == 0 {
            break;
        }

        let mut edges = Vec::with_capacity(m);

        for _ in 0..m {
            let (c, f, t) = (
                scan.token::<char>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
            );
            edges.push(Edge {
                u: f - 1,
                v: t - 1,
                flag: if c == 'B' { 1 } else { 0 },
            });
        }

        let blue_min = process_kruskal(n, &edges, false);
        let blue_max = process_kruskal(n, &edges, true);

        writeln!(
            out,
            "{}",
            if k >= blue_min && k <= blue_max { 1 } else { 0 }
        )
        .unwrap();
    }
}

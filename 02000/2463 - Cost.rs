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
    x: usize,
    y: usize,
    w: i64,
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

const MOD: i64 = 1_000_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut edges = Vec::with_capacity(m);
    let mut sum = 0;

    for _ in 0..m {
        let (x, y, w) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        edges.push(Edge { x, y, w });
        sum += w;
    }

    edges.sort_unstable_by(|a, b| b.w.cmp(&a.w));

    let mut union_find = UnionFind::new(n + 1);
    union_find.init();

    let mut ret = 0;

    for &edge in edges.iter() {
        let root_x = union_find.find(edge.x);
        let root_y = union_find.find(edge.y);

        if root_x == root_y {
            sum -= edge.w;
            continue;
        }

        let size_a = union_find.size[root_x] as i64;
        let size_b = union_find.size[root_y] as i64;

        ret = (ret + sum * size_a * size_b) % MOD;
        sum -= edge.w;
        
        union_find.union(edge.x, edge.y);
    }

    writeln!(out, "{ret}").unwrap();
}

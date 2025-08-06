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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut edges = vec![(0, 0, 0); n - 1];
    let mut queries = vec![(0, 0, 0); q];

    for i in 0..n - 1 {
        edges[i] = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
    }

    for i in 0..q {
        queries[i] = (scan.token::<usize>(), scan.token::<usize>(), i);
    }

    edges.sort_unstable_by_key(|&(_, _, r)| r);
    queries.sort_unstable_by(|&a, &b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)).then(b.2.cmp(&a.2)));

    let mut union_find = UnionFind::new(n + 1);
    let mut ret = vec![0; q];

    union_find.init();

    for (k, v, idx) in queries {
        while !edges.is_empty() && edges.last().unwrap().2 >= k {
            let (u, v, _) = edges.pop().unwrap();
            union_find.union(u, v);
        }

        let pos = union_find.find(v);
        ret[idx] = union_find.size[pos] - 1;
    }

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}

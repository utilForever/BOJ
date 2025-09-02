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
    dist: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![0; n],
            dist: vec![0; n],
            size: vec![1; n],
        }
    }

    fn init(&mut self) {
        for i in 0..self.parent.len() {
            self.parent[i] = i;
            self.dist[i] = 0;
            self.size[i] = 1;
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut path = Vec::new();
        let mut u = x;

        while self.parent[u] != u {
            path.push(u);
            u = self.parent[u];
        }

        let root = u;

        let mut sum = 0;

        for &node in path.iter().rev() {
            sum += self.dist[node];
            self.parent[node] = root;
            self.dist[node] = sum;
        }

        root
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        self.parent[root_x] = root_y;
        self.dist[root_x] = self.size[root_y];
        self.size[root_y] += self.size[root_x];
    }

    fn count_below(&mut self, x: usize) -> usize {
        self.find(x);
        self.dist[x]
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let p = scan.token::<usize>();
    let mut union_find = UnionFind::new(30001);

    union_find.init();

    for _ in 0..p {
        let op = scan.token::<char>();

        if op == 'M' {
            let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
            union_find.union(x, y);
        } else {
            let x = scan.token::<usize>();
            writeln!(out, "{}", union_find.count_below(x)).unwrap();
        }
    }
}

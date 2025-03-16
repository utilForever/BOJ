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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut degree = vec![0; n];
    let mut union_find = UnionFind::new(n);

    union_find.init();

    for _ in 0..m {
        let (u, v) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);

        degree[u] += 1;
        degree[v] += 1;
        union_find.union(u, v);
    }

    if degree.iter().any(|&x| x > 3) {
        writeln!(out, "NO").unwrap();
        return;
    }

    for i in 0..n - 1 {
        if union_find.find(i) != union_find.find(i + 1) {
            writeln!(out, "NO").unwrap();
            return;
        }
    }

    let degree_zero = degree.iter().filter(|&&x| x == 0).count();
    let degree_one = degree.iter().filter(|&&x| x == 1).count();
    let degree_three = degree.iter().filter(|&&x| x == 3).count();

    writeln!(
        out,
        "{}",
        if degree_zero == 0 && degree_one == 1 && degree_three == 1 {
            "YES"
        } else {
            "NO"
        }
    )
    .unwrap();
}

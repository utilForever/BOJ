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
    diff: Vec<i64>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![0; n],
            size: vec![0; n],
            diff: vec![0; n],
            components: n - 1,
        }
    }

    fn init(&mut self) {
        for i in 0..self.parent.len() {
            self.parent[i] = i;
            self.size[i] = 1;
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        let par = self.parent[x];
        let root = self.find(par);

        self.diff[x] += self.diff[par];
        self.parent[x] = root;

        root
    }

    #[allow(dead_code)]
    fn union(&mut self, a: usize, b: usize, w: i64) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        if self.size[root_a] >= self.size[root_b] {
            self.parent[root_b] = root_a;
            self.diff[root_b] = w + self.diff[a] - self.diff[b];
            self.size[root_a] += self.size[root_b];
            self.size[root_b] = 0;
        } else {
            self.parent[root_a] = root_b;
            self.diff[root_a] = -w + self.diff[b] - self.diff[a];
            self.size[root_b] += self.size[root_a];
            self.size[root_a] = 0;
        }

        self.components -= 1;
    }

    fn diff_between(&mut self, a: usize, b: usize) -> Option<i64> {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a != root_b {
            return None;
        }

        Some(self.diff[b] - self.diff[a])
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

        if n == 0 && m == 0 {
            break;
        }

        let mut union_find = UnionFind::new(n + 1);
        union_find.init();

        for _ in 0..m {
            let op = scan.token::<char>();

            if op == '!' {
                let (a, b, w) = (
                    scan.token::<usize>(),
                    scan.token::<usize>(),
                    scan.token::<i64>(),
                );
                union_find.union(a, b, w);
            } else {
                let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

                if let Some(diff) = union_find.diff_between(a, b) {
                    writeln!(out, "{diff}").unwrap();
                } else {
                    writeln!(out, "UNKNOWN").unwrap();
                }
            }
        }
    }
}

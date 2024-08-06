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
}

fn find(parent: &mut Vec<usize>, node: usize) -> usize {
    if parent[node] == 0 {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn process_union(parent: &mut Vec<usize>, mut a: usize, mut b: usize) -> bool {
    a = find(parent, a);
    b = find(parent, b);

    if a == b {
        false
    } else {
        parent[a] = b;
        true
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut edges = vec![(0, 0, 0); m];

    for i in 0..m {
        edges[i] = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
    }

    edges.sort_by(|a, b| a.2.cmp(&b.2));

    let mut parent = vec![0; n];
    let mut edges_mst = Vec::new();

    for edge in edges {
        if process_union(&mut parent, edge.0, edge.1) {
            edges_mst.push(edge);
        }
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let mut edges_cloned = edges_mst.clone();

        for i in 1..n {
            edges_cloned.push((0, i, scan.token::<i64>()));
        }

        edges_cloned.sort_by(|a, b| a.2.cmp(&b.2));

        let mut parent_cloned = vec![0; n];

        let mut ret = 0;
        let mut idx = 0;

        while idx < edges_cloned.len() {
            if process_union(&mut parent_cloned, edges_cloned[idx].0, edges_cloned[idx].1) {
                ret += edges_cloned[idx].2;
            }

            idx += 1;
        }

        writeln!(out, "{ret}").unwrap();
    }
}
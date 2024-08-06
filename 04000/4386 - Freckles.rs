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
        parent[node] = node;
        node
    } else if parent[node] == node {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn process_union(parent: &mut Vec<usize>, mut a: usize, mut b: usize) {
    a = find(parent, a);
    b = find(parent, b);

    if a == b {
        return;
    }

    parent[a] = b;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut vertices = vec![(0.0, 0.0); n];
    let mut edges = Vec::new();

    for i in 0..n {
        vertices[i] = (scan.token::<f64>(), scan.token::<f64>());
    }

    for i in 0..n - 1 {
        for j in i + 1..n {
            let dist = (vertices[i].0 - vertices[j].0).hypot(vertices[i].1 - vertices[j].1);
            edges.push((i + 1, j + 1, dist));
        }
    }

    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut root = vec![0; n + 1];
    let mut visited = vec![false; n + 1];
    let mut sum = 0.0;

    for i in 0..edges.len() {
        if find(&mut root, edges[i].0) == find(&mut root, edges[i].1) {
            continue;
        }

        visited[edges[i].0] = true;
        visited[edges[i].1] = true;

        process_union(&mut root, edges[i].0, edges[i].1);

        sum += edges[i].2;
    }

    writeln!(out, "{:.2}", sum).unwrap();
}

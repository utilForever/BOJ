use io::Write;
use std::{collections::BTreeSet, io, str};

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
    if parent[node] == node {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn process_union(
    parent: &mut Vec<usize>,
    cnt_vertices: &mut Vec<i64>,
    is_tree: &mut Vec<bool>,
    a: usize,
    b: usize,
) -> bool {
    let a = find(parent, a);
    let b = find(parent, b);

    if a == b {
        is_tree[a] = false;
    } else {
        parent[b] = parent[a];
        cnt_vertices[a] += cnt_vertices[b];
        is_tree[a] = if is_tree[a] && is_tree[b] {
            true
        } else {
            false
        };
    }

    is_tree[a]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut parent = vec![0; n + 1];
    let mut cnt_vertices = vec![0; n + 1];
    let mut is_tree = vec![false; n + 1];
    let mut vals = BTreeSet::new();

    for i in 1..=n {
        parent[i] = i;
        cnt_vertices[i] = -1;
        is_tree[i] = true;

        vals.insert((-1, i));
    }

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let (mut u, mut v) = (scan.token::<usize>(), scan.token::<usize>());
            u = find(&mut parent, u);
            v = find(&mut parent, v);

            if u > v {
                std::mem::swap(&mut u, &mut v);
            }

            vals.remove(&(cnt_vertices[u], u));
            vals.remove(&(cnt_vertices[v], v));

            if process_union(&mut parent, &mut cnt_vertices, &mut is_tree, u, v) {
                vals.insert((cnt_vertices[u], u));
            }
        } else {
            let ret = *vals.iter().next().unwrap();
            writeln!(out, "{}", ret.1).unwrap();

            vals.remove(&ret);
        }
    }
}

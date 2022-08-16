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
    if parent[node] == node {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn process_union(parent: &mut Vec<usize>, num_class: &mut Vec<usize>, mut a: usize, mut b: usize) {
    a = find(parent, a);
    b = find(parent, b);

    if a == b {
        return;
    }

    parent[a] = b;
    num_class[b] += num_class[a];
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut parent = vec![0; n + 1];
    let mut num_class = vec![1; n + 1];

    for i in 1..=n {
        parent[i] = i;
    }

    let mut a = vec![0; n + 1];

    for i in 1..=n {
        a[i] = scan.token::<usize>();
    }

    let mut vertices = Vec::new();

    for i in 1..=n {
        vertices.push((a[i], i));
    }

    vertices.sort_by(|a, b| b.0.cmp(&a.0));

    let mut edges = vec![Vec::new(); n + 1];

    for _ in 1..n {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());

        edges[u].push(v);
        edges[v].push(u);
    }

    let mut ret = 0;

    for i in 0..n {
        let cur_node = vertices[i].1;

        for &next_node in edges[cur_node].iter() {
            if a[next_node] >= a[cur_node] {
                process_union(&mut parent, &mut num_class, cur_node, next_node);
            }
        }

        ret = ret.max(a[cur_node] * num_class[find(&mut parent, cur_node)]);
    }

    writeln!(out, "{}", ret).unwrap();
}

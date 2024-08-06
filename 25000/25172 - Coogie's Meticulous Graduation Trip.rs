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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut parent = vec![0; n + 1];
    let mut is_included = vec![false; n + 1];

    for i in 1..=n {
        parent[i] = i;
    }

    let mut queries = vec![0; n];

    for i in 0..n {
        queries[i] = scan.token::<usize>();
    }

    let mut ret = Vec::new();
    let mut num_groups = 0;

    for query in queries.iter().rev() {
        num_groups += 1;
        is_included[*query] = true;

        for vertex in graph[*query].iter() {
            if !is_included[*vertex] {
                continue;
            }

            if find(&mut parent, *query) != find(&mut parent, *vertex) {
                process_union(&mut parent, *query, *vertex);
                num_groups -= 1;
            }
        }

        ret.push(num_groups == 1);
    }

    for val in ret.iter().rev() {
        writeln!(out, "{}", if *val { "CONNECT" } else { "DISCONNECT" }).unwrap();
    }

    // In last query, always all vertices are disconnected
    writeln!(out, "DISCONNECT").unwrap();
}

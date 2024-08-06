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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let mut edges = Vec::new();

        for i in 1..n {
            let (u, c) = (scan.token::<usize>(), scan.token::<i64>());
            edges.push((i, u, c));
        }

        let mut ret = 0;

        for _ in 0..m {
            let (u, v, c) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            edges.push((u, v, c));

            let mut parent = vec![0; n + 1];
            let mut cost_total = 0;

            for i in 1..=n {
                parent[i] = i;
            }

            edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

            for i in 0..edges.len() {
                if find(&mut parent, edges[i].0) == find(&mut parent, edges[i].1) {
                    continue;
                }

                process_union(&mut parent, edges[i].0, edges[i].1);
                cost_total += edges[i].2;
            }

            ret = ret ^ cost_total;
        }

        writeln!(out, "{ret}").unwrap();
    }
}

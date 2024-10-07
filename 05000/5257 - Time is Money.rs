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

struct Edge {
    u: usize,
    v: usize,
    t: i64,
    c: i64,
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
    let mut edges = Vec::with_capacity(m);

    for _ in 0..m {
        let (u, v, t, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        edges.push(Edge { u, v, t, c });
    }

    let mut value_optimal = i64::MAX;
    let mut time_sum_optimal = 0;
    let mut money_sum_optimal = 0;
    let mut links_optimal = Vec::new();
    let mut slope = 0.0;

    while slope <= 10.0 {
        let mut parent = (0..n).collect::<Vec<usize>>();
        let mut time_sum_local = 0;
        let mut money_sum_local = 0;
        let mut links_local = Vec::new();

        edges.sort_by(|a, b| {
            let a_weighted = (a.t as f64) + slope * (a.c as f64);
            let b_weighted = (b.t as f64) + slope * (b.c as f64);

            a_weighted.partial_cmp(&b_weighted).unwrap()
        });

        for edge in edges.iter() {
            let root_u = find(&mut parent, edge.u);
            let root_v = find(&mut parent, edge.v);

            if root_u != root_v {
                process_union(&mut parent, root_u, root_v);

                time_sum_local += edge.t;
                money_sum_local += edge.c;

                links_local.push((edge.u, edge.v));

                if links_local.len() == n - 1 {
                    break;
                }
            }
        }

        if links_local.len() != n - 1 {
            slope += 0.01;
            continue;
        }

        let value_local = time_sum_local * money_sum_local;

        if value_local < value_optimal {
            value_optimal = value_local;
            time_sum_optimal = time_sum_local;
            money_sum_optimal = money_sum_local;
            std::mem::swap(&mut links_optimal, &mut links_local);
        }

        slope += 0.01;
    }

    writeln!(out, "{time_sum_optimal} {money_sum_optimal}").unwrap();

    for (x, y) in links_optimal {
        writeln!(out, "{x} {y}").unwrap();
    }
}

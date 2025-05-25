use io::Write;
use std::{
    collections::{HashSet, VecDeque},
    io, str,
};

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

const INF: usize = 1 << 60;

#[derive(Clone, Copy)]
struct Edge {
    a: usize,
    b: usize,
}

impl Edge {
    fn new(a: usize, b: usize) -> Self {
        Self { a, b }
    }
}

fn process_bfs_parity(graph: &Vec<Vec<(usize, usize)>>, start: usize, n: usize) -> Vec<[usize; 2]> {
    let mut dist = vec![[INF; 2]; n + 1];
    let mut queue = VecDeque::new();

    dist[start][0] = 0;
    queue.push_back((start, 0));

    while let Some((node, parity)) = queue.pop_front() {
        let dist_curr = dist[node][parity];

        for &(next, _) in graph[node].iter() {
            let parity_next = parity ^ 1;

            if dist[next][parity_next] == INF {
                dist[next][parity_next] = dist_curr + 1;
                queue.push_back((next, parity_next));
            }
        }
    }

    dist
}

fn process_shortest_cycle_parity(
    graph: &Vec<Vec<(usize, usize)>>,
    edges: &Vec<Edge>,
    n: usize,
) -> (usize, usize) {
    let mut cycle_even = INF;
    let mut cycle_odd = INF;
    let mut visited = HashSet::new();

    for &edge in edges.iter() {
        let mut edge = (edge.a, edge.b);

        if edge.0 > edge.1 {
            std::mem::swap(&mut edge.0, &mut edge.1);
        }

        if !visited.insert(edge) {
            cycle_even = 2;
        }
    }

    let mut dist = vec![[INF; 2]; n + 1];
    let mut queue = VecDeque::new();

    for (idx, &Edge { a, b }) in edges.iter().enumerate() {
        dist.fill([INF; 2]);
        queue.clear();

        dist[a][0] = 0;
        queue.push_back((a, 0));

        while let Some((node, parity)) = queue.pop_front() {
            let dist_curr = dist[node][parity];

            for &(next, idx_next) in graph[node].iter() {
                if idx_next == idx {
                    continue;
                }

                let parity_next = parity ^ 1;

                if dist[next][parity_next] == INF {
                    dist[next][parity_next] = dist_curr + 1;
                    queue.push_back((next, parity_next));
                }
            }
        }

        for parity in 0..2 {
            if dist[b][parity] == INF {
                continue;
            }

            let len = dist[b][parity] + 1;

            if len & 1 == 0 {
                cycle_even = cycle_even.min(len);
            } else {
                cycle_odd = cycle_odd.min(len);
            }
        }
    }

    (cycle_even, cycle_odd)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, a, b) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut edges = Vec::with_capacity(m);
    let mut graph = vec![Vec::new(); n + 2];

    for i in 0..m {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>() + 1);

        edges.push(Edge::new(l, r));
        graph[l].push((r, i));
        graph[r].push((l, i));
    }

    let dist = process_bfs_parity(&graph, 1, n + 1);
    let dist_even = dist[n + 1][0];
    let dist_odd = dist[n + 1][1];

    let (cycle_even, cycle_odd) = process_shortest_cycle_parity(&graph, &edges, n + 1);
    let mut ret = INF as i64;

    if b >= a {
        let dist_min = dist_even.min(dist_odd) as i64;

        if dist_min != INF as i64 {
            ret = ret.min(a * dist_min);
        }

        let cycle_min = cycle_even.min(cycle_odd) as i64;

        if cycle_min != INF as i64 {
            ret = ret.min(a * cycle_min + b - a);
        }
    } else {
        let penalty = a - b;

        if dist_even != INF {
            ret = ret.min(b * dist_even as i64);
        }

        if dist_odd != INF {
            ret = ret.min(b * dist_odd as i64 + penalty);
        }

        if cycle_odd != INF {
            ret = ret.min(b * cycle_odd as i64);
        }

        if cycle_even != INF {
            ret = ret.min(b * cycle_even as i64 + penalty);
        }
    }

    if ret == INF as i64 {
        writeln!(out, "-1").unwrap();
    } else {
        writeln!(out, "{ret}").unwrap();
    }
}

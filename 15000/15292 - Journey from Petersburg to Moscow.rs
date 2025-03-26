use io::Write;
use std::{collections::BinaryHeap, io, str};

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

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Vec<i64> {
    let mut ret = vec![i64::MAX / 4; graph.len()];
    ret[from] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, from));

    while !queue.is_empty() {
        let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
        cost_curr *= -1;

        if ret[vertex_curr] < cost_curr {
            continue;
        }

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;

            cost_next += cost_curr;

            if ret[vertex_next] > cost_next {
                ret[vertex_next] = cost_next;
                queue.push((-cost_next, vertex_next));
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut roads = Vec::with_capacity(m);

    for _ in 0..m {
        let (u, v, w) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        roads.push((u - 1, v - 1, w));
    }

    let mut candidates = roads.iter().map(|&(_, _, w)| w).collect::<Vec<i64>>();
    candidates.sort_unstable();
    candidates.dedup();

    let mut ret = i64::MAX / 4;

    for candidate in candidates {
        let mut graph = vec![Vec::new(); n];

        for &(u, v, w) in roads.iter() {
            let weight = (w - candidate).max(0);
            graph[u].push((v, weight));
            graph[v].push((u, weight));
        }

        let dist = process_dijkstra(&graph, 0);

        if dist[n - 1] == i64::MAX / 4 {
            continue;
        }

        let val = dist[n - 1] + candidate * k as i64;
        ret = ret.min(val);
    }

    let mut graph = vec![Vec::new(); n];

    for &(u, v, w) in roads.iter() {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let dist = process_dijkstra(&graph, 0);

    ret = ret.min(dist[n - 1]);

    writeln!(out, "{ret}").unwrap();
}

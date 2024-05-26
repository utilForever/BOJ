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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (u, v, w) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let (x, z) = (scan.token::<usize>(), scan.token::<usize>());
    let dist = process_dijkstra(&graph, x);

    let p = scan.token::<usize>();
    let mut vertices = vec![0; p];
    let mut dists = vec![Vec::new(); p];

    for i in 0..p {
        let y = scan.token::<usize>();
        vertices[i] = y;
        dists[i] = process_dijkstra(&graph, y);
    }

    let mut ret = i64::MAX;

    for i in 0..p {
        for j in 0..p {
            if i == j {
                continue;
            }

            for k in 0..p {
                if i == k || j == k {
                    continue;
                }

                if dist[vertices[i]] == i64::MAX / 4
                    || dists[i][vertices[j]] == i64::MAX / 4
                    || dists[j][vertices[k]] == i64::MAX / 4
                    || dists[k][z] == i64::MAX / 4
                {
                    continue;
                }

                ret = ret.min(
                    dist[vertices[i]] + dists[i][vertices[j]] + dists[j][vertices[k]] + dists[k][z],
                );
            }
        }
    }

    writeln!(out, "{}", if ret == i64::MAX { -1 } else { ret }).unwrap();
}

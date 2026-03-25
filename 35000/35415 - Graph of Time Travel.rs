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

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Vec<i64> {
    let mut ret = vec![INF; graph.len()];
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

const INF: i64 = i64::MAX / 4;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k, t) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (u, v, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        graph[u].push((v, c));
        graph[v].push((u, c));
    }

    let dist1 = process_dijkstra(&graph, 1);
    let dist2 = process_dijkstra(&graph, k);
    let dist3 = process_dijkstra(&graph, n);
    let mut ret = INF;

    if dist1[k] < INF && dist2[n] < INF {
        ret = ret.min(dist1[k] + dist2[n]);
    }

    for i in 1..=n {
        if dist2[i] <= t && dist1[i] < INF && dist3[i] < INF {
            ret = ret.min(dist1[i] + dist3[i] + t);
        }
    }

    writeln!(out, "{}", if ret < INF { ret } else { -1 }).unwrap();
}

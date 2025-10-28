use io::Write;
use std::{cmp::Reverse, collections::BinaryHeap, io, str};

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

const INF: i64 = i64::MAX / 4;

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize, k: usize) -> Vec<i64> {
    let mut dist = vec![INF; graph.len()];
    let mut heap_max = vec![BinaryHeap::new(); graph.len()];
    let mut heap_min = BinaryHeap::new();

    heap_max[from].push(0);
    heap_min.push((Reverse(0), from));

    while let Some((Reverse(cost_curr), vertex_curr)) = heap_min.pop() {
        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;
            cost_next += cost_curr;

            if heap_max[vertex_next].len() < k {
                heap_max[vertex_next].push(cost_next);
                heap_min.push((Reverse(cost_next), vertex_next));
            } else if *heap_max[vertex_next].peek().unwrap() > cost_next {
                heap_max[vertex_next].pop();
                heap_max[vertex_next].push(cost_next);
                heap_min.push((Reverse(cost_next), vertex_next));
            }
        }
    }

    for i in 1..graph.len() {
        if heap_max[i].len() == k {
            dist[i] = *heap_max[i].peek().unwrap();
        }
    }

    dist
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
    let mut edges = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        edges[a].push((b, c));
    }

    let ret = process_dijkstra(&edges, 1, k);

    for i in 1..=n {
        writeln!(out, "{}", if ret[i] == INF { -1 } else { ret[i] }).unwrap();
    }
}

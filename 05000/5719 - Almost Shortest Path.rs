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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Vec<i64> {
    let mut ret = vec![i64::MAX / 4; graph.len()];
    ret[from] = 0;

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, from)));

    while !queue.is_empty() {
        let Reverse((cost_curr, vertex_curr)) = queue.pop().unwrap();

        for &(vertex_next, cost_next) in graph[vertex_curr].iter() {
            if ret[vertex_next] > cost_next + cost_curr {
                ret[vertex_next] = cost_next + cost_curr;
                queue.push(Reverse((cost_next + cost_curr, vertex_next)));
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

        if n == 0 && m == 0 {
            break;
        }

        let (s, d) = (scan.token::<usize>(), scan.token::<usize>());
        let mut graph = vec![Vec::new(); n];
        let mut graph_rev = vec![Vec::new(); n];

        for _ in 0..m {
            let (u, v, p) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            graph[u].push((v, p));
            graph_rev[v].push((u, p));
        }

        // Need the backward graph because we should remove the shortest path from the graph to find the second shortest path
        // NOTE: We can't remove the shortest path only from the forward graph
        //       because there is a possibility that removed path can be candidate for the part of the second shortest path
        let forward = process_dijkstra(&graph, s);
        let backward = process_dijkstra(&graph_rev, d);

        // Perform dijkstra's algorithm to find the second shortest path
        let mut queue = BinaryHeap::new();
        let mut ret = vec![i64::MAX / 4; n];

        queue.push(Reverse(s));
        ret[s] = 0;

        while !queue.is_empty() {
            let Reverse(vertex_curr) = queue.pop().unwrap();

            for &(vertex_next, cost_next) in graph[vertex_curr].iter() {
                // If the path is the shortest path, we should skip it
                if forward[vertex_curr] + cost_next + backward[vertex_next] == forward[d] {
                    continue;
                }

                // If the path is the second shortest path, we should update the cost
                if ret[vertex_next] > ret[vertex_curr] + cost_next {
                    ret[vertex_next] = ret[vertex_curr] + cost_next;
                    queue.push(Reverse(vertex_next));
                }
            }
        }

        writeln!(out, "{}", if ret[d] == i64::MAX / 4 { -1 } else { ret[d] }).unwrap();
    }
}

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

fn process_dijkstra(
    graph: &Vec<Vec<(usize, i64, i64)>>,
    from: usize,
    cost_limit: i64,
    time_limit: i64,
) -> Vec<Vec<i64>> {
    let mut ret = vec![vec![i64::MAX; time_limit as usize + 1]; graph.len()];
    ret[from][0] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, 0, from));

    while !queue.is_empty() {
        let (mut cost_curr, time_curr, vertex_curr) = queue.pop().unwrap();
        cost_curr *= -1;

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut time_next, mut cost_next) = *info;
            cost_next += cost_curr;
            time_next += time_curr;

            if time_next > time_limit || cost_next > cost_limit {
                continue;
            }

            if ret[vertex_next][time_next as usize] > cost_next {
                ret[vertex_next][time_next as usize] = cost_next;
                queue.push((-cost_next, time_next, vertex_next));
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, t, m, l) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..l {
        let (u, v, time, cost) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        graph[u].push((v, time, cost));
        graph[v].push((u, time, cost));
    }

    let dists = process_dijkstra(&graph, 1, m, t);
    let mut ret = i64::MAX;

    for i in 0..=t as usize {
        ret = ret.min(dists[n][i]);
    }

    writeln!(out, "{}", if ret == i64::MAX { -1 } else { ret }).unwrap();
}

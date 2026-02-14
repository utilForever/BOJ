use io::Write;
use std::{
    collections::{BinaryHeap, VecDeque},
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
}

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Vec<i64> {
    let mut ret = vec![i64::MAX; graph.len()];
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

    let (n, s, t) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let m_stefan = scan.token::<usize>();
    let mut graph_stefan = vec![Vec::new(); n + 1];
    let mut edges_stefan = vec![(0, 0, 0); m_stefan];

    for _ in 0..m_stefan {
        let (a, b, l) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        graph_stefan[a].push((b, l));
        graph_stefan[b].push((a, l));
        edges_stefan.push((a, b, l));
    }

    let m_konstantin = scan.token::<usize>();
    let mut graph_konstantin = vec![Vec::new(); n + 1];
    let mut edges_konstantin = vec![(0, 0, 0); m_konstantin];

    for _ in 0..m_konstantin {
        let (a, b, l) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        graph_konstantin[a].push((b, l));
        graph_konstantin[b].push((a, l));
        edges_konstantin.push((a, b, l));
    }

    let dist_stefan = process_dijkstra(&graph_stefan, t);
    let dist_konstantin = process_dijkstra(&graph_konstantin, t);
    let mut graph = vec![Vec::new(); n * 2];
    let idx = |x: usize, is_night: bool| -> usize {
        if is_night {
            n + x - 1
        } else {
            x - 1
        }
    };

    for (a, b, w) in edges_stefan {
        if dist_stefan[a] > dist_stefan[b] {
            graph[idx(a, false)].push((idx(b, true), w));
        } else if dist_stefan[a] < dist_stefan[b] {
            graph[idx(b, false)].push((idx(a, true), w));
        }
    }

    for (a, b, w) in edges_konstantin {
        if dist_konstantin[a] > dist_konstantin[b] {
            graph[idx(a, true)].push((idx(b, false), w));
        } else if dist_konstantin[a] < dist_konstantin[b] {
            graph[idx(b, true)].push((idx(a, false), w));
        }
    }

    let start = idx(s, false);
    let mut reachable = vec![false; n * 2];
    let mut queue = VecDeque::new();

    reachable[start] = true;
    queue.push_back(start);

    while let Some(u) = queue.pop_front() {
        for &(v, _) in graph[u].iter() {
            if !reachable[v] {
                reachable[v] = true;
                queue.push_back(v);
            }
        }
    }

    let cnt = reachable.iter().filter(|&&x| x).count();
    let mut degree_in = vec![0; n * 2];

    for u in 0..n * 2 {
        if !reachable[u] {
            continue;
        }

        for &(v, _) in graph[u].iter() {
            if reachable[v] {
                degree_in[v] += 1;
            }
        }
    }

    let mut queue_topological = VecDeque::new();

    for i in 0..n * 2 {
        if reachable[i] && degree_in[i] == 0 {
            queue_topological.push_back(i);
        }
    }

    let mut order_topological = Vec::new();

    while let Some(u) = queue_topological.pop_front() {
        order_topological.push(u);

        for &(v, _) in graph[u].iter() {
            if !reachable[v] {
                continue;
            }

            degree_in[v] -= 1;

            if degree_in[v] == 0 {
                queue_topological.push_back(v);
            }
        }
    }

    if order_topological.len() != cnt {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut dp = vec![i64::MIN / 4; n * 2];
    dp[start] = 0;

    for u in order_topological {
        if dp[u] == i64::MIN / 4 {
            continue;
        }

        for &(v, w) in graph[u].iter() {
            if !reachable[v] {
                continue;
            }

            dp[v] = dp[v].max(dp[u] + w);
        }
    }

    let ret = dp[idx(t, false)].max(dp[idx(t, true)]);

    writeln!(out, "{ret}").unwrap();
}

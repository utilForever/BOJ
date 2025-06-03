use io::Write;
use std::{collections::VecDeque, io, str};

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

fn process_bfs(graph: &Vec<Vec<(usize, i64)>>, start: usize) -> (Vec<i64>, Vec<usize>) {
    let n = graph.len() - 1;
    let mut queue = VecDeque::new();
    let mut dist = vec![i64::MAX; n + 1];
    let mut order = Vec::with_capacity(n);

    queue.push_back(start);
    dist[start] = 0;

    while let Some(node) = queue.pop_front() {
        order.push(node);

        let dist_next = dist[node] + 1;

        for &(next, _) in graph[node].iter() {
            if dist[next] == i64::MAX {
                dist[next] = dist_next;
                queue.push_back(next);
            }
        }
    }

    (dist, order)
}

fn process_dp(
    graph: &Vec<Vec<(usize, i64)>>,
    dist: &Vec<i64>,
    order: &Vec<usize>,
) -> (Vec<i64>, Vec<i64>) {
    let n = dist.len() - 1;
    let mut cost_min = vec![i64::MAX; n + 1];
    let mut cost_max = vec![i64::MIN; n + 1];

    cost_min[1] = 0;
    cost_max[1] = 0;

    for &node in order {
        let (node_min, node_max) = (cost_min[node], cost_max[node]);

        if node_min == i64::MAX {
            continue;
        }

        for &(next, cost_next) in graph[node].iter() {
            if dist[next] == dist[node] + 1 {
                cost_min[next] = cost_min[next].min(node_min + cost_next);
                cost_max[next] = cost_max[next].max(node_max + cost_next);
            }
        }
    }

    (cost_min, cost_max)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut time_stay = vec![0; n + 1];

    for i in 2..=n {
        time_stay[i] = scan.token::<i64>();
    }

    let mut graph = vec![Vec::new(); n + 1];
    let mut graph_rev = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (v, w, f) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        let cost = f + if w == 1 { 0 } else { time_stay[w] };

        graph[v].push((w, cost));
        graph_rev[w].push((v, cost));
    }

    let (dist1, order1) = process_bfs(&graph, 1);
    let (min1, max1) = process_dp(&graph, &dist1, &order1);

    let (dist2, order2) = process_bfs(&graph_rev, 1);
    let (min2, max2) = process_dp(&graph_rev, &dist2, &order2);

    let mut dist_min = i64::MAX;

    for i in 2..=n {
        if dist1[i] != i64::MAX && dist2[i] != i64::MAX {
            dist_min = dist_min.min(dist1[i] + dist2[i]);
        }
    }

    let mut ret_min = i64::MAX;
    let mut ret_max = i64::MIN;

    for i in 2..=n {
        if dist1[i] + dist2[i] == dist_min {
            ret_min = ret_min.min(min1[i] + min2[i]);
            ret_max = ret_max.max(max1[i] + max2[i]);
        }
    }

    writeln!(out, "{ret_min}").unwrap();
    writeln!(out, "{ret_max}").unwrap();
}

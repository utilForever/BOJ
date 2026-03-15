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

const INF: i64 = i64::MAX / 4;
const DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut costs = vec![vec![INF; m]; n];

    for i in 0..n {
        for j in 0..m {
            costs[i][j] = match scan.token::<i64>() {
                -2 => 0,
                -1 => INF,
                x => x,
            };
        }
    }

    let mut graph = vec![Vec::new(); n * m + 1];
    let idx = |x: usize, y: usize| -> usize { x * m + y };

    for i in 0..n {
        for j in 0..m {
            if costs[i][j] == INF {
                continue;
            }

            let u = idx(i, j);

            for &(dy, dx) in DIRECTIONS.iter() {
                let (y_next, x_next) = (i as i64 + dy, j as i64 + dx);

                if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= m as i64 {
                    continue;
                }

                let (y_next, x_next) = (y_next as usize, x_next as usize);

                if costs[y_next][x_next] == INF {
                    continue;
                }

                let v = idx(y_next, x_next);
                graph[u].push((v, costs[y_next][x_next]));
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if i == 0 || j == m - 1 {
                if costs[i][j] == INF {
                    continue;
                }

                let u = n * m;
                let v = idx(i, j);

                graph[u].push((v, costs[i][j]));
            }
        }
    }

    let dist = process_dijkstra(&graph, n * m);
    let mut ret = INF;

    for i in 0..n {
        for j in 0..m {
            if j == 0 || i == n - 1 {
                ret = ret.min(dist[idx(i, j)]);
            }
        }
    }

    writeln!(out, "{}", if ret >= INF / 2 { -1 } else { ret }).unwrap();
}

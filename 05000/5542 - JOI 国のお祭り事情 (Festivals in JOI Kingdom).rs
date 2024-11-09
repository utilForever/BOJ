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

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, festivals: &Vec<bool>) -> Vec<i64> {
    let mut queue = BinaryHeap::new();
    let mut ret = vec![i64::MAX; graph.len()];

    for (idx, &festival) in festivals.iter().enumerate() {
        if festival {
            queue.push((0, idx));
            ret[idx] = 0;
        }
    }

    while !queue.is_empty() {
        let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
        cost_curr *= -1;

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

fn find(pos: &mut Vec<i64>, idx: usize) -> i64 {
    if pos[idx] < 0 {
        idx as i64
    } else {
        pos[idx] = find(pos, pos[idx] as usize);
        pos[idx]
    }
}

fn merge(pos: &mut Vec<i64>, p: usize, q: usize) {
    let mut idx_p = find(pos, p);
    let mut idx_q = find(pos, q);

    if idx_p != idx_q {
        if pos[idx_p as usize] > pos[idx_q as usize] {
            std::mem::swap(&mut idx_p, &mut idx_q);
        }

        pos[idx_p as usize] += pos[idx_q as usize];
        pos[idx_q as usize] = idx_p;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut graph1 = vec![Vec::new(); n + 1];
    let mut graph2 = vec![Vec::new(); n + 1];
    let mut festivals = vec![false; n + 1];
    let mut queries = vec![(0, 0); q];

    for _ in 0..m {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        graph1[a].push((b, c));
        graph1[b].push((a, c));
        graph2[a].push((b, c));
    }

    for _ in 0..k {
        let x = scan.token::<usize>();
        festivals[x] = true;
    }

    let dist = process_dijkstra(&graph1, &festivals);
    let mut edges = Vec::new();

    for i in 1..=n {
        for &(j, _) in graph2[i].iter() {
            let dist_min = dist[i].min(dist[j]);
            edges.push((i, j, dist_min));
        }
    }

    edges.sort_by(|a, b| b.2.cmp(&a.2));

    for i in 0..q {
        let (s, e) = (scan.token::<usize>(), scan.token::<usize>());
        queries[i] = (s, e);
    }

    let mut left = vec![0; q];
    let mut right = vec![0; q];

    for i in 0..q {
        left[i] = 0;
        right[i] = m - 1;
    }

    let mut queries_mid = vec![Vec::new(); m];

    loop {
        for i in 0..m {
            queries_mid[i].clear();
        }

        let mut should_check = false;

        for i in 0..q {
            if left[i] <= right[i] {
                should_check = true;
                queries_mid[(left[i] + right[i]) / 2].push(i);
            }
        }

        if !should_check {
            break;
        }

        let mut parent = vec![-1; n + 1];

        for i in 0..m {
            let (a, b, _) = edges[i];
            merge(&mut parent, a, b);

            for &idx in queries_mid[i].iter() {
                if find(&mut parent, queries[idx].0) == find(&mut parent, queries[idx].1) {
                    right[idx] = i - 1;
                } else {
                    left[idx] = i + 1;
                }
            }
        }
    }

    for i in 0..q {
        writeln!(out, "{}", edges[left[i]].2).unwrap();
    }
}

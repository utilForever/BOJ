use io::Write;
use std::{cmp, collections::BinaryHeap, io, str};

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

fn process_dijkstra(edge: &Vec<Vec<usize>>, len: &Vec<Vec<usize>>, start: usize) -> Vec<i64> {
    let mut dist = vec![std::i64::MAX; edge.len() + 1];
    let mut priority_queue = BinaryHeap::new();

    priority_queue.push((0, start));
    dist[start] = 0;

    while !priority_queue.is_empty() {
        let (mut d, x) = priority_queue.pop().unwrap();
        d *= -1;

        if dist[x] < d {
            continue;
        }

        for i in 0..edge[x].len() {
            let next_len = len[x][i] as i64 + d;

            if next_len < dist[edge[x][i]] {
                dist[edge[x][i]] = next_len;
                priority_queue.push((-next_len, edge[x][i]));
            }
        }
    }

    dist
}

fn calculate_min_dist(
    accumulated_dist: &Vec<i64>,
    min_dist: &mut Vec<Vec<i64>>,
    idx: &mut Vec<Vec<i64>>,
    t: usize,
    start: usize,
    end: usize,
    left: usize,
    right: usize,
) {
    if start > end {
        return;
    }

    let mid = (start + end) / 2;
    min_dist[t][mid] = -1;
    idx[t][mid] = -1;

    for k in left..=cmp::min(mid - 1, right) {
        let dist = min_dist[t - 1][k]
            + (accumulated_dist[mid] - accumulated_dist[k]) * (mid - k - 1) as i64;

        if min_dist[t][mid] == -1 || min_dist[t][mid] > dist {
            min_dist[t][mid] = dist;
            idx[t][mid] = k as i64;
        }
    }

    calculate_min_dist(
        accumulated_dist,
        min_dist,
        idx,
        t,
        start,
        mid - 1,
        left,
        idx[t][mid] as usize,
    );
    calculate_min_dist(
        accumulated_dist,
        min_dist,
        idx,
        t,
        mid + 1,
        end,
        idx[t][mid] as usize,
        right,
    );
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, b, s, r) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut front_edge = vec![Vec::new(); n + 1];
    let mut back_edge = vec![Vec::new(); n + 1];
    let mut front_len = vec![Vec::new(); n + 1];
    let mut back_len = vec![Vec::new(); n + 1];

    for _ in 0..r {
        let (u, v, l) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        front_edge[u].push(v);
        front_len[u].push(l);
        back_edge[v].push(u);
        back_len[v].push(l);
    }

    let front_dist = process_dijkstra(&front_edge, &front_len, b + 1);
    let back_dist = process_dijkstra(&back_edge, &back_len, b + 1);

    let mut dist = vec![0; b];
    for i in 0..b {
        dist[i] = front_dist[i + 1] + back_dist[i + 1];
    }

    dist.sort();
    dist.insert(0, 0);

    let mut accumulated_dist = vec![0; b + 1];
    for i in 1..=b {
        accumulated_dist[i] = accumulated_dist[i - 1] + dist[i];
    }

    let mut min_dist = vec![vec![0; b + 1]; b + 1];
    let mut idx = vec![vec![0; b + 1]; b + 1];
    for i in 1..=b {
        min_dist[1][i] = (i - 1) as i64 * accumulated_dist[i];
        idx[1][i] = 1;
    }

    for i in 2..=s {
        calculate_min_dist(&accumulated_dist, &mut min_dist, &mut idx, i, i, b, 0, b);
    }

    writeln!(out, "{}", min_dist[s][b]).unwrap();
}

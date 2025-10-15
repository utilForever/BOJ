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

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Vec<i64> {
    let mut ret = vec![i64::MAX / 4; graph.len()];
    ret[from] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), from));

    while let Some((Reverse(cost_curr), vertex_curr)) = queue.pop() {
        if ret[vertex_curr] != cost_curr {
            continue;
        }

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;
            cost_next += cost_curr;

            if ret[vertex_next] > cost_next {
                ret[vertex_next] = cost_next;
                queue.push((Reverse(cost_next), vertex_next));
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());

    let mut villages = vec![(0, 0, 0); n];

    for i in 0..n {
        villages[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    let mut graph = vec![Vec::new(); n + k];

    if n >= 2 {
        let mut idxes = (0..n).collect::<Vec<_>>();
        idxes.sort_unstable_by_key(|&i| villages[i].0);

        for i in 0..n - 1 {
            let u = idxes[i];
            let v = idxes[i + 1];
            let dist = (villages[u].0 - villages[v].0)
                .abs()
                .min((villages[u].1 - villages[v].1).abs());

            graph[u].push((v, dist));
            graph[v].push((u, dist));
        }

        let mut idxes = (0..n).collect::<Vec<_>>();
        idxes.sort_unstable_by_key(|&i| villages[i].1);

        for i in 0..n - 1 {
            let u = idxes[i];
            let v = idxes[i + 1];
            let dist = (villages[u].0 - villages[v].0)
                .abs()
                .min((villages[u].1 - villages[v].1).abs());

            graph[u].push((v, dist));
            graph[v].push((u, dist));
        }
    }

    for i in 0..n {
        let x = villages[i].2 as usize % k;
        let y = (k - x) % k;

        graph[i].push((n + x, villages[i].2));
        graph[n + y].push((i, villages[i].2));
    }

    let dist = process_dijkstra(&graph, 0);

    for i in 0..n {
        writeln!(out, "{}", dist[i]).unwrap();
    }
}

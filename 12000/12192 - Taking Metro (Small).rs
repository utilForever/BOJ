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
    let mut ret = vec![i64::MAX / 4; graph.len()];
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

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut stations = vec![0; n];
        let mut times_wait = vec![0; n];
        let mut stations_first = vec![0; n];

        let mut sum_stations = 0;
        let mut edges_station: Vec<(usize, usize, i64)> = Vec::new();

        for j in 0..n {
            let (sn, w) = (scan.token::<usize>(), scan.token::<i64>());
            stations[j] = sn;
            times_wait[j] = w;
            stations_first[j] = sum_stations;

            for k in 0..sn - 1 {
                let time = scan.token::<i64>();
                let u = sum_stations + k;
                let v = sum_stations + k + 1;

                edges_station.push((u, v, time));
                edges_station.push((v, u, time));
            }

            sum_stations += sn;
        }

        let m = scan.token::<usize>();
        let mut edges_tunnel: Vec<(usize, usize, i64)> = vec![(0, 0, 0); m];

        for j in 0..m {
            let (m1, s1, m2, s2, t) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            let u = stations_first[m1 - 1] + (s1 - 1);
            let v = stations_first[m2 - 1] + (s2 - 1);

            edges_tunnel[j] = (u, v, t);
        }

        let mut graph: Vec<Vec<(usize, i64)>> = vec![Vec::new(); 2 * sum_stations];

        for j in 0..n {
            for k in 0..stations[j] {
                let station = stations_first[j] + k;
                let u = station * 2;
                let v = station * 2 + 1;

                graph[u].push((v, times_wait[j]));
                graph[v].push((u, 0));
            }
        }

        for &(u, v, len) in edges_station.iter() {
            graph[u * 2 + 1].push((v * 2 + 1, len));
        }

        for &(u, v, len) in edges_tunnel.iter() {
            graph[u * 2].push((v * 2, len));
            graph[v * 2].push((u * 2, len));
        }

        writeln!(out, "Case #{i}:").unwrap();

        let q = scan.token::<i64>();

        for _ in 0..q {
            let (x1, y1, x2, y2) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
            );

            let station_start = stations_first[x1 - 1] + (y1 - 1);
            let station_end = stations_first[x2 - 1] + (y2 - 1);
            let from = station_start * 2;
            let to = station_end * 2;
            let dist = process_dijkstra(&graph, from);

            if dist[to] >= i64::MAX / 4 {
                writeln!(out, "-1").unwrap();
            } else {
                writeln!(out, "{}", dist[to]).unwrap();
            }
        }
    }
}

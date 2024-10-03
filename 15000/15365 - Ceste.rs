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
}

fn process_dijkstra(graph: &Vec<Vec<(i16, i64, i64)>>, ret: &mut Vec<i64>) {
    let mut queue: BinaryHeap<Reverse<(i64, i64, i16)>> = BinaryHeap::new();
    let mut costs = vec![i64::MAX; graph.len()];

    queue.push(Reverse((0, 0, 1)));
    ret[1] = 0;

    while !queue.is_empty() {
        let (t, c, curr) = queue.pop().unwrap().0;
        ret[curr as usize] = ret[curr as usize].min(c * t);

        if costs[curr as usize] < c {
            continue;
        }

        costs[curr as usize] = c;

        for (next, time, cost) in graph[curr as usize].iter() {
            let next_cost = c + cost;
            let next_time = t + time;
            queue.push(Reverse((next_time, next_cost, *next)));
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (a, b, t, c) = (
            scan.token::<i16>(),
            scan.token::<i16>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        graph[a as usize].push((b, t, c));
        graph[b as usize].push((a, t, c));
    }

    let mut ret = vec![i64::MAX; n + 1];

    process_dijkstra(&graph, &mut ret);

    for i in 2..=n {
        writeln!(out, "{}", if ret[i] == i64::MAX { -1 } else { ret[i] }).unwrap();
    }
}

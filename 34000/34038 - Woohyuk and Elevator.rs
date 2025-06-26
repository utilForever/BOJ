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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, e, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let (t1, t2) = (scan.token::<i64>(), scan.token::<i64>());
    let mut cost_min = vec![vec![i64::MAX / 4; n + 1]; n + 1];

    for _ in 0..m {
        let (c, t) = (scan.token::<usize>(), scan.token::<i64>());
        let mut floors = vec![0; c];

        for i in 0..c {
            floors[i] = scan.token::<i64>();
        }

        floors.sort_unstable();

        floors.windows(2).for_each(|w| {
            let cost = (w[1] - w[0]) * t;

            if cost < cost_min[w[0] as usize][w[1] as usize] {
                cost_min[w[0] as usize][w[1] as usize] = cost;
                cost_min[w[1] as usize][w[0] as usize] = cost;
            }
        });
    }

    let mut graph = vec![Vec::new(); n + 1];

    for i in 1..=n {
        for j in 1..=n {
            if cost_min[i][j] < i64::MAX / 4 {
                graph[i].push((j, cost_min[i][j]));
            }
        }
    }

    let mut dist = vec![vec![i64::MAX / 4; k + 1]; n + 1];
    let mut priority_queue = BinaryHeap::new();

    dist[1][0] = 0;
    priority_queue.push((Reverse(0), 1, 0));

    while let Some((Reverse(cost_curr), floor, k_curr)) = priority_queue.pop() {
        if cost_curr != dist[floor][k_curr] {
            continue;
        }

        if floor == e {
            writeln!(out, "{cost_curr}").unwrap();
            return;
        }

        if k_curr < k && floor < n {
            let cost_next = cost_curr + t1 + k_curr as i64 * t2;

            if cost_next < dist[floor + 1][k_curr + 1] {
                dist[floor + 1][k_curr + 1] = cost_next;
                priority_queue.push((Reverse(cost_next), floor + 1, k_curr + 1));
            }
        }

        if k_curr < k && floor > 1 {
            let cost_next = cost_curr + t1 + k_curr as i64 * t2;

            if cost_next < dist[floor - 1][k_curr + 1] {
                dist[floor - 1][k_curr + 1] = cost_next;
                priority_queue.push((Reverse(cost_next), floor - 1, k_curr + 1));
            }
        }

        for &(floor_next, cost_walk) in graph[floor].iter() {
            let cost_next = cost_curr + cost_walk;

            if cost_next < dist[floor_next as usize][k_curr] {
                dist[floor_next as usize][k_curr] = cost_next;
                priority_queue.push((Reverse(cost_next), floor_next as usize, k_curr));
            }
        }
    }

    writeln!(out, "-1").unwrap();
}

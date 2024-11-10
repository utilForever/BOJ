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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let x = scan.token::<usize>();
    let mut exits = vec![0; x];

    for i in 0..x {
        exits[i] = scan.token::<usize>();
    }

    let mut dist = vec![i64::MAX; n + 1];
    let mut queue = BinaryHeap::new();

    dist[1] = 0;
    queue.push((0, 1));

    while !queue.is_empty() {
        let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
        cost_curr *= -1;

        if dist[vertex_curr] < cost_curr {
            continue;
        }

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;
            cost_next += cost_curr;

            if dist[vertex_next] > cost_next {
                dist[vertex_next] = cost_next;
                queue.push((-cost_next, vertex_next));
            }
        }
    }

    let mut ret = i64::MAX;

    for i in 0..x {
        if dist[exits[i]] == i64::MAX {
            continue;
        }

        let time_arrive = dist[exits[i]];
        let period = time_arrive / k;
        let idx_exit = period as usize % x;

        let time_escape: i64 = if idx_exit == i {
            time_arrive
        } else {
            let time_shift = ((i + x) as i64 - ((period + 1) % x as i64)) % x as i64;
            let period_next = period + 1 + time_shift;
            let time_cand = k * period_next;

            if time_arrive > time_cand {
                k * (period_next + x as i64)
            } else {
                time_cand
            }
        };

        if time_escape >= time_arrive && time_escape < ret {
            ret = time_escape;
        }
    }

    writeln!(out, "{ret}").unwrap();
}

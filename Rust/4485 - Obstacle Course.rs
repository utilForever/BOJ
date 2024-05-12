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

    let mut idx = 1;

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut graph = vec![Vec::new(); n * n + 1];
        let mut cost_arrive = 0;

        for i in 1..=n {
            for j in 1..=n {
                let cost = scan.token::<i64>();
                let offset = (i - 1) * n + j;

                if i == n && j == n {
                    cost_arrive = cost;
                }

                if i > 1 {
                    graph[offset].push((offset - n, cost));
                }

                if i < n {
                    graph[offset].push((offset + n, cost));
                }

                if j > 1 {
                    graph[offset].push((offset - 1, cost));
                }

                if j < n {
                    graph[offset].push((offset + 1, cost));
                }
            }
        }

        let ret = process_dijkstra(&graph, 1);

        writeln!(out, "Problem {idx}: {}", ret[n * n] + cost_arrive).unwrap();

        idx += 1;
    }
}

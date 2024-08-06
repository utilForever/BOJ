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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
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

    let mut dists = vec![vec![i64::MAX / 4; n + 1]; n + 1];
    let mut visited = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        dists[i][i] = 0;

        let mut queue = BinaryHeap::new();
        queue.push((0, i));

        while !queue.is_empty() {
            let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
            cost_curr *= -1;

            if dists[i][vertex_curr] < cost_curr {
                continue;
            }

            for info in graph[vertex_curr].iter() {
                let (vertex_next, mut cost_next) = *info;

                cost_next += cost_curr;

                if dists[i][vertex_next] > cost_next {
                    dists[i][vertex_next] = cost_next;
                    visited[vertex_next][i] = vertex_curr;
                    queue.push((-cost_next, vertex_next));
                }
            }
        }
    }

    for i in 1..=n {
        for j in 1..=n {
            if i == j {
                write!(out, "- ").unwrap();
            } else {
                write!(out, "{} ", visited[i][j]).unwrap();
            }
        }

        writeln!(out).unwrap();
    }
}

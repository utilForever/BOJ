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

    let (n, m, p) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut graph = vec![Vec::new(); n];

    for _ in 0..m {
        let (a, b, c, tag) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<char>(),
        );
        graph[a].push((b, c, tag));
        graph[b].push((a, c, tag));
    }

    for _ in 0..p {
        let (s, e) = (scan.token::<usize>(), scan.token::<usize>());
        let mut ret = vec![(i64::MAX / 4, i64::MAX / 4); graph.len()];
        ret[s] = (0, 0);

        let mut queue = BinaryHeap::new();
        queue.push((0, 0, s));

        while !queue.is_empty() {
            let (mut cost_curr_out, mut cost_curr_total, vertex_curr) = queue.pop().unwrap();
            cost_curr_out *= -1;
            cost_curr_total *= -1;

            for info in graph[vertex_curr].iter() {
                let (vertex_next, cost_next, tag) = *info;

                let cost_next_out = cost_curr_out + if tag == 'O' { cost_next } else { 0 };
                let cost_next_total = cost_curr_total + cost_next;

                if ret[vertex_next].0 > cost_next_out
                    || (ret[vertex_next].0 == cost_next_out && ret[vertex_next].1 > cost_next_total)
                {
                    ret[vertex_next] = (cost_next_out, cost_next_total);
                    queue.push((-cost_next_out, -cost_next_total, vertex_next));
                }
            }
        }

        if ret[e].1 == i64::MAX / 4 {
            writeln!(out, "IMPOSSIBLE").unwrap();
            continue;
        }

        writeln!(out, "{} {}", ret[e].0, ret[e].1).unwrap();
    }
}

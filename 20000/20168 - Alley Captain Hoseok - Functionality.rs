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

    let (n, m, a, b, c) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (s, e, p) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        graph[s].push((e, p));
        graph[e].push((s, p));
    }

    let mut ret = vec![i64::MAX / 4; graph.len()];
    ret[a] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, 0, a));

    while !queue.is_empty() {
        let (mut cost_curr, shame_max, vertex_curr) = queue.pop().unwrap();
        cost_curr *= -1;

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;

            cost_next += cost_curr;

            if cost_next > c {
                continue;
            }

            let shame_next = shame_max.max(cost_next - cost_curr);

            if ret[vertex_next] > shame_next {
                ret[vertex_next] = shame_next;
                queue.push((-cost_next, shame_next, vertex_next));
            }
        }
    }

    writeln!(out, "{}", if ret[b] == i64::MAX / 4 { -1 } else { ret[b] }).unwrap();
}

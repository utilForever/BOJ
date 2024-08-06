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

    let (m, n) = (scan.token::<usize>(), scan.token::<i64>());
    let mut graph = vec![Vec::new(); m];

    for _ in 0..n {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        graph[a].push((b, c));
    }

    let mut ret = vec![(i64::MAX / 4, i64::MAX / 4); graph.len()];
    ret[0] = (0, 0);

    let mut queue = BinaryHeap::new();
    queue.push((0, 0, 0));

    while !queue.is_empty() {
        let (mut cost_curr, mut length, vertex_curr) = queue.pop().unwrap();
        cost_curr *= -1;
        length += 1;

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;

            cost_next += cost_curr;

            if ret[vertex_next].0 > length
                || (ret[vertex_next].0 == length && ret[vertex_next].1 > cost_next)
            {
                ret[vertex_next] = (length, cost_next);
                queue.push((-cost_next, length, vertex_next));
            }
        }
    }

    writeln!(out, "{}", ret[1].1).unwrap();
}

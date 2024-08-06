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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let (s, t) = (scan.token::<usize>(), scan.token::<usize>());
    let mut vertices = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (u, v, w) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        vertices[u].push((v, w));
    }

    let mut dist = vec![vec![i64::MAX; 50]; n + 1];
    let mut queue: BinaryHeap<Reverse<(i64, usize, i64)>> = BinaryHeap::new();

    dist[s][0] = 0;
    queue.push(Reverse((0, s, 0)));

    while !queue.is_empty() {
        let Reverse((cost, vertex, fee)) = queue.pop().unwrap();

        if cost > dist[vertex][fee as usize] {
            continue;
        }

        for info in vertices[vertex].iter() {
            let (next_vertex, next_cost) = *info;
            let next_fee = (fee + next_cost) % k;

            if dist[next_vertex][next_fee as usize] > cost + next_cost {
                dist[next_vertex][next_fee as usize] = cost + next_cost;
                queue.push(Reverse((cost + next_cost, next_vertex, next_fee)));
            }
        }
    }

    if dist[t][0] == i64::MAX {
        writeln!(out, "IMPOSSIBLE").unwrap();
    } else {
        writeln!(out, "{}", dist[t][0]).unwrap();
    }
}

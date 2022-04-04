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

fn process_dijkstra(
    vertices: &mut Vec<i64>,
    vertex_info: &Vec<Vec<(usize, i64)>>,
    from: usize,
    to: usize,
) -> i64 {
    vertices.fill(1_000_000_000);
    vertices[from] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, from));

    while !queue.is_empty() {
        let (mut cost, vertex) = queue.pop().unwrap();
        cost *= -1;

        for info in vertex_info[vertex].iter() {
            let (next_vertex, mut next_cost) = *info;
            next_cost += cost;

            if vertices[next_vertex] > next_cost {
                vertices[next_vertex] = next_cost;
                queue.push((-next_cost, next_vertex));
            }
        }
    }

    return vertices[to];
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, e) = (scan.token::<usize>(), scan.token::<usize>());
    let mut vertices = vec![1_000_000_000; n + 1];
    let mut vertex_info = vec![Vec::new(); n + 1];

    for _ in 0..e {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        vertex_info[a].push((b, c));
        vertex_info[b].push((a, c));
    }

    let (v1, v2) = (scan.token::<usize>(), scan.token::<usize>());

    let cost1 = process_dijkstra(&mut vertices, &vertex_info, 1, v1)
        + process_dijkstra(&mut vertices, &vertex_info, v1, v2)
        + process_dijkstra(&mut vertices, &vertex_info, v2, n);
    let cost2 = process_dijkstra(&mut vertices, &vertex_info, 1, v2)
        + process_dijkstra(&mut vertices, &vertex_info, v2, v1)
        + process_dijkstra(&mut vertices, &vertex_info, v1, n);

    if cost1 <= cost2 && cost1 < 1_000_000_000 {
        writeln!(out, "{}", cost1).unwrap();
    } else if cost1 >= cost2 && cost2 < 1_000_000_000 {
        writeln!(out, "{}", cost2).unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}

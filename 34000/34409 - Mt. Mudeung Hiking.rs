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

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Vec<i64> {
    let mut ret = vec![i64::MAX; graph.len()];
    ret[from] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, from));

    while !queue.is_empty() {
        let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
        cost_curr *= -1;

        if cost_curr > ret[vertex_curr] {
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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let (x, y) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
    let (a, b, c) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut heights = vec![vec![0; m]; n];

    let mut max_height = 0;
    let mut max_idx = (0, 0);

    for i in 0..n {
        for j in 0..m {
            heights[i][j] = scan.token::<i64>();

            if heights[i][j] > max_height {
                max_height = heights[i][j];
                max_idx = (i, j);
            }
        }
    }

    let mut graph = vec![Vec::with_capacity(4); n * m];
    let dy = [-1, 1, 0, 0];
    let dx = [0, 0, -1, 1];

    for i in 0..n {
        for j in 0..m {
            for k in 0..4 {
                let y_next = i as i64 + dy[k];
                let x_next = j as i64 + dx[k];

                if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= m as i64 {
                    continue;
                }

                let y_next = y_next as usize;
                let x_next = x_next as usize;
                let diff = (heights[y_next][x_next] - heights[i][j]).abs();

                if diff > c {
                    continue;
                }

                let cost = if diff == 0 {
                    1
                } else if heights[y_next][x_next] > heights[i][j] {
                    a * diff
                } else {
                    b * diff
                };

                graph[i * m + j].push((y_next * m + x_next, cost));
            }
        }
    }

    let dist = process_dijkstra(&graph, x * m + y);
    let ret = dist[max_idx.0 * m + max_idx.1];

    if ret == i64::MAX {
        writeln!(out, "-1").unwrap();
    } else {
        writeln!(out, "{ret}").unwrap();
    }
}

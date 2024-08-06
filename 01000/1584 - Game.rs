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

    let n = scan.token::<i64>();
    let mut maze = vec![vec![0; 501]; 501];

    for _ in 0..n {
        let (x1, y1, x2, y2) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        for i in y1.min(y2)..=y1.max(y2) {
            for j in x1.min(x2)..=x1.max(x2) {
                if i == 0 && j == 0 {
                    continue;
                }

                maze[i][j] = 1;
            }
        }
    }

    let m = scan.token::<i64>();

    for _ in 0..m {
        let (x1, y1, x2, y2) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        for i in y1.min(y2)..=y1.max(y2) {
            for j in x1.min(x2)..=x1.max(x2) {
                if i == 0 && j == 0 {
                    continue;
                }

                maze[i][j] = 2;
            }
        }
    }

    let mut graph = vec![Vec::new(); 501 * 501];

    for i in 0..=500 {
        for j in 0..=500 {
            if maze[i][j] == 2 {
                continue;
            }

            let pos = i * 501 + j;

            if i > 0 && maze[i - 1][j] != 2 {
                graph[pos].push(((i - 1) * 501 + j, if maze[i - 1][j] == 1 { 1 } else { 0 }));
            }

            if i < 500 && maze[i + 1][j] != 2 {
                graph[pos].push(((i + 1) * 501 + j, if maze[i + 1][j] == 1 { 1 } else { 0 }));
            }

            if j > 0 && maze[i][j - 1] != 2 {
                graph[pos].push((i * 501 + j - 1, if maze[i][j - 1] == 1 { 1 } else { 0 }));
            }

            if j < 500 && maze[i][j + 1] != 2 {
                graph[pos].push((i * 501 + j + 1, if maze[i][j + 1] == 1 { 1 } else { 0 }));
            }
        }
    }

    let ret = process_dijkstra(&graph, 0);
    let val = ret[501 * 501 - 1];

    writeln!(out, "{}", if val == i64::MAX / 4 { -1 } else { val }).unwrap();
}

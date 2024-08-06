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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut codes = vec![0; n + 1];
    let mut graph = vec![Vec::new(); n + 1];

    for i in 1..=n {
        codes[i] = scan.token::<i64>();
    }

    let code_max = *codes.iter().max().unwrap() as usize * 2;
    let mut is_prime = vec![true; code_max + 1];
    is_prime[1] = false;

    let mut i = 2;

    while i * i <= code_max {
        if !is_prime[i] {
            i += 1;
            continue;
        }

        for j in (i * i..=code_max).step_by(i) {
            is_prime[j] = false;
        }

        i += 1;
    }

    for _ in 0..m {
        let (u, v, w) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        if !is_prime[(codes[u] + codes[v]) as usize] {
            continue;
        }

        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let ret = process_dijkstra(&graph, 1);

    if ret[n] == i64::MAX / 4 {
        writeln!(out, "Now where are you?").unwrap();
    } else {
        writeln!(out, "{}", ret[n]).unwrap();
    }
}

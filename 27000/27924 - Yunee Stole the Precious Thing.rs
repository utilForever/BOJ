use io::Write;
use std::{io, str};

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

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    dist: &mut Vec<usize>,
    node_cur: usize,
    node_prev: usize,
    d: usize,
) {
    dist[node_cur] = d;

    for &vertex in &graph[node_cur] {
        if vertex == node_prev {
            continue;
        }

        process_dfs(graph, dist, vertex, node_cur, d + 1);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    let (a, b, c) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut dist_a = vec![0; n + 1];
    let mut dist_b = vec![0; n + 1];
    let mut dist_c = vec![0; n + 1];

    process_dfs(&graph, &mut dist_a, a, 0, 0);
    process_dfs(&graph, &mut dist_b, b, 0, 0);
    process_dfs(&graph, &mut dist_c, c, 0, 0);

    let mut ret = false;

    for i in 1..=n {
        if graph[i].len() == 1 && dist_a[i] < dist_b[i].min(dist_c[i]) {
            ret = true;
            break;
        }
    }

    writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
}

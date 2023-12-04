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
    tracking: &mut Vec<(usize, usize, usize)>,
    curr: usize,
    parent: usize,
    depth: usize,
) {
    for &next in graph[curr].iter() {
        if next != parent {
            if depth > 0 {
                tracking.push((next, curr, 1));
            }

            process_dfs(graph, tracking, next, curr, depth + 1);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut graph_a = vec![Vec::new(); n + 1];
    let mut graph_b = vec![Vec::new(); n + 1];

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph_a[u].push(v);
        graph_a[v].push(u);
    }

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph_b[u].push(v);
        graph_b[v].push(u);
    }

    let mut tracking_a = Vec::new();
    let mut tracking_b = Vec::new();

    process_dfs(&graph_a, &mut tracking_a, 1, 0, 0);
    process_dfs(&graph_b, &mut tracking_b, 1, 0, 0);

    writeln!(out, "{}", tracking_a.len() + tracking_b.len()).unwrap();

    for (a, b, c) in tracking_a.iter() {
        writeln!(out, "{a} {b} {c}").unwrap();
    }

    for (a, b, c) in tracking_b.iter().rev() {
        writeln!(out, "{a} {c} {b}").unwrap();
    }
}

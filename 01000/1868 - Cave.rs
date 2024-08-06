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
    seen: &mut Vec<Vec<i64>>,
    rank: &mut Vec<usize>,
    ret: &mut usize,
    u: usize,
    v: usize,
) {
    for vertex in graph[u].iter() {
        if *vertex == v {
            continue;
        }

        process_dfs(graph, seen, rank, ret, *vertex, u);

        for i in 0..16 {
            seen[u][i] += seen[*vertex][i];
        }
    }

    for i in 0..16 {
        if seen[u][i] >= 2 {
            rank[u] = i + 1;
        } else if seen[u][i] == 1 && rank[u] == i {
            rank[u] += 1;
        }
    }

    seen[u][rank[u]] = 1;

    for i in 0..rank[u] {
        seen[u][i] = 0;
    }

    if *ret < rank[u] {
        *ret = rank[u];
    }
}

// Reference: https://www.secmem.org/blog/2019/07/20/Optimal-Search-On-Tree/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];
    let mut seen = vec![vec![0; 16]; n + 1];
    let mut rank = vec![0; n + 1];
    let mut ret = 0;

    for _ in 0..n - 1 {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

        graph[a].push(b);
        graph[b].push(a);
    }

    process_dfs(&graph, &mut seen, &mut rank, &mut ret, 1, 0);

    writeln!(out, "{}", ret).unwrap();
}

use io::Write;
use std::{cmp, io, str};

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
    graph: &Vec<Vec<(usize, usize)>>,
    visited: &mut Vec<bool>,
    dist: &mut Vec<usize>,
    cycles: &mut Vec<usize>,
    cur_idx: usize,
    parent: usize,
) {
    visited[cur_idx] = true;

    for edge in graph[cur_idx].iter() {
        let (next_idx, weight) = *edge;
        if next_idx == parent {
            continue;
        }

        if !visited[next_idx] {
            dist[next_idx] = dist[cur_idx] ^ weight;
            process_dfs(graph, visited, dist, cycles, next_idx, cur_idx);
        } else {
            cycles.push(dist[cur_idx] ^ dist[next_idx] ^ weight);
        }
    }
}

fn process_gaussian_elimination(cycles: &mut Vec<usize>, basis: &mut Vec<usize>) {
    if cycles.is_empty() {
        return;
    }

    loop {
        let val_max = *cycles.iter().max().unwrap();
        basis.push(val_max);

        for val in cycles.iter_mut() {
            *val = cmp::min(*val, *val ^ val_max);
        }

        if val_max == 0 {
            break;
        }
    }
}

// Reference: https://seastar105.tistory.com/112
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (x, y, w) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        graph[x].push((y, w));
        graph[y].push((x, w));
    }

    let mut visited = vec![false; n + 1];
    let mut dist = vec![0; n + 1];
    let mut cycles = Vec::new();
    let mut basis = Vec::new();

    process_dfs(&graph, &mut visited, &mut dist, &mut cycles, 1, 0);
    process_gaussian_elimination(&mut cycles, &mut basis);

    for _ in 0..q {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        let mut ret = dist[a] ^ dist[b];

        for val in basis.iter() {
            ret = cmp::min(ret, ret ^ val);
        }

        writeln!(out, "{ret}").unwrap();
    }
}

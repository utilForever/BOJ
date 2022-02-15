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

// Reference: https://www.secmem.org/blog/2019/12/12/HLD/
fn process_dfs(
    graph: &Vec<Vec<(usize, usize)>>,
    parent: &mut Vec<usize>,
    size: &mut Vec<usize>,
    dist: &mut Vec<usize>,
    cur_idx: usize,
    parent_idx: usize,
    cur_dist: usize,
) -> usize {
    parent[cur_idx] = parent_idx;
    size[cur_idx] = 1;
    dist[cur_idx] = cur_dist;

    for vertex in graph[cur_idx].iter() {
        if vertex.0 != parent_idx {
            size[cur_idx] += process_dfs(
                graph,
                parent,
                size,
                dist,
                vertex.0,
                cur_idx,
                cur_dist + vertex.1,
            );
        }
    }

    size[cur_idx]
}

fn process_hld(
    graph: &Vec<Vec<(usize, usize)>>,
    size: &Vec<usize>,
    depth: &mut Vec<usize>,
    chain_number: &mut Vec<usize>,
    chain_index: &mut Vec<usize>,
    chain: &mut Vec<Vec<usize>>,
    cur_idx: usize,
    parent_idx: usize,
    cur_chain: usize,
    cur_depth: usize,
) {
    depth[cur_idx] = cur_depth;
    chain_number[cur_idx] = cur_chain;
    chain_index[cur_idx] = chain[cur_chain].len();
    chain[cur_chain].push(cur_idx);

    let mut heavy = -1;

    for vertex in graph[cur_idx].iter() {
        if vertex.0 != parent_idx && (heavy == -1 || size[vertex.0] > size[heavy as usize]) {
            heavy = vertex.0 as i64;
        }
    }

    if heavy != -1 {
        process_hld(
            graph,
            size,
            depth,
            chain_number,
            chain_index,
            chain,
            heavy as usize,
            cur_idx,
            cur_chain,
            cur_depth,
        );
    }

    for vertex in graph[cur_idx].iter() {
        if vertex.0 != parent_idx && vertex.0 as i64 != heavy {
            process_hld(
                graph,
                size,
                depth,
                chain_number,
                chain_index,
                chain,
                vertex.0,
                cur_idx,
                vertex.0,
                cur_depth + 1,
            );
        }
    }
}

fn find_lca(
    parent: &Vec<usize>,
    depth: &Vec<usize>,
    chain_number: &Vec<usize>,
    chain_index: &Vec<usize>,
    mut a: usize,
    mut b: usize,
) -> usize {
    while chain_number[a] != chain_number[b] {
        if depth[a] > depth[b] {
            a = parent[chain_number[a]];
        } else {
            b = parent[chain_number[b]];
        }
    }

    if chain_index[a] > chain_index[b] {
        b
    } else {
        a
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..(n - 1) {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let mut parent = vec![0; n + 1];
    let mut size = vec![0; n + 1];
    let mut dist = vec![0; n + 1];

    process_dfs(&graph, &mut parent, &mut size, &mut dist, 1, 0, 0);

    let mut depth = vec![0; n + 1];
    let mut chain_number = vec![0; n + 1];
    let mut chain_index = vec![0; n + 1];
    let mut chain = vec![Vec::new(); n + 1];

    process_hld(
        &graph,
        &size,
        &mut depth,
        &mut chain_number,
        &mut chain_index,
        &mut chain,
        1,
        0,
        1,
        0,
    );

    let m = scan.token::<usize>();

    for _ in 0..m {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        let lca = find_lca(&parent, &depth, &chain_number, &chain_index, a, b);
        writeln!(out, "{}", dist[a] + dist[b] - 2 * dist[lca]).unwrap();
    }
}

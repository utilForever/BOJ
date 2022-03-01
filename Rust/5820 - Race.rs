use io::Write;
use std::{
    cmp,
    collections::{BTreeMap, BTreeSet},
    io, str,
};

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

fn get_subtree_sizes(
    graph: &Vec<BTreeSet<usize>>,
    subtree: &mut Vec<usize>,
    node: usize,
    parent: usize,
) {
    subtree[node] = 1;

    for idx in graph[node].iter() {
        if *idx != parent {
            get_subtree_sizes(graph, subtree, *idx, node);
            subtree[node] += subtree[*idx];
        }
    }
}

fn get_centroid(
    graph: &Vec<BTreeSet<usize>>,
    subtree: &Vec<usize>,
    node: usize,
    parent: usize,
    tree_size: usize,
) -> usize {
    for idx in graph[node].iter() {
        if *idx != parent && subtree[*idx] > tree_size {
            return get_centroid(graph, subtree, *idx, node, tree_size);
        }
    }

    node
}

fn process_dfs(
    graph: &Vec<BTreeSet<usize>>,
    costs: &Vec<BTreeMap<usize, usize>>,
    achievable: &mut Vec<usize>,
    min_paths: &mut Vec<usize>,
    component: &mut usize,
    num_min_highways: &mut usize,
    k: usize,
    node: usize,
    parent: usize,
    cost: usize,
    depth: usize,
    filling: bool,
) {
    if cost > k {
        return;
    }

    if filling {
        if achievable[k - cost] == *component {
            *num_min_highways = cmp::min(*num_min_highways, depth + min_paths[k - cost]);
        }

        if cost == k {
            *num_min_highways = cmp::min(*num_min_highways, depth);
        }
    } else {
        if achievable[cost] < *component || depth < min_paths[cost] {
            achievable[cost] = *component;
            min_paths[cost] = depth;
        }
    }

    for idx in graph[node].iter() {
        if *idx != parent {
            process_dfs(
                graph,
                costs,
                achievable,
                min_paths,
                component,
                num_min_highways,
                k,
                *idx,
                node,
                cost + costs[node][idx],
                depth + 1,
                filling,
            );
        }
    }
}

fn calculate_best_path(
    graph: &mut Vec<BTreeSet<usize>>,
    costs: &Vec<BTreeMap<usize, usize>>,
    subtree: &mut Vec<usize>,
    achievable: &mut Vec<usize>,
    min_paths: &mut Vec<usize>,
    component: &mut usize,
    num_min_highways: &mut usize,
    k: usize,
    node: usize,
) {
    get_subtree_sizes(graph, subtree, node, 0);
    let centroid = get_centroid(graph, subtree, node, 0, subtree[node] / 2);

    *component += 1;

    for idx in graph[centroid].iter() {
        process_dfs(
            graph,
            costs,
            achievable,
            min_paths,
            component,
            num_min_highways,
            k,
            *idx,
            centroid,
            costs[centroid][idx],
            1,
            true,
        );
        process_dfs(
            graph,
            costs,
            achievable,
            min_paths,
            component,
            num_min_highways,
            k,
            *idx,
            centroid,
            costs[centroid][idx],
            1,
            false,
        );
    }

    for idx in graph[centroid].clone().iter() {
        graph[*idx].remove(&centroid);
        graph[centroid].remove(idx);

        calculate_best_path(
            graph,
            costs,
            subtree,
            achievable,
            min_paths,
            component,
            num_min_highways,
            k,
            *idx,
        );
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut graph: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); 200_001];
    let mut costs: Vec<BTreeMap<usize, usize>> = vec![BTreeMap::new(); 200_001];

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());

    for _ in 1..n {
        let (h0, h1, l) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        graph[h0].insert(h1);
        graph[h1].insert(h0);
        costs[h0].insert(h1, l);
        costs[h1].insert(h0, l);
    }

    let mut subtree: Vec<usize> = vec![0; 200_001];
    let mut achievable: Vec<usize> = vec![0; 1_000_001];
    let mut min_paths: Vec<usize> = vec![0; 1_000_001];
    let mut component: usize = 0;
    let mut num_min_highways: usize = 200_001;

    calculate_best_path(
        &mut graph,
        &costs,
        &mut subtree,
        &mut achievable,
        &mut min_paths,
        &mut component,
        &mut num_min_highways,
        k,
        1,
    );

    writeln!(
        out,
        "{}",
        if num_min_highways == 200_001 {
            -1
        } else {
            num_min_highways as i64
        }
    )
    .unwrap();
}

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

struct CographicOracle {
    n: usize,
    edges: Vec<(usize, usize)>,
    edge_colors: Vec<usize>,
    color_excluded: usize,
    graph: Vec<Vec<usize>>,
    preorder: Vec<usize>,
    low_link: Vec<usize>,
    timestamp: usize,
}

impl CographicOracle {
    fn new(
        n: usize,
        edges: Vec<(usize, usize)>,
        edge_colors: Vec<usize>,
        color_excluded: usize,
    ) -> Self {
        Self {
            n,
            edges,
            edge_colors,
            color_excluded,
            graph: Vec::new(),
            preorder: Vec::new(),
            low_link: Vec::new(),
            timestamp: 0,
        }
    }

    fn dfs(&mut self, vertex: usize, parent: i64) -> usize {
        self.timestamp += 1;
        self.preorder[vertex] = self.timestamp;
        self.low_link[vertex] = self.timestamp;

        let edges = self.graph[vertex].clone();
        let mut is_skipped = false;

        for &vertex_next in edges.iter() {
            if (vertex_next as i64) == parent && !is_skipped {
                is_skipped = true;
                continue;
            }

            self.low_link[vertex] = if self.preorder[vertex_next] == 0 {
                let val = self.dfs(vertex_next, vertex as i64);
                self.low_link[vertex].min(val)
            } else {
                self.low_link[vertex].min(self.preorder[vertex_next])
            };
        }

        self.low_link[vertex]
    }

    fn init(&mut self, selected: &[bool]) {
        self.graph = vec![Vec::new(); self.n];
        self.preorder = vec![0; self.n];
        self.low_link = vec![0; self.n];
        self.timestamp = 0;

        for (i, &is_selected) in selected.iter().enumerate() {
            if !is_selected {
                let (u, v) = self.edges[i];

                if u == v {
                    continue;
                }

                if self.edge_colors[i] == self.color_excluded {
                    continue;
                }

                self.graph[u].push(v);
                self.graph[v].push(u);
            }
        }

        for v in 0..self.n {
            if self.preorder[v] == 0 {
                self.dfs(v, -1);
            }
        }
    }

    fn can_add(&self, edge_index: usize) -> bool {
        let (u, v) = self.edges[edge_index];

        if u == v || self.edge_colors[edge_index] == self.color_excluded {
            return true;
        }

        let preorder_max = self.preorder[u].max(self.preorder[v]);
        let low_link_max = self.low_link[u].max(self.low_link[v]);

        preorder_max != low_link_max
    }
}

fn maximize_matroid_intersection(
    oracle_a: &mut CographicOracle,
    oracle_b: &mut CographicOracle,
    n: usize,
    weights: &[i64],
) -> Vec<i64> {
    let mut selected = vec![false; n];
    let mut cnt = 0;

    let mut ret = vec![-1; n + 1];
    ret[0] = 0;

    loop {
        oracle_a.init(&selected);
        oracle_b.init(&selected);

        let mut graph: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
        let mut is_sink = vec![false; n];

        let mut cost = vec![(i64::MAX, i64::MAX); n];
        let mut prev = vec![-1; n];

        for i in 0..n {
            if selected[i] {
                continue;
            }

            if oracle_a.can_add(i) {
                cost[i] = (-weights[i], 0);
                prev[i] = -2;
            }

            is_sink[i] = oracle_b.can_add(i);
        }

        // Build graph
        for i in 0..n {
            if !selected[i] {
                continue;
            }

            // Temporarily unselect i for building graph
            selected[i] = false;

            oracle_a.init(&selected);
            oracle_b.init(&selected);

            for j in 0..n {
                if i != j && !selected[j] {
                    if oracle_a.can_add(j) {
                        // from i to j with cost = -edge_weights[j]
                        graph[i].push((j, -weights[j]));
                    }

                    if oracle_b.can_add(j) {
                        // from j to i with cost = edge_weights[i]
                        graph[j].push((i, weights[i]));
                    }
                }
            }

            // Restore
            selected[i] = true;
        }

        // Find some path of exchanges from source to sink
        // Use Bellman-Ford because some edges may have negative weights
        let mut relaxed = true;

        while relaxed {
            relaxed = false;

            for u in 0..n {
                if prev[u] == -1 {
                    continue;
                }

                for &(v, cost_next) in graph[u].iter() {
                    let cost_new = (cost[u].0 + cost_next, cost[u].1 + 1);

                    if cost_new < cost[v] {
                        cost[v] = cost_new;
                        prev[v] = u as i64;
                        relaxed = true;
                    }
                }
            }
        }

        // Find the minimum cost sink
        let mut cost_min = (i64::MAX, i64::MAX);
        let mut edge_min = -1;

        for i in 0..n {
            if is_sink[i] && cost[i] < cost_min {
                cost_min = cost[i];
                edge_min = i as i64;
            }
        }

        // No more exchanges
        if edge_min == -1 {
            break;
        }

        selected[edge_min as usize] = true;

        let mut curr = edge_min as usize;

        // Update selected set
        while prev[curr] >= 0 {
            curr = prev[curr] as usize;
            selected[curr] = false;
            curr = prev[curr] as usize;
            selected[curr] = true;
        }

        cnt += 1;
        ret[cnt] = 0;

        for i in 0..n {
            if selected[i] {
                continue;
            }

            ret[cnt] += weights[i];
        }
    }

    ret
}

// Reference: https://codeforces.com/blog/entry/69287
// Reference: https://github.com/infossm/infossm.github.io/blob/master/_posts/2019-05-08-introduction-to-matroid.md
// Reference: https://github.com/infossm/infossm.github.io/blob/master/_posts/2019-06-17-Matroid-Intersection.md
// Reference: https://github.com/ahsoltan/kactl/blob/main/content/combinatorial/WeightedMatroidIsect.h
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut edges = vec![(0, 0); m];
    let mut weights = vec![0; m];
    let mut colors = vec![0; m];

    for i in 0..m {
        let (a, b, w, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<char>(),
        );

        edges[i] = (a - 1, b - 1);
        weights[i] = w;
        // 00 = Red, 01 = Blue, 10 = Green
        colors[i] = match c {
            'R' => 0,
            'B' => 1,
            'G' => 2,
            _ => unreachable!(),
        };
    }

    // To minimize the total weight, we need to maximize the total weight of the complement set
    let mut oracle_r = CographicOracle::new(n, edges.clone(), colors.clone(), 0);
    let mut oracle_b = CographicOracle::new(n, edges.clone(), colors.clone(), 1);

    let mut ret = maximize_matroid_intersection(&mut oracle_r, &mut oracle_b, m, &weights);
    ret[0] = weights.iter().sum::<i64>();

    for i in (0..m).rev() {
        if ret[i] == -1 {
            writeln!(out, "-1").unwrap();
        } else {
            writeln!(out, "{}", ret[i]).unwrap();
        }
    }
}

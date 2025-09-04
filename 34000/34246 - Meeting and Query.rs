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

fn lca(
    ancestor: &Vec<Vec<usize>>,
    depth: &Vec<i64>,
    mut a: usize,
    mut b: usize,
    log: usize,
) -> usize {
    if depth[a] < depth[b] {
        std::mem::swap(&mut a, &mut b);
    }

    let diff = (depth[a] - depth[b]) as usize;

    for p in 0..log {
        if (diff >> p) & 1 == 1 {
            a = ancestor[p][a];
        }
    }

    if a == b {
        return a;
    }

    for p in (0..log).rev() {
        if ancestor[p][a] != ancestor[p][b] {
            a = ancestor[p][a];
            b = ancestor[p][b];
        }
    }

    ancestor[0][a]
}

fn distance(
    ancestor: &Vec<Vec<usize>>,
    depth: &Vec<i64>,
    dist_from_root: &Vec<i64>,
    u: usize,
    v: usize,
    log: usize,
) -> i64 {
    let w = lca(ancestor, depth, u, v, log);
    dist_from_root[u] + dist_from_root[v] - 2 * dist_from_root[w]
}

fn merge_diameter_endpoints(
    ancestor: &Vec<Vec<usize>>,
    depth: &Vec<i64>,
    dist_from_root: &Vec<i64>,
    a: (usize, usize),
    b: (usize, usize),
    log: usize,
) -> (usize, usize) {
    if a.0 == 0 {
        return b;
    }

    if b.0 == 0 {
        return a;
    }

    let candidates = [a.0, a.1, b.0, b.1];
    let mut best_pair = (0, 0);
    let mut best_dist = -1;

    for i in 0..4 {
        for j in i + 1..4 {
            if candidates[i] == 0 || candidates[j] == 0 {
                continue;
            }

            let dist = distance(
                ancestor,
                depth,
                dist_from_root,
                candidates[i],
                candidates[j],
                log,
            );

            if dist > best_dist {
                best_dist = dist;
                best_pair = (candidates[i], candidates[j]);
            }
        }
    }

    best_pair
}

fn climb_up_by_weight_upper(
    ancestor: &Vec<Vec<usize>>,
    jump_weight: &Vec<Vec<i64>>,
    mut u: usize,
    mut upper: i64,
    log: usize,
) -> usize {
    for p in (0..log).rev() {
        if ancestor[p][u] != 0 && jump_weight[p][u] <= upper {
            upper -= jump_weight[p][u];
            u = ancestor[p][u];
        }
    }

    u
}

fn climb_up_by_weight_lower(
    ancestor: &Vec<Vec<usize>>,
    jump_weight: &Vec<Vec<i64>>,
    mut u: usize,
    mut lower: i64,
    log: usize,
) -> usize {
    if lower <= 0 {
        return u;
    }

    for p in (0..log).rev() {
        if ancestor[p][u] != 0 && jump_weight[p][u] < lower {
            lower -= jump_weight[p][u];
            u = ancestor[p][u];
        }
    }

    if ancestor[0][u] != 0 {
        ancestor[0][u]
    } else {
        u
    }
}

fn furthest_vertex_within_on_path(
    ancestor: &Vec<Vec<usize>>,
    jump_weight: &Vec<Vec<i64>>,
    depth: &Vec<i64>,
    dist_from_root: &Vec<i64>,
    u: usize,
    v: usize,
    t: i64,
    log: usize,
) -> usize {
    let w = lca(ancestor, depth, u, v, log);
    let dist_uw = distance(ancestor, depth, dist_from_root, u, w, log);
    let dist_vw = distance(ancestor, depth, dist_from_root, v, w, log);

    if t <= dist_uw {
        climb_up_by_weight_upper(ancestor, jump_weight, u, t, log)
    } else {
        climb_up_by_weight_lower(ancestor, jump_weight, v, dist_vw - (t - dist_uw), log)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..n - 1 {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let mut log = 1;

    while (1usize << log) <= n {
        log += 1;
    }

    let mut ancestor = vec![vec![0; n + 1]; log];
    let mut jump_weight = vec![vec![0; n + 1]; log];
    let mut depth = vec![0; n + 1];
    let mut dist_from_root = vec![0; n + 1];

    let mut stack = Vec::with_capacity(n);
    let mut visited = vec![false; n + 1];

    stack.push(1);
    visited[1] = true;

    while let Some(u) = stack.pop() {
        for &(v, w) in graph[u].iter() {
            if visited[v] {
                continue;
            }

            ancestor[0][v] = u;
            jump_weight[0][v] = w;
            depth[v] = depth[u] + 1;
            dist_from_root[v] = dist_from_root[u] + w;

            stack.push(v);
            visited[v] = true;
        }
    }

    for p in 1..log {
        for v in 1..=n {
            let parent = ancestor[p - 1][v];

            ancestor[p][v] = if parent != 0 {
                ancestor[p - 1][parent]
            } else {
                0
            };
            jump_weight[p][v] = jump_weight[p - 1][v]
                + if parent != 0 {
                    jump_weight[p - 1][parent]
                } else {
                    0
                };
        }
    }

    let mut floor = vec![0; n + 1];

    for i in 2..=n {
        floor[i] = floor[i / 2] + 1;
    }

    let levels = floor[n] + 1;
    let mut diameters = vec![vec![(0, 0); n + 1]; levels];

    for i in 1..=n {
        diameters[0][i] = (i, i);
    }

    for level in 1..levels {
        let len = 1usize << level;
        let half = len >> 1;

        for i in 1..=n {
            if i + len - 1 > n {
                break;
            }

            let pair_left = diameters[level - 1][i];
            let pair_right = diameters[level - 1][i + half];

            diameters[level][i] = merge_diameter_endpoints(
                &ancestor,
                &depth,
                &dist_from_root,
                pair_left,
                pair_right,
                log,
            );
        }
    }

    for _ in 0..q {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
        let len = r - l + 1;
        let level = floor[len];

        let pair1 = diameters[level][l];
        let pair2 = diameters[level][r + 1 - (1 << level)];

        let (end_a, end_b) =
            merge_diameter_endpoints(&ancestor, &depth, &dist_from_root, pair1, pair2, log);
        let dist_ab = distance(&ancestor, &depth, &dist_from_root, end_a, end_b, log);

        if dist_ab == 0 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let half = dist_ab / 2;
        let candidate_from_a = furthest_vertex_within_on_path(
            &ancestor,
            &jump_weight,
            &depth,
            &dist_from_root,
            end_a,
            end_b,
            half,
            log,
        );
        let candidate_from_b = furthest_vertex_within_on_path(
            &ancestor,
            &jump_weight,
            &depth,
            &dist_from_root,
            end_b,
            end_a,
            half,
            log,
        );

        let dist_a = distance(
            &ancestor,
            &depth,
            &dist_from_root,
            end_a,
            candidate_from_a,
            log,
        );
        let dist_b = distance(
            &ancestor,
            &depth,
            &dist_from_root,
            end_a,
            candidate_from_b,
            log,
        );

        let radius1 = dist_a.max(dist_ab - dist_a);
        let radius2 = dist_b.max(dist_ab - dist_b);

        writeln!(out, "{}", radius1.min(radius2)).unwrap();
    }
}

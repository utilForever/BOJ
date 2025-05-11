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

static mut PARENT: [[usize; 100001]; 20] = [[0; 100001]; 20];
static mut DIST_MIN: [[i64; 100001]; 20] = [[i64::MAX; 100001]; 20];
static mut DIST_MAX: [[i64; 100001]; 20] = [[i64::MIN; 100001]; 20];
static mut DEPTH: [usize; 100001] = [0; 100001];

unsafe fn process_dfs(graph: &Vec<Vec<(usize, i64)>>, node: usize, parent: usize, cost: i64) {
    PARENT[0][node] = parent;
    DIST_MIN[0][node] = cost;
    DIST_MAX[0][node] = cost;

    for &(next, cost) in graph[node].iter() {
        if next == parent {
            continue;
        }

        DEPTH[next] = DEPTH[node] + 1;
        process_dfs(graph, next, node, cost);
    }
}

unsafe fn build_tables(graph: &Vec<Vec<(usize, i64)>>, n: usize, len: usize) {
    DEPTH[1] = 0;

    process_dfs(graph, 1, 0, i64::MAX);

    for k in 1..len {
        for v in 1..=n {
            let mid = PARENT[k - 1][v];

            PARENT[k][v] = PARENT[k - 1][mid];
            DIST_MIN[k][v] = DIST_MIN[k - 1][v].min(DIST_MIN[k - 1][mid]);
            DIST_MAX[k][v] = DIST_MAX[k - 1][v].max(DIST_MAX[k - 1][mid]);
        }
    }
}

unsafe fn query(mut u: usize, mut v: usize, log: usize) -> (i64, i64) {
    let mut ret_min = i64::MAX;
    let mut ret_max = i64::MIN;

    if DEPTH[u] < DEPTH[v] {
        std::mem::swap(&mut u, &mut v);
    }

    let diff = DEPTH[u] - DEPTH[v];

    for k in (0..log).rev() {
        if diff & (1 << k) != 0 {
            ret_min = ret_min.min(DIST_MIN[k][u]);
            ret_max = ret_max.max(DIST_MAX[k][u]);
            u = PARENT[k][u];
        }
    }

    if u == v {
        return (ret_min, ret_max);
    }

    for k in (0..log).rev() {
        if PARENT[k][u] != PARENT[k][v] {
            ret_min = ret_min.min(DIST_MIN[k][u]);
            ret_min = ret_min.min(DIST_MIN[k][v]);
            ret_max = ret_max.max(DIST_MAX[k][u]);
            ret_max = ret_max.max(DIST_MAX[k][v]);
            u = PARENT[k][u];
            v = PARENT[k][v];
        }
    }

    ret_min = ret_min.min(DIST_MIN[0][u]);
    ret_min = ret_min.min(DIST_MIN[0][v]);
    ret_max = ret_max.max(DIST_MAX[0][u]);
    ret_max = ret_max.max(DIST_MAX[0][v]);

    (ret_min, ret_max)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
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

    let len = (n as f64).log(2.0).ceil() as usize;

    unsafe {
        build_tables(&graph, n, len);
    }

    let k = scan.token::<i64>();

    unsafe {
        for _ in 0..k {
            let (d, e) = (scan.token::<usize>(), scan.token::<usize>());
            let (dist_shortest, dist_longest) = query(d, e, len);

            writeln!(out, "{dist_shortest} {dist_longest}").unwrap();
        }
    }
}

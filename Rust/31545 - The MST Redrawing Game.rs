use io::Write;
use std::{
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

static MOD: i64 = 1_000_000_007;

fn pow(x: i64, y: i64) -> i64 {
    if y == 0 {
        return 1;
    }

    pow((x * x) % MOD, y / 2) * if y % 2 == 0 { 1 } else { x } % MOD
}

fn determinant(mat: &mut Vec<Vec<i64>>) -> i64 {
    let mut n = mat.len();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            mat[i][i] += mat[i][j];
            mat[i][j] *= -1;
        }

        mat[i][i] %= MOD;
    }

    n -= 1;

    for i in 0..n {
        let val = pow(mat[i][i], MOD - 2);

        for j in i + 1..n {
            let temp = val * mat[j][i] % MOD;

            for k in i..n {
                mat[j][k] = (mat[j][k] - mat[i][k] * temp) % MOD;
            }
        }
    }

    let mut ret = 1;

    for i in 0..n {
        ret = (ret * mat[i][i]) % MOD;
    }

    ret
}

fn find(parent: &mut Vec<usize>, node: usize) -> usize {
    if parent[node] == node {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (v, e) = (scan.token::<usize>(), scan.token::<usize>());
    let mut edges = vec![(0, (0, 0)); e];

    for i in 0..e {
        let (u, v, w) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        edges[i] = (w, (u - 1, v - 1));
    }

    edges.sort_unstable();

    let mut parent = vec![0; v];
    let mut subparent = vec![0; v];

    for i in 0..v {
        parent[i] = i;
        subparent[i] = i;
    }

    let mut weights = Vec::new();
    weights.push(edges[0].0);

    for i in 1..e {
        if weights[weights.len() - 1] != edges[i].0 {
            weights.push(edges[i].0);
        }
    }

    let mut cnt_weights = Vec::new();
    let mut edges_new: Vec<(usize, usize)> = Vec::new();
    let mut components = vec![BTreeSet::new(); v];
    let mut curr = 0;

    for i in 0..e {
        if edges[i].0 != weights[curr] {
            let mut subgraph = vec![Vec::new(); v];
            let mut map = BTreeMap::new();

            for j in 0..v {
                components[find(&mut parent, j)].insert(find(&mut subparent, j));
            }

            for j in 0..v {
                if components[j].is_empty() {
                    continue;
                }

                let temp_n = components[j].len();
                let mut cnt = 0;

                subgraph[j].resize(temp_n, vec![0; temp_n]);

                for &compnent in components[j].iter() {
                    map.entry(compnent).or_insert((j, cnt));
                    cnt += 1;
                }
            }

            for j in 0..edges_new.len() {
                let p = map[&find(&mut subparent, edges_new[j].0)];
                let q = map[&find(&mut subparent, edges_new[j].1)];

                if p.1 == q.1 {
                    continue;
                }

                subgraph[p.0][p.1][q.1] += 1;
                subgraph[p.0][q.1][p.1] += 1;
            }

            cnt_weights.push((weights[curr], 1));

            for j in 0..v {
                if subgraph[j].len() == 0 {
                    continue;
                }

                let len = cnt_weights.len();
                cnt_weights[len - 1].1 =
                    (cnt_weights[len - 1].1 * determinant(&mut subgraph[j])) % MOD;
            }

            curr += 1;

            // Reset
            for j in 0..v {
                subparent[j] = parent[j];
                components[j].clear();
            }

            edges_new.clear();
        }

        let p = find(&mut parent, edges[i].1 .0);
        let q = find(&mut parent, edges[i].1 .1);

        if p != q {
            parent[q] = p;
        }

        edges_new.push(edges[i].1);
    }

    let mut subgraph = vec![Vec::new(); v];
    let mut map = BTreeMap::new();

    for i in 0..v {
        components[find(&mut parent, i)].insert(find(&mut subparent, i));
    }

    for i in 0..v {
        if components[i].is_empty() {
            continue;
        }

        let temp_n = components[i].len();
        let mut cnt = 0;

        subgraph[i].resize(temp_n, vec![0; temp_n]);

        for &compnent in components[i].iter() {
            map.entry(compnent).or_insert((i, cnt));
            cnt += 1;
        }
    }

    for j in 0..edges_new.len() {
        let p = map[&find(&mut subparent, edges_new[j].0)];
        let q = map[&find(&mut subparent, edges_new[j].1)];

        if p.1 == q.1 {
            continue;
        }

        subgraph[p.0][p.1][q.1] += 1;
        subgraph[p.0][q.1][p.1] += 1;
    }

    cnt_weights.push((weights[curr], 1));

    for i in 0..v {
        if subgraph[i].len() == 0 {
            continue;
        }

        let len = cnt_weights.len();
        cnt_weights[len - 1].1 = (cnt_weights[len - 1].1 * determinant(&mut subgraph[i])) % MOD;
    }

    let mut ret = 1;

    for i in 0..cnt_weights.len() {
        let mut val = (cnt_weights[i].1 + cnt_weights[i].0 - 1) % MOD;
        val = (val * pow(cnt_weights[i].0, MOD - 2)) % MOD;
        ret = (ret * val) % MOD;
    }

    if ret < 0 {
        ret += MOD;
    }

    writeln!(out, "{ret}").unwrap();
}

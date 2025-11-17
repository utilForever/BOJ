use io::Write;
use std::{collections::HashMap, io, str};

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

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![0; n + 1],
            size: vec![1; n + 1],
        }
    }

    fn init(&mut self) {
        for i in 0..self.parent.len() {
            self.parent[i] = i;
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }

        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];

        true
    }
}

fn lower_bound(v: &Vec<usize>, x: usize) -> usize {
    let mut left = 0;
    let mut right = v.len();

    while left < right {
        let mid = (left + right) / 2;

        if v[mid] < x {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut weight = vec![0; n + 1];
    let mut is_open = vec![false; n + 1];

    for i in 1..=n {
        let parenthesis = scan.token::<i64>();

        weight[i] = if parenthesis == 0 { 1 } else { -1 };
        is_open[i] = parenthesis == 0;
    }

    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut parent = vec![0; n + 1];
    let mut time_in = vec![0; n + 1];
    let mut time_out = vec![0; n + 1];
    let mut sum = vec![0; n + 1];

    let mut time = 0;
    let mut stack = Vec::with_capacity(2 * n);

    stack.push((1, 0, 0));

    while let Some((u, p, state)) = stack.pop() {
        if state == 0 {
            parent[u] = p;
            sum[u] = if p == 0 {
                weight[u]
            } else {
                sum[p] + weight[u]
            };

            time += 1;
            time_in[u] = time;

            stack.push((u, p, 1));

            for &v in graph[u].iter().rev() {
                if v != p {
                    stack.push((v, u, 0));
                }
            }
        } else {
            time_out[u] = time;
        }
    }

    let mut is_leaf = vec![false; n + 1];

    for i in 1..=n {
        is_leaf[i] = if i == 1 {
            graph[i].is_empty()
        } else {
            graph[i].len() == 1
        };
    }

    let mut nodes_by_sum: HashMap<i64, Vec<usize>> = HashMap::new();
    let mut leaves_by_sum: HashMap<i64, Vec<usize>> = HashMap::new();

    for v in 1..=n {
        nodes_by_sum.entry(sum[v]).or_default().push(v);

        if is_leaf[v] {
            leaves_by_sum.entry(sum[v]).or_default().push(v);
        }
    }

    let mut queries_by_time: HashMap<i64, Vec<usize>> = HashMap::new();

    for i in 1..=n {
        if !is_open[i] {
            continue;
        }

        let base = if parent[i] == 0 { 0 } else { sum[parent[i]] };
        queries_by_time.entry(base).or_default().push(i);
    }

    let mut levels = Vec::new();
    levels.extend(nodes_by_sum.keys().copied());
    levels.extend(queries_by_time.keys().copied());
    levels.sort_unstable();
    levels.dedup();
    levels.sort_unstable_by(|a, b| b.cmp(a));

    let mut union_find = UnionFind::new(n + 1);
    union_find.init();

    let mut alive = vec![false; n + 1];
    let mut ret = 0;

    for level in levels {
        if let Some(nodes) = nodes_by_sum.get(&level) {
            for &node in nodes {
                alive[node] = true;
            }

            for &node1 in nodes {
                for &node2 in graph[node1].iter() {
                    if alive[node2] {
                        union_find.union(node1, node2);
                    }
                }
            }
        }

        let mut components: HashMap<usize, Vec<usize>> = HashMap::new();

        if let Some(leaves) = leaves_by_sum.get(&level) {
            for &leave in leaves {
                let root = union_find.find(leave);
                components.entry(root).or_default().push(time_in[leave]);
            }
        }

        for component in components.values_mut() {
            component.sort_unstable();
        }

        if let Some(queires) = queries_by_time.get(&level) {
            for &query in queires {
                let root = union_find.find(query);

                if let Some(component) = components.get(&root) {
                    let idx = lower_bound(component, time_in[query]);

                    if idx < component.len() && component[idx] <= time_out[query] {
                        ret += 1;
                    }
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

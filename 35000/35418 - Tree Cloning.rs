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

struct IsomorphismHasher {
    mapping: HashMap<Vec<usize>, usize>,
    next: usize,
}

impl IsomorphismHasher {
    fn new() -> Self {
        let mut mapping = HashMap::new();
        mapping.insert(Vec::new(), 1);

        Self { mapping, next: 2 }
    }

    fn encode(&mut self, childs: &mut Vec<usize>) -> usize {
        childs.sort_unstable();

        if let Some(&idx) = self.mapping.get(childs) {
            return idx;
        }

        let idx = self.next;

        self.next += 1;
        self.mapping.insert(std::mem::take(childs), idx);

        idx
    }
}

fn rooted_hash(
    graph: &Vec<Vec<(usize, usize)>>,
    cut: &Vec<bool>,
    iso: &mut IsomorphismHasher,
    hash_ids: &mut Vec<usize>,
    root: usize,
) -> usize {
    let mut order = Vec::new();
    let mut stack = Vec::new();

    stack.push((root, 0));

    while let Some((u, p)) = stack.pop() {
        order.push((u, p));

        for &(v, idx) in graph[u].iter() {
            if cut[idx] || v == p {
                continue;
            }

            stack.push((v, u));
        }
    }

    for &(u, p) in order.iter().rev() {
        let mut childs = Vec::new();

        for &(v, idx) in graph[u].iter() {
            if cut[idx] || v == p {
                continue;
            }

            childs.push(hash_ids[v]);
        }

        hash_ids[u] = iso.encode(&mut childs);
    }

    hash_ids[root]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];

    for idx in 0..n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());

        graph[u].push((v, idx));
        graph[v].push((u, idx));
    }

    let mut parent = vec![0; n + 1];
    let mut parent_idx = vec![0; n + 1];
    let mut order = Vec::with_capacity(n);
    let mut stack = Vec::new();

    stack.push((1, 0));

    while let Some((u, p)) = stack.pop() {
        parent[u] = p;
        order.push(u);

        for &(v, idx) in graph[u].iter() {
            if v == p {
                continue;
            }

            parent_idx[v] = idx;
            stack.push((v, u));
        }
    }

    let mut size_subtree = vec![1; n + 1];

    for &node in order.iter().rev() {
        if parent[node] != 0 {
            size_subtree[parent[node]] += size_subtree[node];
        }
    }

    let mut divisors = Vec::new();

    for i in 2..=(n as f64).sqrt() as usize {
        if n % i == 0 {
            divisors.push(i);

            if i * i != n {
                divisors.push(n / i);
            }
        }
    }

    divisors.push(1);
    divisors.push(n);
    divisors.sort_unstable();

    'outer: for a in divisors {
        let b = n / a;
        let mut cut = vec![false; n - 1];

        for i in 2..=n {
            if size_subtree[i] % a == 0 {
                cut[parent_idx[i]] = true;
            }
        }

        let mut component = vec![usize::MAX; n + 1];
        let mut roots = Vec::new();
        let mut degrees = Vec::new();
        let mut check = true;

        for i in 1..=n {
            if component[i] != usize::MAX {
                continue;
            }

            let mut stack = Vec::new();

            component[i] = roots.len();
            stack.push(i);

            let mut size = 0;
            let mut root = 0;
            let mut degree = 0;

            while let Some(u) = stack.pop() {
                size += 1;

                for &(v, idx) in &graph[u] {
                    if cut[idx] {
                        degree += 1;

                        if root == 0 {
                            root = u;
                        } else if root != u {
                            check = false;
                        }
                    } else if component[v] == usize::MAX {
                        component[v] = roots.len();
                        stack.push(v);
                    }
                }
            }

            if size != a || degree == 0 || degree > 2 {
                check = false;
            }

            roots.push(root);
            degrees.push(degree);
        }

        if !check || roots.len() != b || degrees.iter().filter(|&&x| x == 1).count() != 2 {
            continue;
        }

        let mut iso = IsomorphismHasher::new();
        let mut hashes = vec![0; n + 1];

        let hash0 = rooted_hash(&graph, &cut, &mut iso, &mut hashes, roots[0]);

        for &root in roots.iter().skip(1) {
            if rooted_hash(&graph, &cut, &mut iso, &mut hashes, root) != hash0 {
                continue 'outer;
            }
        }

        let mut idx_new = vec![0; n + 1];
        let mut stack = Vec::new();
        let mut ret = Vec::new();
        let mut next = 2;

        idx_new[roots[0]] = 1;
        stack.push(roots[0]);

        while let Some(u) = stack.pop() {
            for &(v, idx) in graph[u].iter() {
                if cut[idx] || idx_new[v] != 0 {
                    continue;
                }

                idx_new[v] = next;
                stack.push(v);
                ret.push((idx_new[u], idx_new[v]));
                next += 1;
            }
        }

        writeln!(out, "{b}").unwrap();
        writeln!(out, "{a}").unwrap();

        for (u, v) in ret {
            writeln!(out, "{u} {v}").unwrap();
        }

        return;
    }
}

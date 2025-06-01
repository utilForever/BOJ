use io::Write;
use std::{collections::VecDeque, io, str};

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

struct TwoSAT {
    num_vars: usize,
    graph: Vec<Vec<usize>>,
    graph_rev: Vec<Vec<usize>>,
}

impl TwoSAT {
    fn new(var_count: usize) -> Self {
        let literal_count = var_count * 2;

        Self {
            num_vars: var_count,
            graph: vec![Vec::new(); literal_count],
            graph_rev: vec![Vec::new(); literal_count],
        }
    }

    #[inline]
    fn lit_index(var: usize, is_true: bool) -> usize {
        var * 2 + is_true as usize
    }

    #[inline]
    fn negate(lit: usize) -> usize {
        lit ^ 1
    }

    fn add_or(&mut self, lit_a: usize, lit_b: usize) {
        let not_a = Self::negate(lit_a);
        let not_b = Self::negate(lit_b);

        // (¬a ⇒ b) and (¬b ⇒ a)
        self.graph[not_a].push(lit_b);
        self.graph_rev[lit_b].push(not_a);
        self.graph[not_b].push(lit_a);
        self.graph_rev[lit_a].push(not_b);
    }

    fn satisfiable(&self) -> bool {
        let lit_count = self.num_vars * 2;

        // 1. Forward DFS to obtain a topological order.
        let mut topological_order = Vec::with_capacity(lit_count);
        let mut visited = vec![false; lit_count];

        fn process_dfs(
            graph: &Vec<Vec<usize>>,
            order: &mut Vec<usize>,
            visited: &mut Vec<bool>,
            start: usize,
        ) {
            let mut stack = vec![(start, 0)];

            while let Some((node, idx)) = stack.pop() {
                if idx == 0 {
                    if visited[node] {
                        continue;
                    }

                    visited[node] = true;
                }

                if idx < graph[node].len() {
                    stack.push((node, idx + 1));

                    let next = graph[node][idx];

                    if !visited[next] {
                        stack.push((next, 0));
                    }
                } else {
                    order.push(node);
                }
            }
        }

        for lit in 0..lit_count {
            if !visited[lit] {
                process_dfs(&self.graph, &mut topological_order, &mut visited, lit);
            }
        }

        // 2. Reverse DFS to identify strongly connected components.
        let mut component_of = vec![usize::MAX; lit_count];
        let mut component_next_id = 0;

        fn process_dfs_reverse(
            graph_rev: &Vec<Vec<usize>>,
            component_of: &mut Vec<usize>,
            start: usize,
            component_id: usize,
        ) {
            let mut stack = vec![start];

            component_of[start] = component_id;

            while let Some(node) = stack.pop() {
                for &prev in graph_rev[node].iter() {
                    if component_of[prev] == usize::MAX {
                        component_of[prev] = component_id;
                        stack.push(prev);
                    }
                }
            }
        }

        for &lit in topological_order.iter().rev() {
            if component_of[lit] == usize::MAX {
                process_dfs_reverse(&self.graph_rev, &mut component_of, lit, component_next_id);
                component_next_id += 1;
            }
        }

        // 3. If a variable and its negation are in the same SCC, the formula is unsatisfiable.
        for var in 0..self.num_vars {
            if component_of[2 * var] == component_of[2 * var + 1] {
                return false;
            }
        }

        true
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Color {
    Blue,
    White,
}

#[derive(Clone)]
struct Group {
    color: Color,
    students: Vec<usize>,
}

impl Group {
    fn new(color: Color, students: Vec<usize>) -> Self {
        Self { color, students }
    }
}

fn check(groups: &Vec<Group>, matrix_friendship: &Vec<Vec<i64>>, threshold: i64, n: usize) -> bool {
    let mut graph = vec![Vec::new(); n];

    for i in 0..n {
        for j in (i + 1)..n {
            if matrix_friendship[i][j] < threshold {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut component_id = vec![usize::MAX; n];
    let mut component_cnt = 0;
    let mut color = vec![Color::Blue; n];

    for v in 0..n {
        if component_id[v] != usize::MAX {
            continue;
        }

        component_id[v] = component_cnt;
        color[v] = Color::Blue;
        queue.push_back(v);

        while let Some(x) = queue.pop_front() {
            for &y in graph[x].iter() {
                if component_id[y] == usize::MAX {
                    queue.push_back(y);

                    component_id[y] = component_cnt;
                    color[y] = if color[x] == Color::Blue {
                        Color::White
                    } else {
                        Color::Blue
                    };
                } else if color[y] == color[x] {
                    return false;
                }
            }
        }

        component_cnt += 1;
    }

    let mut sat = TwoSAT::new(component_cnt);

    for group in groups {
        let len = group.students.len();

        if len <= 1 {
            continue;
        }

        let mut lits: Vec<usize> = Vec::with_capacity(len);

        for &student in group.students.iter() {
            let cid = component_id[student];
            let c = color[student];
            let is_true = c == group.color;

            let lit = TwoSAT::lit_index(cid, is_true);
            lits.push(lit);
        }

        for i in 0..len {
            for j in (i + 1)..len {
                sat.add_or(TwoSAT::negate(lits[i]), TwoSAT::negate(lits[j]));
            }
        }
    }

    sat.satisfiable()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut groups = Vec::with_capacity(m + k);

    for _ in 0..m {
        let sz = scan.token::<usize>();
        let mut students = Vec::with_capacity(sz);

        for _ in 0..sz {
            students.push(scan.token::<usize>() - 1);
        }

        groups.push(Group::new(Color::Blue, students));
    }

    for _ in 0..k {
        let sz = scan.token::<usize>();
        let mut students = Vec::with_capacity(sz);

        for _ in 0..sz {
            students.push(scan.token::<usize>() - 1);
        }

        groups.push(Group::new(Color::White, students));
    }

    let mut matrix_friendship = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            matrix_friendship[i][j] = scan.token::<i64>();
        }
    }

    let mut left = 0;
    let mut right = 2_000_000_001;

    while left + 1 < right {
        let mid = (left + right) / 2;

        if check(&groups, &matrix_friendship, mid, n) {
            left = mid;
        } else {
            right = mid;
        }
    }

    if left == 2_000_000_000 {
        writeln!(out, "INFINITY").unwrap();
    } else {
        writeln!(out, "{left}").unwrap();
    }
}

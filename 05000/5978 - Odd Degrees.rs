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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n + 1];

    for i in 1..=m {
        let (v, w) = (scan.token::<usize>(), scan.token::<usize>());
        graph[v].push((w, i));
        graph[w].push((v, i));
    }

    let mut visited = vec![false; n + 1];
    let mut parent = vec![0; n + 1];
    let mut edge_parent = vec![0; n + 1];
    let mut children = vec![Vec::new(); n + 1];
    let mut parity_subtree = vec![0; n + 1];
    let mut ret = Vec::new();

    for i in 1..=n {
        if visited[i] {
            continue;
        }

        let mut component = Vec::new();
        let mut stack = Vec::new();

        visited[i] = true;
        parent[i] = 0;
        stack.push(i);

        while let Some(u) = stack.pop() {
            component.push(u);

            for &(v, idx) in graph[u].iter() {
                if visited[v] {
                    continue;
                }

                visited[v] = true;
                parent[v] = u;
                edge_parent[v] = idx;
                children[u].push(v);
                stack.push(v);
            }
        }

        if component.len() % 2 == 1 {
            writeln!(out, "-1").unwrap();
            return;
        }

        let mut order = Vec::with_capacity(component.len());

        {
            let mut stack = Vec::new();
            stack.push((i, 0));

            while let Some((u, idx)) = stack.pop() {
                if idx < children[u].len() {
                    stack.push((u, idx + 1));
                    stack.push((children[u][idx], 0));
                } else {
                    order.push(u);
                }
            }
        }

        for &u in order.iter() {
            let mut p = 1;

            for &child in children[u].iter() {
                p ^= parity_subtree[child];
            }

            parity_subtree[u] = p;
        }

        for &u in component.iter() {
            if parent[u] == 0 {
                continue;
            }

            if parity_subtree[u] == 1 {
                ret.push(edge_parent[u]);
            }
        }
    }

    ret.sort_unstable();

    writeln!(out, "{}", ret.len()).unwrap();

    for val in ret.iter() {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}

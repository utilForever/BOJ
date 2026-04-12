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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut graph = vec![Vec::new(); n];
    let mut graph_inv = vec![Vec::new(); n];

    for _ in 0..m {
        let (v, w, x) = (
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<i64>(),
        );

        graph[v].push((w, x % k));
        graph_inv[w].push(v);
    }

    let mut visited = vec![false; n];
    let mut order = Vec::with_capacity(n);
    let mut idxes = vec![0; n];

    for i in 0..n {
        if visited[i] {
            continue;
        }

        let mut stack = Vec::new();

        visited[i] = true;
        stack.push(i);

        while let Some(&u) = stack.last() {
            if idxes[u] < graph[u].len() {
                let v = graph[u][idxes[u]].0;

                idxes[u] += 1;

                if !visited[v] {
                    visited[v] = true;
                    stack.push(v);
                }
            } else {
                stack.pop();
                order.push(u);
            }
        }
    }

    let mut idx_component = 0;
    let mut components = vec![usize::MAX; n];
    let mut stack = Vec::new();

    for &v in order.iter().rev() {
        if components[v] != usize::MAX {
            continue;
        }

        components[v] = idx_component;
        stack.push(v);

        while let Some(u) = stack.pop() {
            for &parent in graph_inv[u].iter() {
                if components[parent] == usize::MAX {
                    components[parent] = idx_component;
                    stack.push(parent);
                }
            }
        }

        idx_component += 1;
    }

    let mut dists = vec![-1; n];

    for i in 0..n {
        if dists[i] != -1 {
            continue;
        }

        dists[i] = 0;

        let mut stack = Vec::new();
        stack.push(i);

        while let Some(u) = stack.pop() {
            for &(v, w) in graph[u].iter() {
                if components[v] != components[i] {
                    continue;
                }

                let dist_new = (dists[u] + w) % k;

                if dists[v] == -1 {
                    dists[v] = dist_new;
                    stack.push(v);
                } else if dists[v] != dist_new {
                    writeln!(out, "No").unwrap();
                    return;
                }
            }
        }
    }

    writeln!(out, "Yes").unwrap();
}

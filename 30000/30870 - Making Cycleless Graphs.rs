use io::Write;
use std::{io, str, vec};

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

fn find(parent: &mut Vec<usize>, node: usize) -> usize {
    if parent[node] == node {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn process_union(parent: &mut Vec<usize>, mut a: usize, mut b: usize) {
    a = find(parent, a);
    b = find(parent, b);

    if a == b {
        return;
    }

    parent[a] = b;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut graph = vec![Vec::new(); n + 1];
    let mut parent = vec![0; n + 1];

    for i in 1..=n {
        parent[i] = i;
    }

    for _ in 0..m {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());

        graph[u].push(v);
        graph[v].push(u);
    }

    let mut snapshot = vec![Vec::new(); n + 1];
    let mut is_removed = vec![false; n + 1];

    for _ in 0..k {
        let v = scan.token::<usize>();

        snapshot[1].push(v);
        is_removed[v] = true;
    }

    for i in 2..=n {
        for j in 0..snapshot[i - 1].len() {
            for &v in graph[snapshot[i - 1][j]].iter() {
                if is_removed[v] {
                    continue;
                }

                snapshot[i].push(v);
                is_removed[v] = true;
            }
        }
    }

    for i in (1..=n).rev() {
        let mut is_connected = false;

        for j in 0..snapshot[i].len() {
            is_removed[snapshot[i][j]] = false;

            for &v in graph[snapshot[i][j]].iter() {
                if is_removed[v] {
                    continue;
                }

                if find(&mut parent, snapshot[i][j]) == find(&mut parent, v) {
                    is_connected = true;
                    break;
                } else {
                    process_union(&mut parent, snapshot[i][j], v);
                }
            }

            if is_connected {
                break;
            }
        }

        if is_connected {
            writeln!(out, "{i}").unwrap();
            break;
        }
    }
}

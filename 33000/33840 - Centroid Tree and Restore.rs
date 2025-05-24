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

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];
    let mut edges = vec![(0, 0); n - 1];

    for i in 0..n - 1 {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

        graph[a].push(b);
        graph[b].push(a);
        edges[i] = (a, b);
    }

    // Find centroid
    let mut parent = vec![0; n + 1];
    let mut order = Vec::with_capacity(n);
    let mut stack = vec![1];

    parent[1] = 0;

    while let Some(node) = stack.pop() {
        order.push(node);

        for &next in graph[node].iter() {
            if next != parent[node] {
                parent[next] = node;
                stack.push(next);
            }
        }
    }

    let mut subtree = vec![0; n + 1];
    let mut centroid = 1;
    let mut component_best = n;

    for &node in order.iter().rev() {
        let mut size = 1;
        let mut component_max = 0;

        for &next in graph[node].iter() {
            if parent[next] == node {
                size += subtree[next];

                if subtree[next] > component_max {
                    component_max = subtree[next];
                }
            }
        }

        subtree[node] = size;

        if n - size > component_max {
            component_max = n - size;
        }

        if component_max < component_best {
            component_best = component_max;
            centroid = node;
        }
    }

    // Check decomposition
    let mut parent = vec![0; n + 1];
    let mut order = Vec::with_capacity(n);
    let mut stack = vec![centroid];

    parent[centroid] = 0;

    while let Some(node) = stack.pop() {
        order.push(node);

        for &next in graph[node].iter() {
            if next != parent[node] {
                parent[next] = node;
                stack.push(next);
            }
        }
    }

    let mut subtree = vec![0; n + 1];
    let mut ret = true;

    'outer: for &node in order.iter().rev() {
        let mut size = 1;

        for &next in graph[node].iter() {
            if parent[next] == node {
                size += subtree[next];
            }
        }

        subtree[node] = size;

        for &next in graph[node].iter() {
            if parent[next] == node && subtree[next] > size / 2 {
                ret = false;
                break 'outer;
            }
        }
    }

    if ret {
        for (u, v) in edges {
            writeln!(out, "{u} {v}").unwrap();
        }
    } else {
        writeln!(out, "-1").unwrap();
    }
}

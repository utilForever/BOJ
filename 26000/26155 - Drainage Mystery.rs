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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut pipes = vec![(0, 0, 0.0); m];
    let mut parent = vec![0; n + 1];

    for i in 0..m {
        pipes[i] = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<f64>(),
        );
    }

    pipes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    for i in 1..=n {
        parent[i] = i;
    }

    let q = scan.token::<usize>();
    let mut queries = vec![(0, 0.0); q];

    for i in 0..q {
        let p = scan.token::<f64>();
        queries[i] = (i, p);
    }

    queries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut ret = vec![0; q];
    let mut idx_pipe = 0;
    let mut num_connected = n;

    for &(idx_query, percent) in queries.iter() {
        while idx_pipe < m && pipes[idx_pipe].2 >= percent {
            if find(&mut parent, pipes[idx_pipe].0) != find(&mut parent, pipes[idx_pipe].1) {
                process_union(&mut parent, pipes[idx_pipe].0, pipes[idx_pipe].1);
                num_connected -= 1;
            }

            idx_pipe += 1;
        }

        ret[idx_query] = num_connected;
    }

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}

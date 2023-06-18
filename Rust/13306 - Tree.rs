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

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut vertices = vec![0; n + 1];

    for i in 2..=n {
        vertices[i] = scan.token::<usize>();
    }

    let mut parent = vec![0; n + 1];

    for i in 1..=n {
        parent[i] = i;
    }

    let mut queries = vec![(0, 0, 0); n + q - 1];

    for i in 0..n + q - 1 {
        let x = scan.token::<usize>();

        if x == 0 {
            let b = scan.token::<usize>();
            queries[i] = (x, b, 0);
        } else {
            let (c, d) = (scan.token::<usize>(), scan.token::<usize>());
            queries[i] = (x, c, d);
        }
    }

    let mut ret = Vec::new();

    for (x, val1, val2) in queries.iter().rev() {
        if *x == 0 {
            process_union(&mut parent, *val1, vertices[*val1]);
        } else {
            ret.push(if find(&mut parent, *val1) == find(&mut parent, *val2) {
                "YES"
            } else {
                "NO"
            });
        }
    }

    for val in ret.iter().rev() {
        writeln!(out, "{val}").unwrap();
    }
}

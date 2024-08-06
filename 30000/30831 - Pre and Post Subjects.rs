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

    let (n, a, b) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut parent = vec![0; n + 1];
    let mut relationships = vec![0; n + 1];
    let mut is_essential = vec![false; n + 1];

    for i in 1..=n {
        parent[i] = i;
    }

    for _ in 0..a {
        let (sub_a, sub_b) = (scan.token::<usize>(), scan.token::<usize>());
        relationships[sub_a] = sub_b;
    }

    for _ in 0..b {
        let (sub_c, sub_d) = (scan.token::<usize>(), scan.token::<usize>());
        relationships[sub_c] = sub_d;
        is_essential[sub_c] = true;

        process_union(&mut parent, sub_d, sub_c);
    }

    let s = scan.token::<i64>();

    for _ in 0..s {
        let e = scan.token::<usize>();
        let ret = find(&mut parent, e);

        if ret == 0 {
            writeln!(out, "-1").unwrap();
        } else {
            writeln!(out, "{ret}").unwrap();
        }

        let idx_next = relationships[ret];

        if is_essential[ret] {
            parent[idx_next] = idx_next;
        }

        process_union(&mut parent, ret, idx_next);
    }
}

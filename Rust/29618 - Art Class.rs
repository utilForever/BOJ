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

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut parent = vec![0; n + 2];
    let mut ret = vec![0; n + 2];

    for i in 1..=n + 1 {
        parent[i] = i;
    }

    for _ in 0..q {
        let (a, b, x) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        let mut idx = a;

        while idx <= b {
            if ret[idx] == 0 {
                ret[idx] = x;
                process_union(&mut parent, idx, idx + 1);
                idx += 1;
            } else {
                idx = find(&mut parent, idx);
            }
        }
    }

    for i in 1..=n {
        write!(out, "{} ", ret[i]).unwrap();
    }

    writeln!(out).unwrap();
}

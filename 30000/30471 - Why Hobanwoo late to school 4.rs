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

fn process_union(
    parent: &mut Vec<usize>,
    slimes: &mut Vec<i64>,
    ret: &mut i64,
    mut a: usize,
    mut b: usize,
) {
    a = find(parent, a);
    b = find(parent, b);

    if a == b {
        return;
    }

    *ret -= slimes[a] * (slimes[a] + 1) / 2 + slimes[b] * (slimes[b] + 1) / 2;

    parent[a] = b;
    slimes[b] += slimes[a];

    *ret += slimes[b] * (slimes[b] + 1) / 2;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut slimes = vec![0; n + 1];
    let mut parent = vec![0; n + 1];

    for i in 1..=n {
        slimes[i] = 1;
        parent[i] = i;
    }

    let mut ret = n as i64;

    for _ in 0..m {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        process_union(&mut parent, &mut slimes, &mut ret, a, b);

        writeln!(out, "{ret}").unwrap();
    }
}

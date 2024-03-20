use io::Write;
use std::{collections::HashMap, io, str};

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

fn process_union(parent: &mut Vec<usize>, size: &mut Vec<i64>, mut a: usize, mut b: usize) -> i64 {
    a = find(parent, a);
    b = find(parent, b);

    if a == b {
        return size[a];
    }

    parent[a] = b;
    size[b] += size[a];

    size[b]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let f = scan.token::<usize>();
        let mut names = HashMap::new();
        let mut parent = (0..=2 * f).collect::<Vec<_>>();
        let mut size = vec![1; 2 * f + 1];
        let mut idx = 0;

        for _ in 0..f {
            let (a, b) = (scan.token::<String>(), scan.token::<String>());
            let a = *names.entry(a).or_insert({
                idx += 1;
                idx
            });
            let b = *names.entry(b).or_insert({
                idx += 1;
                idx
            });

            writeln!(out, "{}", process_union(&mut parent, &mut size, a, b)).unwrap();
        }
    }
}

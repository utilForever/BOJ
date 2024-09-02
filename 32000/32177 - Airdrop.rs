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

    let (n, k, t) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut positions = vec![(0, 0, 0, 0); n + 2];

    let (xp, yp, vp) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    positions[1] = (xp, yp, vp, 0);

    for i in 2..=n + 1 {
        positions[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    let mut parents = vec![0; n + 2];

    for i in 1..=n + 1 {
        parents[i] = i;
    }

    for i in 1..=n {
        for j in i + 1..=n + 1 {
            let (x1, y1, v1, _) = positions[i];
            let (x2, y2, v2, _) = positions[j];

            let dist = (x1 - x2).pow(2) + (y1 - y2).pow(2);

            if dist <= k * k && (v2 - v1).abs() <= t {
                process_union(&mut parents, j, i);
            }
        }
    }

    let mut ret = Vec::new();

    for i in 2..=n + 1 {
        if find(&mut parents, i) == find(&mut parents, 1) && positions[i].3 == 1 {
            ret.push(i - 1);
        }
    }

    if ret.is_empty() {
        writeln!(out, "0").unwrap();
    } else {
        for val in ret {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}

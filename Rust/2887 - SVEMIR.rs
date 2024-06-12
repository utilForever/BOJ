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

    let n = scan.token::<usize>();
    let mut planets = vec![(0, 0, 0, 0); n];

    for i in 0..n {
        planets[i] = (
            i,
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
        );
    }

    let mut edges = Vec::new();
    let mut parent = vec![0; n];
    let mut ret = 0;

    // for i in 0..n {
    //     for j in 0..n {
    //         if i == j {
    //             continue;
    //         }

    //         let (x1, y1, z1) = planets[i];
    //         let (x2, y2, z2) = planets[j];
    //         let cost = ((x1 - x2).abs()).min((y1 - y2).abs()).min((z1 - z2).abs());

    //         edges.push((i, j, cost));
    //     }
    // }

    for idx in 0..3 {
        planets.sort_by(|a, b| match idx {
            0 => a.1.partial_cmp(&b.1).unwrap(),
            1 => a.2.partial_cmp(&b.2).unwrap(),
            2 => a.3.partial_cmp(&b.3).unwrap(),
            _ => unreachable!(),
        });

        for i in 1..n {
            let p1 = planets[i - 1];
            let p2 = planets[i];

            edges.push((
                p1.0,
                p2.0,
                match idx {
                    0 => (p1.1 - p2.1).abs(),
                    1 => (p1.2 - p2.2).abs(),
                    2 => (p1.3 - p2.3).abs(),
                    _ => unreachable!(),
                },
            ));
        }
    }

    for i in 0..n {
        parent[i] = i;
    }

    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    for i in 0..edges.len() {
        if find(&mut parent, edges[i].0) == find(&mut parent, edges[i].1) {
            continue;
        }

        process_union(&mut parent, edges[i].0, edges[i].1);
        ret += edges[i].2;
    }

    writeln!(out, "{ret}").unwrap();
}

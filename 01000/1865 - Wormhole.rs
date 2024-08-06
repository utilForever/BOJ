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

fn process_bellman_ford(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Option<Vec<i64>> {
    let n: usize = graph.len() - 1;
    let mut ret = vec![i64::MAX / 4; graph.len()];

    ret[from] = 0;

    for i in 1..=n {
        for j in 1..=n {
            for info in graph[j].iter() {
                let (vertex_next, cost_next) = *info;

                if ret[vertex_next] > ret[j] + cost_next {
                    ret[vertex_next] = ret[j] + cost_next;

                    if i == n {
                        return None;
                    }
                }
            }
        }
    }

    Some(ret)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let tc = scan.token::<i64>();

    for _ in 0..tc {
        let (n, m, w) = (
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let mut graph = vec![Vec::new(); n + 1];

        for _ in 0..m {
            let (s, e, t) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            graph[s].push((e, t));
            graph[e].push((s, t));
        }

        for _ in 0..w {
            let (s, e, t) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );
            graph[s].push((e, -t));
        }

        writeln!(
            out,
            "{}",
            match process_bellman_ford(&graph, 1) {
                Some(_) => "NO",
                None => "YES",
            }
        )
        .unwrap();
    }
}

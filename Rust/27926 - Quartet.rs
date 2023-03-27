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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut data = vec![(0, 0, 0); m];
    let mut graph = vec![Vec::new(); n + 1];

    for i in 0..m {
        let (u, v, d) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        data[i] = (u, v, d);
        graph[u].push((v, d));
        graph[v].push((u, d));
    }

    data.sort_by(|a, b| b.2.cmp(&a.2));

    for i in 1..=n {
        graph[i].sort_by(|a, b| b.1.cmp(&a.1));
    }

    let mut ret = if data.is_empty() {
        0
    } else if data.len() == 1 {
        data[0].2
    } else {
        data[0].2 + data[1].2
    };

    for i in 0..m {
        let (u, v, d) = data[i];

        for j in 0..graph[u].len().min(3) {
            for k in 0..graph[v].len().min(3) {
                if graph[u][j].0 != graph[v][k].0 && graph[u][j].0 != v && graph[v][k].0 != u {
                    ret = ret.max(d + graph[u][j].1 + graph[v][k].1);
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}

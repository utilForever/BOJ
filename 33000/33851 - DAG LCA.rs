use io::Write;
use std::{collections::VecDeque, io, str};

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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, q) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph[u].push(v);
    }

    let mut dist = vec![vec![-1; n + 1]; n + 1];

    for start in 1..=n {
        let mut queue = VecDeque::new();

        dist[start][start] = 0;
        queue.push_back(start);

        while let Some(curr) = queue.pop_front() {
            for &next in graph[curr].iter() {
                if dist[start][next] != -1 {
                    continue;
                }

                dist[start][next] = dist[start][curr] + 1;
                queue.push_back(next);
            }
        }
    }

    for _ in 0..q {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        let mut ret = i64::MAX;

        for i in 1..=n {
            if dist[i][u] == -1 || dist[i][v] == -1 {
                continue;
            }

            ret = ret.min(dist[i][u].max(dist[i][v]));
        }

        if ret == i64::MAX {
            writeln!(out, "-1").unwrap();
        } else {
            writeln!(out, "{ret}").unwrap();
        }
    }
}

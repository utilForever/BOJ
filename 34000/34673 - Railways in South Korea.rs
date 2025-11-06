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

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..n - 1 {
        let (f, t, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        graph[f].push((t, c));
        graph[t].push((f, c));
    }

    let mut populations = vec![0; n + 1];

    for i in 1..=n {
        populations[i] = scan.token::<i64>();
    }

    let mut stack = Vec::with_capacity(n);
    let mut parent = vec![0; n + 1];
    let mut dist = vec![0; n + 1];

    stack.push(1);
    parent[1] = 0;

    while let Some(node) = stack.pop() {
        for &(next, d) in graph[node].iter() {
            if next == parent[node] {
                continue;
            }

            parent[next] = node;
            dist[next] = dist[node] + d;
            stack.push(next);
        }
    }

    let mut pairs = Vec::with_capacity(n);

    for i in 1..=n {
        pairs.push((dist[i], populations[i]));
    }

    pairs.sort_unstable();

    let mut pairs_same = 0;
    let mut cnt = 0;
    let mut prev = None;

    for pair in pairs {
        match prev {
            Some(prev) if prev == pair => cnt += 1,
            Some(_) => {
                pairs_same += cnt * (cnt - 1) / 2;
                cnt = 1;
            }
            None => cnt = 1,
        }

        prev = Some(pair);
    }

    if cnt > 0 {
        pairs_same += cnt * (cnt - 1) / 2;
    }

    let ret = n * (n - 1) / 2 - pairs_same;

    writeln!(out, "{ret} {ret}").unwrap();
}

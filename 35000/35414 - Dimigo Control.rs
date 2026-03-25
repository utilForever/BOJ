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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());

        graph[u].push(v);
        graph[v].push(u);
    }

    let mut visited = vec![false; n + 1];
    let mut ret = Vec::with_capacity(n);

    for i in 1..=n {
        if visited[i] {
            continue;
        }

        visited[i] = true;

        let mut stack = Vec::new();
        let mut order = Vec::new();

        stack.push(i);
        order.reserve(1);

        while let Some(u) = stack.pop() {
            order.push(u);

            for &v in graph[u].iter() {
                if visited[v] {
                    continue;
                }

                visited[v] = true;
                stack.push(v);
            }
        }

        let root = order[0];

        for &node in order.iter().rev() {
            if node != root {
                ret.push(node);
            }
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for node in ret {
        writeln!(out, "{node}").unwrap();
    }
}

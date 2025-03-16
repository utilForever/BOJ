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
    let mut graph = vec![Vec::new(); n];

    for _ in 0..m {
        let (u, v) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        graph[u].push(v);
        graph[v].push(u);
    }

    if n != m {
        writeln!(out, "NO").unwrap();
        return;
    }

    let mut queue = VecDeque::new();
    let mut visited = vec![false; n];

    queue.push_back(0);
    visited[0] = true;

    while let Some(curr) = queue.pop_front() {
        for &next in graph[curr].iter() {
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }

    if !visited.iter().all(|&x| x) {
        writeln!(out, "NO").unwrap();
        return;
    }

    let mut degree = vec![0; n];

    for i in 0..n {
        degree[i] = graph[i].len();
    }

    let mut is_cycle = vec![true; n];
    let mut queue = VecDeque::new();

    for i in 0..n {
        if degree[i] == 1 {
            queue.push_back(i);
        }
    }

    while let Some(curr) = queue.pop_front() {
        is_cycle[curr] = false;

        for &next in graph[curr].iter() {
            if degree[next] > 0 {
                degree[next] -= 1;

                if degree[next] == 1 {
                    queue.push_back(next);
                }
            }
        }
    }

    let mut bridge = 0;

    for curr in 0..n {
        if !is_cycle[curr] {
            continue;
        }

        for &next in graph[curr].iter() {
            if !is_cycle[next] {
                bridge += 1;
            }
        }
    }

    if bridge != 1 {
        writeln!(out, "NO").unwrap();
        return;
    }

    for node in 0..n {
        if !is_cycle[node] && graph[node].len() > 2 {
            writeln!(out, "NO").unwrap();
            return;
        }
    }

    writeln!(out, "YES").unwrap();
}

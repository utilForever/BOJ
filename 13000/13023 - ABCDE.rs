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

fn process_backtrack(
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    node: usize,
    depth: usize,
    ret: &mut bool,
) {
    if depth == 5 {
        *ret = true;
        return;
    }

    visited[node] = true;

    for &next in &graph[node] {
        if visited[next] {
            continue;
        }

        process_backtrack(graph, visited, next, depth + 1, ret);
    }

    visited[node] = false;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n];

    for _ in 0..m {
        let u = scan.token::<usize>();
        let v = scan.token::<usize>();
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut visited = vec![false; n];
    let mut ret = false;

    for i in 0..n {
        process_backtrack(&graph, &mut visited, i, 1, &mut ret);
        
        if ret {
            break;
        }
    }

    writeln!(out, "{}", if ret { 1 } else { 0 }).unwrap();
}

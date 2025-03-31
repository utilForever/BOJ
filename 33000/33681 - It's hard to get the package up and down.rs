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

fn process_dfs(
    packages: &Vec<i64>,
    graph: &Vec<Vec<usize>>,
    extra: &mut i64,
    node: usize,
    parent: i64,
) -> i64 {
    let mut sum_child = 0;
    let mut max_child = 0;

    for &child in graph[node].iter() {
        if child as i64 == parent {
            continue;
        }

        let ret_child = process_dfs(packages, graph, extra, child, node as i64);

        sum_child += ret_child;
        max_child = max_child.max(ret_child);
    }

    let cost = packages[node] + sum_child;

    if parent != -1 && sum_child > 0 {
        *extra += 2 * (sum_child - max_child);
    }

    cost
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut packages = vec![0; n];

    for i in 0..n {
        packages[i] = scan.token::<i64>();
    }

    let mut graph = vec![Vec::new(); n];

    for _ in 0..n - 1 {
        let (i, j) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        graph[i].push(j);
        graph[j].push(i);
    }

    let mut ret_extra = 0;
    let ret = process_dfs(&packages, &graph, &mut ret_extra, 0, -1);
    let ret_base = 2 * (ret - packages[0]);

    writeln!(out, "{}", ret_base + ret_extra).unwrap();
}

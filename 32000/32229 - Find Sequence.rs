use io::Write;
use std::{collections::HashSet, io, str};

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
    graph: &Vec<Vec<usize>>,
    indices: &mut Vec<HashSet<usize>>,
    nums: &mut Vec<usize>,
    curr: usize,
) {
    nums.push(curr);

    for &next in graph[curr].iter() {
        if indices[curr].contains(&next) {
            continue;
        }

        indices[curr].insert(next);
        indices[next].insert(curr);

        process_dfs(graph, indices, nums, next);
    }
}

fn process_dfs_for_connected(
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    cnt: &mut usize,
    curr: usize,
) {
    visited[curr] = true;
    *cnt += 1;

    for &next in graph[curr].iter() {
        if visited[next] {
            continue;
        }

        process_dfs_for_connected(graph, visited, cnt, next);
    }
}

fn is_connected(graph: &Vec<Vec<usize>>, cnt: &mut usize) -> bool {
    let mut visited = vec![false; graph.len()];

    process_dfs_for_connected(graph, &mut visited, cnt, 1);

    *cnt != graph.len() - 1
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (a, b, n) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    let mut graph = vec![Vec::new(); n + 1];
    let mut indices = vec![HashSet::new(); n + 1];
    let mut degree = vec![0; n + 1];
    let mut ret = 1;

    for i in 1..=n {
        if i as i64 - a as i64 >= 1 {
            graph[i].push(i - a);
            graph[i - a].push(i);

            degree[i] += 1;
            degree[i - a] += 1;

            ret += 1;
        }

        if a != b && i as i64 - b as i64 >= 1 {
            graph[i].push(i - b);
            graph[i - b].push(i);

            degree[i] += 1;
            degree[i - b] += 1;

            ret += 1;
        }
    }

    let mut cnt = 0;

    if is_connected(&graph, &mut cnt) {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut idx = 0;
    let mut odd = [0, 0];

    for i in 1..=n {
        if degree[i] % 2 == 1 {
            if idx == 2 {
                writeln!(out, "-1").unwrap();
                return;
            }

            odd[idx] = i;
            idx += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();

    let mut nums = Vec::new();

    if idx == 0 {
        process_dfs(&graph, &mut indices, &mut nums, 1);
    } else {
        process_dfs(&graph, &mut indices, &mut nums, odd[0]);
    }

    for val in nums {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}

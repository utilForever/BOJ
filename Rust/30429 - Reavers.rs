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

fn process_dfs(graph: &Vec<Vec<(usize, i64)>>, visited: &mut Vec<bool>, curr: usize) -> (i64, i64) {
    let (mut min, mut max) = (0, 1);

    for &next in &graph[curr] {
        let (opinion, target) = next;

        if visited[opinion] {
            continue;
        }

        visited[opinion] = true;

        let (next_min, next_max) = process_dfs(graph, visited, opinion);

        if target == 1 {
            min += next_max;
            max += next_min;
        } else {
            min += next_min;
            max += next_max;
        }
    }

    (min, max)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];

    for i in 1..=n {
        let (opinion, target) = (scan.token::<usize>(), scan.token::<i64>());

        graph[i].push((opinion, target));
        graph[opinion].push((i, target));
    }

    let mut visited = vec![false; n + 1];
    let mut ret = 0;

    for i in 1..=n {
        if visited[i] {
            continue;
        }

        visited[i] = true;

        let (min, max) = process_dfs(&graph, &mut visited, i);
        ret += min.min(max);
    }

    writeln!(out, "{ret}").unwrap();
}

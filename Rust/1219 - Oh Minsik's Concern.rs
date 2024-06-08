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
}

fn process_bellman_ford(
    graph: &Vec<Vec<(usize, i64)>>,
    reward: &Vec<i64>,
    from: usize,
) -> (Vec<i64>, Vec<bool>) {
    let n: usize = graph.len();
    let mut has_cycle = vec![false; graph.len()];
    let mut ret = vec![i64::MIN / 4; graph.len()];

    ret[from] = reward[from];

    for i in 0..n {
        for j in 0..n {
            for info in graph[j].iter() {
                let (vertex_next, mut cost_next) = *info;
                cost_next += ret[j] + reward[vertex_next];

                if ret[j] != i64::MIN / 4 && ret[vertex_next] < cost_next {
                    ret[vertex_next] = cost_next;

                    if i == n - 1 {
                        has_cycle[j] = true;
                    }
                }
            }
        }
    }

    (ret, has_cycle)
}

fn process_bfs(graph: &Vec<Vec<(usize, i64)>>, has_cycle: &Vec<bool>, to: usize) -> bool {
    let n = graph.len();
    let mut queue = VecDeque::new();
    let mut visited = vec![false; n];

    for i in 0..n {
        if has_cycle[i] {
            queue.push_back(i);
            visited[i] = true;
        }
    }

    while !queue.is_empty() {
        let vertex_curr = queue.pop_front().unwrap();

        if vertex_curr == to {
            return true;
        }

        for info in graph[vertex_curr].iter() {
            let (vertex_next, _) = *info;

            if !visited[vertex_next] {
                visited[vertex_next] = true;
                queue.push_back(vertex_next);
            }
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, a, b, m) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut graph = vec![Vec::new(); n];
    let mut reward = vec![0; n];

    for _ in 0..m {
        let (s, e, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        graph[s].push((e, -c));
    }

    for i in 0..n {
        reward[i] = scan.token::<i64>();
    }

    let (costs_min, has_cycle) = process_bellman_ford(&graph, &reward, a);
    let is_infinite = process_bfs(&graph, &has_cycle, b);

    if costs_min[b] == i64::MIN / 4 {
        writeln!(out, "gg").unwrap();
    } else if is_infinite {
        writeln!(out, "Gee").unwrap();
    } else {
        writeln!(out, "{}", costs_min[b]).unwrap();
    }
}

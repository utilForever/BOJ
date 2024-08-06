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

fn calculate_prefix_sum(
    capacities: &Vec<i64>,
    graph: &Vec<Vec<usize>>,
    prefix_sum: &mut Vec<i64>,
    curr: usize,
    parent: usize,
) {
    prefix_sum[curr] = capacities[curr];

    for &next in graph[curr].iter() {
        if next == parent {
            continue;
        }

        calculate_prefix_sum(capacities, graph, prefix_sum, next, curr);
        prefix_sum[curr] += prefix_sum[next];
    }
}

fn calculate_cost(x: i64, y: i64, cost: i64) -> i64 {
    if x.min(y) + cost >= x.max(y) {
        x + y + cost
    } else if x > y {
        (y + cost) * 2
    } else {
        (x + cost) * 2
    }
}

fn process_tree_dp(
    graph: &Vec<Vec<usize>>,
    prefix_sum: &Vec<i64>,
    ret: &mut Vec<i64>,
    curr: usize,
    parent: usize,
    cost: i64,
) {
    if parent == 0 {
        ret[curr] = prefix_sum[curr];

        for &next in graph[curr].iter() {
            process_tree_dp(
                graph,
                prefix_sum,
                ret,
                next,
                curr,
                prefix_sum[curr] - prefix_sum[next],
            );
        }

        return;
    }

    let mut x = 0;
    let mut y = 0;

    for &next in graph[curr].iter() {
        if next == parent {
            break;
        }

        x += prefix_sum[next];
    }

    for &next in graph[curr].iter().rev() {
        if next == parent {
            break;
        }

        y += prefix_sum[next];
    }

    ret[curr] = calculate_cost(x, y, cost);

    for &next in graph[curr].iter() {
        if next == parent {
            continue;
        }

        if next < parent {
            process_tree_dp(
                graph,
                prefix_sum,
                ret,
                next,
                curr,
                calculate_cost(x - prefix_sum[next], y, cost),
            );
        } else {
            process_tree_dp(
                graph,
                prefix_sum,
                ret,
                next,
                curr,
                calculate_cost(x, y - prefix_sum[next], cost),
            );
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut capacities = vec![0; n + 1];
    let mut graph = vec![Vec::new(); n + 1];
    let mut prefix_sum = vec![0; n + 1];

    for i in 1..=n {
        capacities[i] = scan.token::<i64>();
    }

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    for i in 1..=n {
        graph[i].sort();
    }

    calculate_prefix_sum(&capacities, &graph, &mut prefix_sum, m, 0);

    let mut ret = vec![0; n + 1];

    process_tree_dp(&graph, &prefix_sum, &mut ret, m, 0, 0);

    for i in 1..=n {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}

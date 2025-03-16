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

fn combine(childs: &Vec<i64>, base: i64) -> i64 {
    if childs.is_empty() {
        base
    } else {
        base.max(childs.iter().copied().max().unwrap() + 1)
    }
}

fn process_dfs1(
    graph: &Vec<Vec<usize>>,
    durabilities: &Vec<i64>,
    dp: &mut Vec<i64>,
    parent: Option<usize>,
    node: usize,
) {
    let mut childs = Vec::new();

    for &next in graph[node].iter() {
        if Some(next) == parent {
            continue;
        }

        process_dfs1(graph, durabilities, dp, Some(node), next);
        childs.push(dp[next]);
    }

    dp[node] = combine(&childs, durabilities[node]);
}

fn process_dfs2(
    graph: &Vec<Vec<usize>>,
    durabilities: &Vec<i64>,
    dp: &Vec<i64>,
    ret: &mut i64,
    parent: Option<usize>,
    node: usize,
    base: i64,
) {
    let mut childs = Vec::new();

    if parent.is_some() {
        childs.push(base);
    }

    for &next in graph[node].iter() {
        if Some(next) == parent {
            continue;
        }

        childs.push(dp[next]);
    }

    let total = if childs.is_empty() {
        durabilities[node]
    } else {
        durabilities[node].max(childs.iter().copied().max().unwrap() + 1)
    };

    *ret = (*ret).min(total);

    let mut childs_adj = Vec::new();

    if let Some(p) = parent {
        childs_adj.push((p, base));
    }

    for &next in &graph[node] {
        if Some(next) == parent {
            continue;
        }

        childs_adj.push((next, dp[next]));
    }

    if childs_adj.is_empty() {
        return;
    }

    let mut total_max = i64::MIN;
    let mut count_max = 0;
    let mut second_max = i64::MIN;

    for &(_, value) in &childs_adj {
        if value > total_max {
            second_max = total_max;
            total_max = value;
            count_max = 1;
        } else if value == total_max {
            count_max += 1;
        } else if value > second_max {
            second_max = value;
        }
    }

    for &(neighbor, value) in &childs_adj {
        if parent.is_some() && neighbor == parent.unwrap() {
            continue;
        }

        let max_new = if childs_adj.len() == 1 {
            None
        } else if value == total_max && count_max == 1 {
            Some(second_max)
        } else {
            Some(total_max)
        };

        let base_new = match max_new {
            Some(m) => durabilities[node].max(m + 1),
            None => durabilities[node],
        };

        process_dfs2(graph, durabilities, dp, ret, Some(node), neighbor, base_new);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n];

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut durabilities = vec![0; n];

    for i in 0..n {
        durabilities[i] = scan.token::<i64>();
    }

    let mut dp = vec![0; n];

    process_dfs1(&graph, &durabilities, &mut dp, None, 0);

    let mut ret = i64::MAX;

    process_dfs2(&graph, &durabilities, &dp, &mut ret, None, 0, 0);

    writeln!(out, "{ret}").unwrap();
}

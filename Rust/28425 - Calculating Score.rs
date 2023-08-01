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
}

fn process_dfs1(
    graph: &Vec<Vec<usize>>,
    scores: &Vec<i64>,
    score_ancestor: &mut Vec<i64>,
    score_subtree: &mut Vec<i64>,
    curr: usize,
    prev: usize,
) {
    score_subtree[curr] = scores[curr];

    for vertex in graph[curr].iter() {
        if *vertex == prev {
            continue;
        }

        score_ancestor[*vertex] = scores[curr] + score_ancestor[curr];
        process_dfs1(graph, scores, score_ancestor, score_subtree, *vertex, curr);
        score_subtree[curr] += score_subtree[*vertex];
    }
}

fn process_dfs2(
    graph: &Vec<Vec<usize>>,
    queries: &Vec<Vec<(usize, usize)>>,
    score_total: i64,
    score_ancestor: &Vec<i64>,
    score_subtree: &Vec<i64>,
    histories: &mut Vec<Vec<(usize, usize)>>,
    traces: &mut Vec<Vec<usize>>,
    ret: &mut Vec<i64>,
    curr: usize,
    prev: usize,
) {
    let score = score_ancestor[curr] + score_subtree[curr];
    let mut set = HashSet::new();

    for (r, i) in queries[curr].iter() {
        histories[*r].push((curr, *i));
        set.insert(*i);
    }

    for (r, i) in histories[curr].iter() {
        traces[*r].push(*i);
    }

    histories[curr].clear();

    for vertex in graph[curr].iter() {
        if *vertex == prev {
            continue;
        }

        process_dfs2(
            graph,
            queries,
            score_total,
            score_ancestor,
            score_subtree,
            histories,
            traces,
            ret,
            *vertex,
            curr,
        );

        for index in traces[curr].iter() {
            ret[*index] = score_total - score_subtree[*vertex];
            set.remove(index);
        }

        traces[curr].clear();
    }

    for index in set.iter() {
        ret[*index] = score - score_ancestor[curr];
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut scores = vec![0; n + 1];
    let mut graph = vec![Vec::new(); n + 1];
    let mut queries = vec![Vec::new(); n + 1];
    let mut ret = vec![0; q];

    for i in 1..=n {
        scores[i] = scan.token::<i64>();
    }

    let score_total = scores.iter().sum::<i64>();

    for _ in 1..n {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        graph[a].push(b);
        graph[b].push(a);
    }

    for i in 0..q {
        let (r, v) = (scan.token::<usize>(), scan.token::<usize>());

        if r == v {
            ret[i] = score_total;
        } else {
            queries[v].push((r, i));
        }
    }

    let mut score_ancestor = vec![0; n + 1];
    let mut score_subtree = vec![0; n + 1];

    process_dfs1(
        &graph,
        &scores,
        &mut score_ancestor,
        &mut score_subtree,
        1,
        1,
    );

    let mut histories = vec![Vec::new(); n + 1];
    let mut traces = vec![Vec::new(); n + 1];

    process_dfs2(
        &graph,
        &queries,
        score_total,
        &score_ancestor,
        &score_subtree,
        &mut histories,
        &mut traces,
        &mut ret,
        1,
        1,
    );

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}

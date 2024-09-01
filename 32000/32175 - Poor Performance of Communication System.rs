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

fn process_backtracking_day(
    pos_base: &(i64, i64),
    pos_nodes: &Vec<(i64, i64)>,
    visited: &mut Vec<usize>,
    ret: &mut i64,
    n: i64,
    k: usize,
    idx: usize,
    depth: usize,
) {
    if idx > pos_nodes.len() {
        return;
    }

    if depth == k {
        let mut visited_reverse = Vec::new();

        for i in 0..pos_nodes.len() {
            if !visited.contains(&i) {
                visited_reverse.push(i);
            }
        }

        let mut p = 0;
        let mut pos_min = (i64::MAX, i64::MAX);
        let mut pos_max = (i64::MIN, i64::MIN);

        for i in 0..visited_reverse.len() {
            p += (pos_base.0 - pos_nodes[visited_reverse[i]].0).abs()
                + (pos_base.1 - pos_nodes[visited_reverse[i]].1).abs();

            pos_min.0 = pos_min.0.min(pos_nodes[visited_reverse[i]].0);
            pos_min.1 = pos_min.1.min(pos_nodes[visited_reverse[i]].1);
            pos_max.0 = pos_max.0.max(pos_nodes[visited_reverse[i]].0);
            pos_max.1 = pos_max.1.max(pos_nodes[visited_reverse[i]].1);
        }

        let u = (pos_max.0 - pos_min.0 + 1) * (pos_max.1 - pos_min.1 + 1);

        *ret = (*ret).max((p - u).max(0));
        return;
    }

    visited.push(idx);

    process_backtracking_day(pos_base, pos_nodes, visited, ret, n, k, idx + 1, depth + 1);

    visited.pop();

    process_backtracking_day(pos_base, pos_nodes, visited, ret, n, k, idx + 1, depth);
}

fn process_backtracking_night(
    pos_base: &(i64, i64),
    pos_nodes: &Vec<(i64, i64)>,
    visited: &mut Vec<usize>,
    ret: &mut i64,
    k: usize,
    idx: usize,
    depth: usize,
) {
    if idx > pos_nodes.len() {
        return;
    }

    if depth == k {
        let mut p = 0;
        let mut pos_min = (i64::MAX, i64::MAX);
        let mut pos_max = (i64::MIN, i64::MIN);

        for i in 0..visited.len() {
            p += (pos_base.0 - pos_nodes[visited[i]].0).abs()
                + (pos_base.1 - pos_nodes[visited[i]].1).abs();

            pos_min.0 = pos_min.0.min(pos_nodes[visited[i]].0);
            pos_min.1 = pos_min.1.min(pos_nodes[visited[i]].1);
            pos_max.0 = pos_max.0.max(pos_nodes[visited[i]].0);
            pos_max.1 = pos_max.1.max(pos_nodes[visited[i]].1);
        }

        let u = (pos_max.0 - pos_min.0 + 1) * (pos_max.1 - pos_min.1 + 1);

        *ret = (*ret).max((p - u).max(0));
        return;
    }

    visited.push(idx);

    process_backtracking_night(pos_base, pos_nodes, visited, ret, k, idx + 1, depth + 1);

    visited.pop();

    process_backtracking_night(pos_base, pos_nodes, visited, ret, k, idx + 1, depth);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k1, k2) = (
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut pos_base = (0, 0);
    let mut pos_nodes = Vec::with_capacity(m);

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            if c == 'B' {
                pos_base = (i, j as i64);
            } else if c == 'N' {
                pos_nodes.push((i, j as i64));
            }
        }
    }

    let mut ret_day = 0;
    let mut ret_night = 0;

    let mut visited = Vec::new();

    process_backtracking_day(
        &pos_base,
        &pos_nodes,
        &mut visited,
        &mut ret_day,
        n,
        m - k1,
        0,
        0,
    );

    visited.clear();

    process_backtracking_night(
        &pos_base,
        &pos_nodes,
        &mut visited,
        &mut ret_night,
        k2,
        0,
        0,
    );

    writeln!(out, "{ret_day}").unwrap();
    writeln!(out, "{ret_night}").unwrap();
}

use io::Write;
use std::{collections::BinaryHeap, io, str};

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

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Vec<i64> {
    let mut ret = vec![i64::MAX; graph.len()];
    ret[from] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, from));

    while !queue.is_empty() {
        let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
        cost_curr *= -1;

        if cost_curr > ret[vertex_curr] {
            continue;
        }

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;
            cost_next += cost_curr;

            if ret[vertex_next] > cost_next {
                ret[vertex_next] = cost_next;
                queue.push((-cost_next, vertex_next));
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n];

    for _ in 0..m {
        let (u, v, w) = (
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<i64>(),
        );

        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let dist1 = process_dijkstra(&graph, 0);
    let distn = process_dijkstra(&graph, n - 1);
    let mut points = vec![(0, 0, 0); n];

    for i in 0..n {
        points[i] = (dist1[i], distn[i], i + 1);
    }

    points.sort_unstable_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut filtered = Vec::new();
    let mut y_max = i64::MIN;

    for &(x, y, idx) in points.iter().rev() {
        if y > y_max {
            filtered.push((x, y, idx));
            y_max = y;
        }
    }

    filtered.reverse();

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
        let mut left = 0;
        let mut right = filtered.len();

        while left < right {
            let mid = (left + right) / 2;

            if a * filtered[mid].0 <= b * filtered[mid].1 {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        let mut ret_idx = 1;
        let mut ret_val = 0;

        if left >= 1 {
            ret_idx = filtered[left - 1].2;
            ret_val = a * filtered[left - 1].0;
        }

        if left < filtered.len() {
            let val = b * filtered[left].1;

            if val > ret_val {
                ret_idx = filtered[left].2;
            }
        }

        writeln!(out, "{ret_idx}").unwrap();
    }
}

use io::Write;
use std::{cmp::Ordering, collections::BinaryHeap, io, str};

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

#[derive(PartialEq)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn process_dijkstra(graph: &Vec<Vec<(usize, f64)>>, from: usize) -> Vec<f64> {
    let mut ret = vec![f64::MAX / 4.0; graph.len()];
    ret[from] = 0.0;

    let mut queue = BinaryHeap::new();
    queue.push((MinNonNan(0.0), from));

    while !queue.is_empty() {
        let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
        cost_curr.0 *= -1.0;

        if ret[vertex_curr] < cost_curr.0 {
            continue;
        }

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;

            cost_next += cost_curr.0;

            if ret[vertex_next] > cost_next {
                ret[vertex_next] = cost_next;
                queue.push((MinNonNan(-cost_next), vertex_next));
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, w) = (scan.token::<usize>(), scan.token::<usize>());
    let m = scan.token::<f64>();
    let mut powerpoles = vec![(0, 0); n + 1];
    let mut graph = vec![Vec::new(); n + 1];

    for i in 1..=n {
        powerpoles[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    for _ in 0..w {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        graph[a].push((b, 0.0));
        graph[b].push((a, 0.0));
    }

    for i in 1..n {
        for j in i + 1..=n {
            let dist_x = (powerpoles[i].0 - powerpoles[j].0).abs();
            let dist_y = (powerpoles[i].1 - powerpoles[j].1).abs();
            let dist = ((dist_x * dist_x + dist_y * dist_y) as f64).sqrt();

            if dist <= m {
                graph[i].push((j, dist));
                graph[j].push((i, dist));
            }
        }
    }

    let ret = process_dijkstra(&graph, 1);

    writeln!(
        out,
        "{}",
        if ret[n] == f64::MAX / 4.0 {
            -1
        } else {
            (ret[n] * 1000.0) as i64
        }
    )
    .unwrap();
}

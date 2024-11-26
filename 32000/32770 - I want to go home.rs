use io::Write;
use std::{
    collections::{BinaryHeap, HashMap},
    io, str,
};

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

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Vec<i64> {
    let mut ret = vec![i64::MAX; graph.len()];
    ret[from] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, from));

    while !queue.is_empty() {
        let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
        cost_curr *= -1;
        
        if ret[vertex_curr] < cost_curr {
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

    let e = scan.token::<usize>();
    let mut edges = Vec::with_capacity(e);
    let mut vertices = HashMap::new();
    let mut idx = 0;

    for _ in 0..e {
        let (a, b, c) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<i64>(),
        );

        if !vertices.contains_key(&a) {
            vertices.insert(a.clone(), idx);
            idx += 1;
        }
        if !vertices.contains_key(&b) {
            vertices.insert(b.clone(), idx);
            idx += 1;
        }

        let idx_a = vertices[&a];
        let idx_b = vertices[&b];

        edges.push((idx_a, idx_b, c));
    }

    let mut graph = vec![Vec::new(); vertices.len()];

    for (idx_a, idx_b, c) in edges {
        graph[idx_a].push((idx_b, c));
    }

    let idx_school = *vertices.get("sasa").unwrap();
    let idx_home = *vertices.get("home").unwrap();

    let dist1 = process_dijkstra(&graph, idx_school);
    let dist2 = process_dijkstra(&graph, idx_home);

    if dist1[idx_home] == i64::MAX || dist2[idx_school] == i64::MAX {
        writeln!(out, "-1").unwrap();
        return;
    }

    writeln!(out, "{}", dist1[idx_home] + dist2[idx_school]).unwrap();
}

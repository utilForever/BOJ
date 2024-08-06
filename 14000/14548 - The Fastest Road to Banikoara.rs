use io::Write;
use std::{
    collections::{BinaryHeap, HashSet},
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
    let mut ret = vec![i64::MAX / 4; graph.len()];
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

    let p = scan.token::<i64>();

    for _ in 0..p {
        let (n, s, e) = (
            scan.token::<usize>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );
        let mut info = vec![(String::new(), String::new(), 0); n];
        let mut set = HashSet::new();

        for i in 0..n {
            let (a, b, c) = (
                scan.token::<String>(),
                scan.token::<String>(),
                scan.token::<i64>(),
            );

            set.insert(a.clone());
            set.insert(b.clone());
            info[i] = (a, b, c);
        }

        let names = set.into_iter().collect::<Vec<_>>();
        let mut graph = vec![Vec::new(); n + 1];
        let s = names.iter().position(|x| x == &s).unwrap() + 1;
        let e = names.iter().position(|x| x == &e).unwrap() + 1;

        for (a, b, c) in info.iter() {
            let a = names.iter().position(|x| x == a).unwrap() + 1;
            let b = names.iter().position(|x| x == b).unwrap() + 1;

            graph[a].push((b, *c));
            graph[b].push((a, *c));
        }

        let ret = process_dijkstra(&graph, s);

        writeln!(out, "{} {} {}", names[s - 1], names[e - 1], ret[e]).unwrap();
    }
}

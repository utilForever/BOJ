use io::Write;
use std::{
    collections::{BinaryHeap, HashMap, BTreeSet},
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

fn process_dijkstra(vertices: &mut [i64], vertex_info: &[Vec<(usize, i64)>], from: usize) {
    vertices.fill(i64::MAX);
    vertices[from] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, from));

    while !queue.is_empty() {
        let (mut cost, vertex) = queue.pop().unwrap();
        cost *= -1;

        if vertices[vertex] < cost {
            continue;
        }

        for info in vertex_info[vertex].iter() {
            let (next_vertex, mut next_cost) = *info;
            next_cost += cost;

            if vertices[next_vertex] > next_cost {
                vertices[next_vertex] = next_cost;
                queue.push((-next_cost, next_vertex));
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, h, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut stations = vec![String::new(); n];
    let mut nodes = vec![(0, ' '); m];
    let mut node_to_idx = HashMap::new();

    for i in 0..n {
        stations[i] = scan.token::<String>();
    }

    for i in 0..m {
        nodes[i] = (scan.token::<usize>(), scan.token::<char>());
        node_to_idx.insert(nodes[i].0, i);
    }

    let mut graph = vec![Vec::new(); 301];

    let k = scan.token::<i64>();

    for _ in 0..k {
        let (mut node1, mut node2, r) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        node1 = node_to_idx[&node1];
        node2 = node_to_idx[&node2];

        graph[node1].push((node2, r));
        graph[node2].push((node1, r));
    }

    let mut vertices = vec![0; 301];
    let mut indexes = vec![(0, 0); 301];
    let mut bucket = vec![0; 301];

    for i in 0..m {
        if nodes[i].1 != 'B' {
            continue;
        }

        process_dijkstra(&mut vertices, &graph, i);

        for j in 0..m {
            if nodes[j].1 == 'C' {
                bucket[j] = vertices[j];
            }
        }

        break;
    }

    for i in 0..m {
        if nodes[i].1 != 'R' {
            continue;
        }

        process_dijkstra(&mut vertices, &graph, i);
        indexes[i] = (i64::MAX, usize::MAX);

        for j in 0..m {
            if nodes[j].1 == 'C' {
                indexes[i] = indexes[i].min((vertices[j], nodes[j].0));
            }
        }
    }

    let mut map = vec![HashMap::new(); m + 1];
    let mut set = vec![BTreeSet::new(); m + 1];
    let mut ret = 0;

    for _ in 0..q {
        let (mut node, station) = (scan.token::<usize>(), scan.token::<String>());
        node = node_to_idx[&node];

        let to = node_to_idx[&indexes[node].1];
        let cost = indexes[node].0;

        let map_local = &mut map[to];
        let set_local = &mut set[to];

        if map_local.contains_key(&station) {
            writeln!(out, "{}", 2 * cost).unwrap();

            set_local.remove(&(map_local[&station], station.clone()));
            set_local.insert((ret, station.clone()));
            *map_local.entry(station.clone()).or_insert(0) = ret;
        } else {
            if map_local.len() == h {
                let val = set_local.iter().next().unwrap().clone();
                map_local.remove(&val.1);
                set_local.remove(&val);
            }

            *map_local.entry(station.clone()).or_insert(0) = ret;
            set_local.insert((ret, station.clone()));

            writeln!(out, "{}", 2 * (cost + bucket[to])).unwrap();
        }

        ret += 1;
    }
}

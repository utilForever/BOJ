use io::Write;
use std::{cmp::Reverse, collections::BinaryHeap, io, str};

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

const INF: i64 = i64::MAX / 4;

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> (Vec<i64>, Vec<usize>) {
    let mut dist = vec![INF; graph.len()];
    let mut parent = vec![0; graph.len()];

    dist[from] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), from));

    while let Some((Reverse(cost_curr), vertex_curr)) = queue.pop() {
        if dist[vertex_curr] != cost_curr {
            continue;
        }

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;
            cost_next += cost_curr;

            if dist[vertex_next] > cost_next {
                dist[vertex_next] = cost_next;
                parent[vertex_next] = vertex_curr;
                queue.push((Reverse(cost_next), vertex_next));
            }
        }
    }

    (dist, parent)
}

#[derive(Default)]
struct RadixHeap {
    last: i64,
    buckets: Vec<Vec<(i64, usize)>>,
    size: usize,
}

impl RadixHeap {
    fn new() -> Self {
        Self {
            last: 0,
            buckets: vec![Vec::new(); 65],
            size: 0,
        }
    }

    #[inline]
    fn push(&mut self, key: i64, val: usize) {
        let b = if key == self.last {
            0
        } else {
            64 - ((key ^ self.last).leading_zeros() as usize)
        };

        self.buckets[b].push((key, val));
        self.size += 1;
    }

    #[inline]
    fn pop(&mut self) -> Option<(i64, usize)> {
        if self.buckets[0].is_empty() {
            let mut i = 1;

            while i < 65 && self.buckets[i].is_empty() {
                i += 1;
            }

            if i == 65 {
                return None;
            }

            let mut last_new = i64::MAX;

            for &(k, _) in &self.buckets[i] {
                if k < last_new {
                    last_new = k;
                }
            }

            self.last = last_new;

            let mut temp = Vec::new();
            std::mem::swap(&mut temp, &mut self.buckets[i]);

            for (k, v) in temp {
                let b = if k == self.last {
                    0
                } else {
                    64 - ((k ^ self.last).leading_zeros() as usize)
                };

                self.buckets[b].push((k, v));
            }
        }

        self.size -= 1;
        self.buckets[0].pop()
    }
}

fn duel_components(
    tree: &Vec<Vec<usize>>,
    is_alive: &Vec<bool>,
    mark: &mut Vec<u32>,
    epoch: &mut u32,
    buffer: &mut Vec<usize>,
    a: usize,
    b: usize,
    block: usize,
) -> (usize, u32) {
    *epoch += 1;
    let epoch_a = *epoch;

    *epoch += 1;
    let epoch_b = *epoch;

    let mut stack_a = Vec::with_capacity(64);
    let mut stack_b = Vec::with_capacity(64);
    let mut idx_a = 0;
    let mut idx_b = 0;

    stack_a.push(a);
    mark[a] = epoch_a;

    stack_b.push(b);
    mark[b] = epoch_b;

    buffer.clear();

    loop {
        if idx_a < stack_a.len() {
            let x = stack_a[idx_a];

            idx_a += 1;
            buffer.push(x);

            for &w in tree[x].iter() {
                if w == block || !is_alive[w] {
                    continue;
                }

                if mark[w] == epoch_a || mark[w] == epoch_b {
                    continue;
                }

                mark[w] = epoch_a;
                stack_a.push(w);
            }
        } else {
            return (b, epoch_a);
        }

        if idx_b < stack_b.len() {
            let x = stack_b[idx_b];
            idx_b += 1;

            for &w in tree[x].iter() {
                if w == block || !is_alive[w] {
                    continue;
                }

                if mark[w] == epoch_b || mark[w] == epoch_a {
                    continue;
                }

                mark[w] = epoch_b;
                stack_b.push(w);
            }
        } else {
            buffer.clear();
            *epoch += 1;

            let mut stack = vec![b];
            let mut idx = 0;

            mark[b] = *epoch;

            while idx < stack.len() {
                let x = stack[idx];

                idx += 1;
                buffer.push(x);

                for &w in tree[x].iter() {
                    if w == block || !is_alive[w] {
                        continue;
                    }

                    if mark[w] == *epoch {
                        continue;
                    }

                    mark[w] = *epoch;
                    stack.push(w);
                }
            }

            return (a, *epoch);
        }
    }
}

fn relax(
    components: &Vec<usize>,
    is_alive: &Vec<bool>,
    mark: &Vec<u32>,
    extra_out: &Vec<Vec<(usize, i64)>>,
    extra_in: &Vec<Vec<(usize, i64)>>,
    heap: &mut RadixHeap,
    dist: &mut Vec<i64>,
    epoch: u32,
    dist_curr: i64,
) {
    for &x in components {
        for &(y, c) in extra_out[x].iter() {
            if !is_alive[y] {
                continue;
            }

            if mark[y] == epoch {
                continue;
            }

            let dist_new = dist_curr + c;

            if dist_new < dist[y] {
                dist[y] = dist_new;
                heap.push(dist_new, y);
            }
        }
    }

    for &x in components {
        for &(y, w) in extra_in[x].iter() {
            if !is_alive[y] {
                continue;
            }

            if mark[y] == epoch {
                continue;
            }

            let dist_new = dist_curr + w;

            if dist_new < dist[x] {
                dist[x] = dist_new;
                heap.push(dist_new, x);
            }
        }
    }
}

// Reference: XX Open Cup, Grand Prix of Wroclaw Editorial
// Reference: https://www.eecs.yorku.ca/course_archive/2007-08/F/6590/Notes/surballe_alg.pdf
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let z = scan.token::<i64>();

    for _ in 0..z {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let mut roads = vec![(0, 0, 0); m];
        let mut graph = vec![Vec::new(); n + 1];

        for i in 0..m {
            let (a, b, c) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );

            roads[i] = (a, b, c);
            graph[a].push((b, c));
        }

        // Find arbitrary shortest path tree D using Dijkstra's algorithm
        let (potential, parent) = process_dijkstra(&graph, 1);
        let mut tree = vec![Vec::new(); n + 1];

        for i in 2..=n {
            if parent[i] == 0 {
                continue;
            }

            tree[parent[i]].push(i);
            tree[i].push(parent[i]);
        }

        // Modify non-tree edges using potential function
        let mut extra_out = vec![Vec::<(usize, i64)>::new(); n + 1];
        let mut extra_in = vec![Vec::<(usize, i64)>::new(); n + 1];

        for &(a, b, c) in roads.iter() {
            if parent[b] == a && potential[b] == potential[a] + c {
                continue;
            }

            let cost_new = c + potential[a] - potential[b];
            extra_out[a].push((b, cost_new));
            extra_in[b].push((a, cost_new));
        }

        let mut dist = vec![INF; n + 1];
        let mut visited = vec![false; n + 1];
        let mut is_alive = vec![true; n + 1];
        let mut heap = RadixHeap::new();

        dist[1] = 0;
        heap.push(0, 1);

        let mut mark = vec![0; n + 1];
        let mut epoch = 1;
        let mut buffer = Vec::new();
        let mut seeds = Vec::new();

        while let Some((d, u)) = heap.pop() {
            if d != dist[u] || visited[u] {
                continue;
            }

            visited[u] = true;

            for &(v, c) in extra_out[u].iter() {
                if !is_alive[v] {
                    continue;
                }

                let dist_new = d + c;

                if dist_new < dist[v] {
                    dist[v] = dist_new;
                    heap.push(dist_new, v);
                }
            }

            if is_alive[u] {
                is_alive[u] = false;
            } else {
                continue;
            }

            seeds.clear();

            for &v in tree[u].iter() {
                if is_alive[v] {
                    seeds.push(v);
                }
            }

            if seeds.is_empty() {
                continue;
            }

            let mut champion = seeds[0];

            for &candidate in seeds.iter().skip(1) {
                let (winner, epoch_small) = duel_components(
                    &tree,
                    &is_alive,
                    &mut mark,
                    &mut epoch,
                    &mut buffer,
                    champion,
                    candidate,
                    u,
                );
                relax(
                    &buffer,
                    &is_alive,
                    &mark,
                    &extra_out,
                    &extra_in,
                    &mut heap,
                    &mut dist,
                    epoch_small,
                    d,
                );
                champion = winner;
            }
        }

        for i in 2..=n {
            if dist[i] >= INF {
                write!(out, "-1 ").unwrap();
            } else {
                write!(out, "{} ", 2 * potential[i] + dist[i]).unwrap();
            }
        }

        writeln!(out).unwrap();
    }
}

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut roads = vec![0; m];
    let mut villages = vec![0; m];

    for i in 0..m {
        roads[i] = scan.token::<usize>();
    }

    for i in 0..m {
        villages[i] = scan.token::<usize>();
    }

    let mut city_start = vec![0; m];
    let mut city_of = vec![0; n + 1];
    let mut idx_start = 1;

    for i in 0..m {
        city_start[i] = idx_start;

        for idx in idx_start..idx_start + villages[i] {
            city_of[idx] = i;
        }

        idx_start += villages[i];
    }

    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..n - m {
        let (_, u, v) = (
            scan.token::<usize>() - 1,
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut node_centroid = vec![0; m];
    let mut size = vec![0; n + 1];
    let mut sum_internal = 0;

    for i in 0..m {
        let village = villages[i];

        if village == 1 {
            node_centroid[i] = city_start[i];
            continue;
        }

        let root = city_start[i];
        let mut order = Vec::with_capacity(village);
        let mut stack = Vec::with_capacity(village);

        stack.push((root, 0));

        while let Some((node, parent)) = stack.pop() {
            order.push(node);

            for &next in graph[node].iter() {
                if next != parent {
                    stack.push((next, node));
                }
            }
        }

        let mut centroid_best = root;
        let mut size_best = village;

        for &node in order.iter().rev() {
            size[node] = 1;

            let mut size_max = 0;

            for &next in graph[node].iter() {
                if size[next] == 0 {
                    continue;
                }

                size[node] += size[next];
                size_max = size_max.max(size[next]);
            }

            size_max = size_max.max(village - size[node]);

            if size_max < size_best {
                size_best = size_max;
                centroid_best = node;
            }
        }

        node_centroid[i] = centroid_best;

        let mut stack = Vec::with_capacity(village);
        stack.push((centroid_best, 0, false));

        while let Some((node, parent, done)) = stack.pop() {
            if done {
                size[node] = 1;

                for &next in graph[node].iter() {
                    if next != parent {
                        size[node] += size[next];
                    }
                }

                if parent != 0 {
                    sum_internal += size[node] * (n - size[node]);
                }
            } else {
                stack.push((node, parent, true));

                for &next in graph[node].iter() {
                    if next != parent {
                        stack.push((next, node, false));
                    }
                }
            }
        }
    }

    let mut sequence = Vec::with_capacity(m - 2);
    let mut heavy = (0..m)
        .filter(|&idx| roads[idx] > 1)
        .map(|idx| (villages[idx], idx))
        .collect::<Vec<_>>();

    heavy.sort_unstable();

    for &(_, idx) in heavy.iter() {
        for _ in 0..roads[idx] - 1 {
            sequence.push(idx);
        }
    }

    let mut heap = BinaryHeap::<Reverse<(usize, usize)>>::new();

    for i in 0..m {
        if roads[i] == 1 {
            heap.push(Reverse((villages[i], i)));
        }
    }

    let mut graph = vec![Vec::new(); m];

    for &idx in sequence.iter() {
        let Reverse((_, leaf)) = heap.pop().unwrap();

        graph[leaf].push(idx);
        graph[idx].push(leaf);

        roads[leaf] -= 1;
        roads[idx] -= 1;

        if roads[idx] == 1 {
            heap.push(Reverse((villages[idx], idx)));
        }
    }

    let Reverse((_, leaf_a)) = heap.pop().unwrap();
    let Reverse((_, leaf_b)) = heap.pop().unwrap();

    graph[leaf_a].push(leaf_b);
    graph[leaf_b].push(leaf_a);

    let mut size = vec![0; m];
    let mut order = Vec::with_capacity(m);
    let mut stack = Vec::with_capacity(m);
    let mut sum_external = 0;

    stack.push((0, m));

    while let Some((node, parent)) = stack.pop() {
        order.push((node, parent));

        for &next in graph[node].iter() {
            if next != parent {
                stack.push((next, node));
            }
        }
    }

    for &(node, parent) in order.iter().rev() {
        size[node] = villages[node];

        for &next in graph[node].iter() {
            if next != parent {
                size[node] += size[next];
            }
        }

        if parent < m {
            let weight = if size[node] < n - size[node] {
                size[node]
            } else {
                n - size[node]
            };
            sum_external += weight * (n - weight);
        }
    }

    writeln!(out, "{}", sum_internal + sum_external).unwrap();
}
